use std::ops::{Deref, DerefMut};

use futures_util::{SinkExt, StreamExt, TryStreamExt};
use futures_util::stream::SplitSink;
use once_cell::sync::Lazy;
use rust_lapper::Lapper;
use serde_json::json;
use tokio::net::TcpStream;
use tokio::sync::mpsc::*;
use tokio::sync::oneshot;
use tokio_stream::wrappers::UnboundedReceiverStream;
use tokio_tungstenite::{MaybeTlsStream, WebSocketStream};
use tungstenite::client::IntoClientRequest;

use crate::*;
use crate::bridge::NativeRun;
use crate::clamp::{clamp, map_range_clamp};
use crate::option_interval::OptionInterval;
use crate::option_sleep::{option_sleep, OptionSleep};
use crate::stream::media_session_service::MediaSessionService;
use crate::stream::message_types;
use crate::stream::native_rust_audio_service::{AudioHandle, RustAudioServiceJava};
use crate::stream::native_rust_stream_plugin::RustNativeStreamPlugin;

type WsSender = SplitSink<WebSocketStream<MaybeTlsStream<TcpStream>>, tungstenite::Message>;

static UPDATE_TIMER_INTERVAL_DURATION: Duration = Duration::from_millis(100);

enum QueueItem {
    Recording {
        id: i32,
        target_time: Duration,
        duration: Duration,
        handle: Mutex<AudioHandle>,
    },
    Message {
        id: i32,
        target_time: Duration,
        msg: serde_json::Value,
    },
    UIEvent {
        id: i32,
        target_time: Duration,
        duration: Duration,
        msg: serde_json::Value,
    },
}

impl PartialEq<Self> for QueueItem {
    fn eq(&self, other: &Self) -> bool {
        self.get_id() == other.get_id()
    }
}

impl Eq for QueueItem {}

impl QueueItem {
    pub fn get_id(&self) -> &i32 {
        match self {
            QueueItem::Recording { id, .. } => id,
            QueueItem::Message { id, .. } => id,
            QueueItem::UIEvent { id, .. } => id,
        }
    }
    pub fn get_target_time(&self) -> Duration {
        match self {
            QueueItem::Recording { target_time, .. } => *target_time,
            QueueItem::Message { target_time, .. } => *target_time,
            QueueItem::UIEvent { target_time, .. } => *target_time,
        }
    }
    pub fn get_duration(&self) -> Duration {
        match self {
            QueueItem::Recording { duration, .. } => *duration,
            QueueItem::UIEvent { duration, .. } => *duration,
            QueueItem::Message { .. } => Duration::from_millis(1000)
        }
    }
}

#[derive(Debug, PartialEq)]
enum StreamMessage {
    Resume,
    Pause,
    Stop,
    AudioEnded { id: i32 },
    Seek { position: f32 },
    SeekAbsolute { position: u128 },
    Next,
    Previous,
    SwitchStream { name: String, parameters: serde_json::Value },
}

struct StreamClientState {
    ws_handle: JoinHandle<Result<()>>,
    msg_tx: UnboundedSender<StreamMessage>,
    stop_rx: oneshot::Receiver<()>,
}

pub struct StreamClient {
    state: Option<StreamClientState>,
}

#[derive(Debug, Copy, Clone, Eq, PartialEq)]
pub enum PlayState {
    Stopped,
    Playing,
    Paused,
    Seeking,
}

struct State<'a> {
    name: String,
    url: String,
    timeline: Lapper<u128, Arc<QueueItem>>,
    playing_items: HashMap<i32, Arc<QueueItem>>,
    next_schedule: OptionSleep,
    env: JNIEnv<'a>,
    reference_time: Option<Instant>,
    progress: Duration,
    total_duration: Duration,
    update_progress_timer: OptionInterval,
    play_state: PlayState,
    // the target time to restore to after unpausing
    unpause_target_time: Option<Duration>,
    iteration_token: u32,
    lag: i64,
    msg_tx: UnboundedSender<StreamMessage>,
    fetching_content: bool,
    at_end: bool,
    ws_tx: UnboundedSender<message_types::ClientMessage>,
}

impl StreamClient {
    pub fn new() -> Self {
        Self { state: None }
    }

    fn get_current_time_without_lag(state: &State) -> Duration {
        match state.reference_time {
            Some(reference_time) => {
                Instant::now() - reference_time
            }
            None => { Duration::default() }
        }
    }

    fn get_current_time(state: &State) -> Duration {
        Duration::from_millis((Self::get_current_time_without_lag(&state).as_millis() as i64 - state.lag) as u64)
    }

    fn sync_time(state: &mut State, position: Duration) {
        state.lag = Self::get_current_time_without_lag(state).as_millis() as i64 - position.as_millis() as i64;
    }

    fn set_progress(state: &mut State, progress: Duration) -> Result<()> {
        state.progress = progress;
        let p = progress.as_millis() as i32;
        RustNativeStreamPlugin::on_progress_update(&mut state.env, p)?;
        MediaSessionService::set_position(&mut state.env, p)?;
        Ok(())
    }

    fn set_play_state(state: &mut State, play_state: PlayState) -> Result<()> {
        if state.play_state == play_state { return Ok(()); }

        state.play_state = play_state;
        RustNativeStreamPlugin::on_play_state_change(&mut state.env, state.play_state)?;

        match play_state {
            PlayState::Stopped => {
                MediaSessionService::stop(&mut state.env)?;
            }
            PlayState::Seeking |
            PlayState::Playing => {
                MediaSessionService::play(&mut state.env)?;
            }
            PlayState::Paused => {
                MediaSessionService::pause(&mut state.env)?;
            }
        }
        Ok(())
    }

    fn update_progress(state: &mut State) -> Result<()> {
        let p = clamp(Self::get_current_time(state), Duration::default(), state.total_duration);
        Self::set_progress(state, p)?;
        Ok(())
    }

    fn set_total_duration(state: &mut State, duration: Duration) -> Result<()> {
        state.total_duration = duration;
        let p = duration.as_millis() as i32;
        RustNativeStreamPlugin::on_total_duration_change(&mut state.env, p)?;
        MediaSessionService::set_duration(&mut state.env, p)?;
        Ok(())
    }

    fn set_update_progress_timer_enabled(state: &mut State, enabled: bool) -> Result<()> {
        Self::update_progress(state)?;
        state.update_progress_timer = if enabled {
            UPDATE_TIMER_INTERVAL_DURATION.into()
        } else {
            None.into()
        };

        Ok(())
    }

    fn _seek_absolute(state: &mut State, position: Duration) -> Result<()> {
        Self::clear_items(state)?;
        Self::sync_time(state, position);
        state.unpause_target_time = Some(Self::get_current_time(state));
        Self::update_progress(state)?;
        state.next_schedule = option_sleep(None);

        if state.play_state != PlayState::Paused {
            Self::schedule(state)?;
        }

        Self::request_content(state)?;
        Ok(())
    }

    fn clear_items(state: &mut State) -> Result<()> {
        Self::pause_items(state)?;
        state.playing_items.clear();
        Ok(())
    }

    fn pause_items(state: &mut State) -> Result<()> {
        for item in state.playing_items.values() {
            match item.as_ref() {
                QueueItem::Recording { handle, id, .. } => {
                    RustAudioServiceJava::audio_pause(&mut state.env, &handle.lock().unwrap())?;
                }
                _ => {}
            };
        }
        Ok(())
    }

    fn on_timeline_change(state: &mut State) -> Result<()> {
        let mut v = vec![];
        for item in state.timeline.iter() {
            match item.val.as_ref() {
                QueueItem::UIEvent { msg, target_time, duration, .. } => {
                    // ignore non "feed" events
                    if msg.get("type") != Some(&json!("feed")) { continue; }

                    v.push((
                        msg.get("value").unwrap(),
                        map_range_clamp(target_time.as_millis(), 0, state.total_duration.as_millis(), 0, 100),
                        map_range_clamp(duration.as_millis(), 0, state.total_duration.as_millis(), 0, 100),
                    ));
                }
                _ => {}
            }
        }

        RustNativeStreamPlugin::on_ui_feed_message_timeline_change(&mut state.env, &serde_json::to_string(&v)?)?;

        Ok(())
    }

    fn ensure_state(&mut self) {
        if self.state.as_mut().and_then(|mut x| x.stop_rx.try_recv().ok()).is_some() {
            self.state = None;
        }
    }

    fn start_stream(&mut self, name: &str, parameters: serde_json::Value, cookies: Option<String>) -> Result<()> {
        let name = name.to_string();
        let (stop_tx, stop_rx) = oneshot::channel();

        let proto = if config::public_proto() == "http" { "ws" } else { "wss" };
        let url = format!("{}://{}:{}/websocket", proto, config::public_host(), config::public_port());
        info!("requesting stream from '{}'", url);

        let (mut msg_tx, mut msg_rx) = unbounded_channel();
        let _msg_tx = msg_tx.clone();

        let ws_handle = thread::spawn(move || {
            let rt = tokio::runtime::Builder::new_current_thread()
                .enable_all()
                .build()?;

            let res: Result<(), anyhow::Error> = rt.block_on(async move {
                let run = NativeRun::new();

                let (mut ws_tx, _ws_rx) = unbounded_channel();

                let mut state = State {
                    name,
                    url: url.to_string(),
                    timeline: rust_lapper::Lapper::new(vec![]),
                    playing_items: HashMap::new(),
                    next_schedule: option_sleep(None),
                    env: run.get_env(),
                    reference_time: None,
                    progress: Duration::default(),
                    total_duration: Duration::default(),
                    lag: 0,
                    update_progress_timer: UPDATE_TIMER_INTERVAL_DURATION.into(),
                    msg_tx,
                    unpause_target_time: None,
                    iteration_token: 0,
                    play_state: PlayState::Stopped,
                    fetching_content: false,
                    at_end: false,
                    ws_tx,
                };

                Self::set_play_state(&mut state, PlayState::Seeking)?;
                MediaSessionService::new(&mut state.env, &state.name)?;
                RustNativeStreamPlugin::on_stream_change(&mut state.env, &state.name)?;

                let mut request = url.into_client_request()?;
                if let Some(cookies) = cookies {
                    request.headers_mut().insert("Cookie", cookies.parse().unwrap());
                }

                let (ws_stream, _) = tokio_tungstenite::connect_async(request).await?;
                let (mut _ws_tx, mut ws_rx) = ws_stream.split();

                let mut ws_tx_forward = UnboundedReceiverStream::new(_ws_rx)
                    .map(|x| Ok(tungstenite::Message::Text(serde_json::to_string(&x).unwrap())))
                    .forward(_ws_tx);

                let msg = message_types::ClientMessage::play {
                    streamName: state.name.to_string(),
                    parameters,
                    iterationToken: 0,
                };
                let _ = state.ws_tx.send(msg);

                loop {
                    let ret = tokio::select! {
                        // do some work on the ws send
                        _ = &mut ws_tx_forward => { Ok(()) }
                        Ok(Some(msg)) = ws_rx.try_next() => {
                            Self::handle_ws_msg(&mut state, msg)
                        }
                        Some(msg) = msg_rx.recv() => {
                            let stop = msg == StreamMessage::Stop;
                            let ret = Self::handle_msg(&mut state, msg);
                            if stop { break; }
                            ret
                        }
                        _ = state.update_progress_timer.tick() => {
                            Self::request_content(&mut state)?;
                            Self::update_progress(&mut state)
                        }
                        _ = state.next_schedule.tick() => {
                            Self::schedule(&mut state)
                        }
                    };
                    ret?;
                }
                Ok(())
            });

            if let Err(err) = res {
                log!(Level::Error, "stream event loop finished with err: {:?}", err);
            }

            log!(Level::Info, "stream loop finished");

            let run = NativeRun::new();
            RustNativeStreamPlugin::on_request_close(&mut run.get_env())?;

            let _ = stop_tx.send(());
            Ok(())
        });

        self.state = Some(StreamClientState {
            ws_handle,
            msg_tx: _msg_tx,
            stop_rx,
        });

        Ok(())
    }

    fn request_content(state: &mut State) -> Result<()> {
        if state.fetching_content || state.at_end { return Ok(()); }

        let fetch_point = state.total_duration.as_millis() as i128 - 5000;
        if fetch_point < 0 || Self::get_current_time(state).as_millis() < fetch_point as u128 { return Ok(()); }
        state.fetching_content = true;
        log!(Level::Info, "request content");

        let msg = message_types::ClientMessage::requestContent {
            iterationToken: state.iteration_token,
        };
        let _ = state.ws_tx.send(msg);

        Ok(())
    }

    fn handle_ws_msg(state: &mut State, msg: tungstenite::Message) -> Result<()> {
        let msg = match msg {
            tungstenite::Message::Text(msg) => { msg }
            _ => { return Ok(()); }
        };
        let ret: message_types::ServerMessage = serde_json::from_str(&msg)?;
        log!(Level::Info, "got msg: {}", msg);

        match ret {
            message_types::ServerMessage::queueTrackItems { trackItems, iterationToken, atEnd } => {
                log!(Level::Info, "queue: {}" , trackItems.len() );
                // ignore outdated messages
                if iterationToken as u32 != state.iteration_token { return Ok(()); };
                state.fetching_content = false;
                state.at_end = atEnd;
                if trackItems.is_empty() { return Ok(()); }

                // this.atEnd = msg.atEnd;
                let mut total_duration = Duration::default();

                for track_item in trackItems {
                    match track_item {
                        message_types::QueuedTrackItem::recording { id, value, targetTime } => {
                            state.timeline.insert(rust_lapper::Interval {
                                start: targetTime as u128,
                                stop: (targetTime + value.duration) as u128,
                                val: Arc::new(QueueItem::Recording {
                                    id,
                                    target_time: Duration::from_millis(targetTime as u64),
                                    duration: Duration::from_millis(value.duration as u64),
                                    handle: Mutex::new(RustAudioServiceJava::audio_new(&mut state.env, &value.url)?),
                                }),
                            });

                            total_duration = total_duration.max(Duration::from_millis((targetTime + value.duration) as u64));
                        }
                        message_types::QueuedTrackItem::message { id, value, targetTime } => {
                            log!(Level::Info, "ev {} is server msg" , id );
                            let stop_time = targetTime + 1000.0;
                            state.timeline.insert(rust_lapper::Interval {
                                start: targetTime as u128,
                                stop: stop_time as u128,
                                val: Arc::new(QueueItem::Message {
                                    id,
                                    target_time: Duration::from_millis(targetTime as u64),
                                    msg: serde_json::to_value(&value).unwrap(),
                                }),
                            });
                        }
                        message_types::QueuedTrackItem::ui { id, value, targetTime } => {
                            let duration = match value {
                                message_types::UIUpdateMessage::feed { duration, .. } => duration
                            };
                            state.timeline.insert(rust_lapper::Interval {
                                start: targetTime as u128,
                                stop: (targetTime + duration) as u128,
                                val: Arc::new(QueueItem::UIEvent {
                                    id,
                                    target_time: Duration::from_millis(targetTime as u64),
                                    duration: Duration::from_millis(duration as u64),
                                    msg: serde_json::to_value(&value).unwrap(),
                                }),
                            });

                            total_duration = total_duration.max(Duration::from_millis((targetTime + duration) as u64));
                        }
                    }
                }

                log!(Level::Info, "new total duration: {}" , total_duration.as_millis());

                Self::set_total_duration(state, total_duration)?;
                Self::on_timeline_change(state)?;

                if state.play_state == PlayState::Seeking {
                    Self::schedule(state)?;
                }
            }
        }

        Ok(())
    }

    fn handle_msg(state: &mut State, msg: StreamMessage) -> Result<()> {
        log!(Level::Info, "handling client msg");

        match msg {
            StreamMessage::SwitchStream { name, parameters } => {
                state.iteration_token += 1;
                Self::set_play_state(state, PlayState::Seeking)?;
                Self::set_total_duration(state, Duration::default())?;
                Self::set_update_progress_timer_enabled(state, false)?;
                Self::clear_items(state)?;
                state.timeline = Lapper::new(vec![]);
                state.at_end = false;
                state.reference_time = None;
                state.unpause_target_time = None;
                state.next_schedule = option_sleep(None);
                state.lag = 0;
                state.progress = Duration::default();
                state.fetching_content = false;
                state.name = name.clone();

                let msg = message_types::ClientMessage::play {
                    streamName: name,
                    parameters,
                    iterationToken: state.iteration_token,
                };
                let _ = state.ws_tx.send(msg);
            }
            StreamMessage::Resume => {
                if let Some(unpause_target_time) = state.unpause_target_time {
                    Self::sync_time(state, unpause_target_time);
                }
                Self::set_update_progress_timer_enabled(state, true)?;
                Self::set_play_state(state, PlayState::Playing)?;

                for item in state.playing_items.values() {
                    match item.as_ref() {
                        QueueItem::Recording { handle, .. } => {
                            RustAudioServiceJava::audio_play(&mut state.env, &handle.lock().unwrap())?;
                        }
                        QueueItem::UIEvent { .. } => {}
                        _ => {}
                    };
                }

                Self::schedule(state)?;
            }
            StreamMessage::Pause => {
                state.unpause_target_time = Some(Self::get_current_time(state));
                Self::pause_items(state)?;
                Self::set_play_state(state, PlayState::Paused)?;
                Self::set_update_progress_timer_enabled(state, false)?;
                state.next_schedule = option_sleep(None);
            }
            StreamMessage::Stop => {
                Self::clear_items(state)?;
                Self::set_update_progress_timer_enabled(state, false)?;
                Self::set_play_state(state, PlayState::Stopped)?;
            }
            StreamMessage::AudioEnded { id } => {
                log!(Level::Info, "ended {}, ps: {:?}", id,state.play_state);
                if state.play_state != PlayState::Playing { return Ok(()); }

                if let Some(item) = state.playing_items.remove(&id) {
                    match item.as_ref() {
                        QueueItem::Recording { duration, id, handle, .. } => {
                            let mut handle = handle.lock().unwrap();
                            Self::sync_time(state, item.get_target_time() + *duration);
                            RustAudioServiceJava::audio_destroy(&mut state.env, handle.deref_mut())?;
                        }
                        _ => {}
                    }
                } else {
                    log!(Level::Info, "no audio found");
                    // was not in the plying items list, do nothing
                    return Ok(());
                }

                Self::update_progress(state)?;
                Self::schedule(state)?;
            }
            StreamMessage::Seek { position } => {
                let millis = (state.total_duration.as_millis() as f32 / 100.0 * position).round() as u64;
                let target_time = Duration::from_millis(millis);
                Self::_seek_absolute(state, target_time)?;
            }
            StreamMessage::SeekAbsolute { position } => {
                Self::_seek_absolute(state, Duration::from_millis(position as u64))?;
            }
            StreamMessage::Next => {
                let now = Self::get_current_time(state);
                let items = state.timeline.find(now.as_millis() as u128, u128::MAX).collect::<Vec<_>>();
                for item in items.iter() {
                    if now < item.val.get_target_time() {
                        Self::_seek_absolute(state, item.val.get_target_time())?;
                        return Ok(());
                    }
                }
            }
            StreamMessage::Previous => {
                let now = Self::get_current_time(state);
                let items = state.timeline.find(0, now.as_millis() + 1).collect::<Vec<_>>();
                for item in items.iter().rev() {
                    if now > item.val.get_target_time() + item.val.get_duration() {
                        Self::_seek_absolute(state, item.val.get_target_time())?;
                        return Ok(());
                    }
                }
            }
        }
        Ok(())
    }

    fn find_next_poi(state: &mut State) -> Option<u128> {
        let now = Self::get_current_time(state).as_millis();
        let mut should_ignore = false;
        let mut poi = None;

        let mut items = state.timeline
            .find(now, u128::MAX)
            .map(|x| x.val.clone())
            .collect::<Vec<_>>();
        items.extend(state.playing_items.values().cloned());

        for item in items {
            let start = item.get_target_time().as_millis();
            let end = (item.get_target_time() + item.get_duration()).as_millis();

            // if in the future
            if start >= now {
                if poi.is_some() && poi.unwrap() <= start { continue; }
                should_ignore = false;
                poi = Some(start);
                continue;
            }
            // it's ongoing
            if poi.is_some() && poi.unwrap() < end { continue; }
            if poi.is_some() && poi.unwrap() == end {
                // if there's a recording and a non-recording option, prioritize the recording
                should_ignore = should_ignore || match item.as_ref() {
                    QueueItem::Recording { .. } => true,
                    _ => false,
                }
            } else {
                should_ignore = match item.as_ref() {
                    QueueItem::Recording { .. } => true,
                    _ => false,
                }
            }
            poi = Some(end);
        }

        if should_ignore { return None; }
        poi
    }

    fn schedule(state: &mut State) -> Result<()> {
        if (state.play_state == PlayState::Stopped) { return Ok(()); }
        state.reference_time.get_or_insert_with(|| Instant::now());

        let now = Self::get_current_time(state);

        // throw away finished items
        state.playing_items.retain(|_, x| x.get_target_time() + x.get_duration() > now);

        let playable_items = state.timeline.find(now.as_millis(), now.as_millis() + 1)
            .map(|x| x.val.clone())
            .filter(|x| !state.playing_items.contains_key(x.as_ref().get_id()))
            .collect::<Vec<_>>();

        log!(Level::Info, "schedule, items: {}", playable_items.len());

        if !playable_items.is_empty() {
            // start playing items
            Self::set_play_state(state, PlayState::Playing)?;
            Self::set_update_progress_timer_enabled(state, true)?;

            for item in playable_items.into_iter() {
                log!(Level::Info, "play {}", item.get_id());

                match item.as_ref() {
                    QueueItem::Recording { id, handle, target_time, .. } => {
                        let id = *id;
                        state.playing_items.insert(id, item.clone());
                        let mut handle = handle.lock().unwrap();

                        let position = u128::max(0, (now - *target_time).as_millis());
                        RustAudioServiceJava::audio_set_current_time(&mut state.env, handle.deref(), position)?;
                        let msg_tx = state.msg_tx.clone();
                        handle.deref_mut().set_on_ended(Some(Box::new(move || {
                            let _ = msg_tx.send(StreamMessage::AudioEnded { id });
                        })));
                        RustAudioServiceJava::audio_play(&mut state.env, handle.deref())?;
                    }
                    QueueItem::Message { msg, .. } => {
                        state.ws_tx.send(message_types::ClientMessage::message {
                            message: msg.clone(),
                        }).unwrap();
                        let msg = serde_json::to_string(&msg).unwrap();
                        log!(Level::Info, "sending message to server {}", msg);
                    }
                    QueueItem::UIEvent { id, target_time, duration, msg, .. } => {
                        state.playing_items.insert(*id, item.clone());
                        let msg = serde_json::to_string(&msg).unwrap();
                        RustNativeStreamPlugin::on_ui_event(&mut state.env, &msg)?;
                    }
                }
            }
        }

        match Self::find_next_poi(state) {
            Some(poi) => {
                log!(Level::Info, "found poi {} | {}", poi, now.as_millis());
                let next_schedule_time = poi - now.as_millis();
                log!(Level::Info, "next schedule at {}", next_schedule_time);
                state.next_schedule = option_sleep(Some(Duration::from_millis(next_schedule_time as u64)));
                Self::set_play_state(state, PlayState::Playing)?;
                Self::set_update_progress_timer_enabled(state, true)?;
                log!(Level::Info, "ok");
            }
            None => {
                log!(Level::Info, "not found poi");
                if !state.playing_items.is_empty() { return Ok(()); }
                if state.at_end {
                    log!(Level::Info, "b1");
                    Self::set_play_state(state, PlayState::Stopped)?;
                    Self::set_update_progress_timer_enabled(state, false)?;
                } else {
                    log!(Level::Info, "b2");
                    Self::set_play_state(state, PlayState::Seeking)?;
                    Self::set_update_progress_timer_enabled(state, false)?;
                    Self::request_content(state)?;
                }
            }
        }

        Ok(())
    }

    fn send_client_message(&mut self, msg: StreamMessage) {
        match &mut self.state {
            Some(StreamClientState { msg_tx, .. }) => {
                let _ = msg_tx.send(msg);
            }
            _ => {}
        }
    }

    pub fn play(&mut self, name: &Option<String>, parameters: Option<serde_json::Value>, cookies: Option<String>) -> Result<()> {
        self.ensure_state();
        match name {
            Some(name) => {
                let parameters = parameters.unwrap_or_else(|| serde_json::Value::default());
                if let Some(state) = self.state.as_mut() {
                    let _ =state.msg_tx.send(StreamMessage::SwitchStream {
                        name: name.clone(),
                        parameters,
                    });
                } else {
                    self.start_stream(name, parameters, cookies)?;
                }
            }
            None => {
                self.send_client_message(StreamMessage::Resume);
            }
        }
        Ok(())
    }

    pub fn pause(&mut self) {
        self.ensure_state();
        self.send_client_message(StreamMessage::Pause);
    }

    pub fn stop(&mut self) {
        self.ensure_state();
        self.send_client_message(StreamMessage::Stop);
    }

    pub fn seek(&mut self, position: f32) {
        self.ensure_state();
        self.send_client_message(StreamMessage::Seek { position });
    }

    pub fn seek_absolute(&mut self, position: u128) {
        self.ensure_state();
        self.send_client_message(StreamMessage::SeekAbsolute { position });
    }

    pub fn next(&mut self) {
        self.ensure_state();
        self.send_client_message(StreamMessage::Next);
    }

    pub fn previous(&mut self) {
        self.ensure_state();
        self.send_client_message(StreamMessage::Previous);
    }
}


pub static STREAM_CLIENT: Lazy<Mutex<StreamClient>> = Lazy::new(|| Mutex::new(StreamClient::new()));

pub fn get_stream_client<'a>() -> MutexGuard<'a, StreamClient> {
    STREAM_CLIENT.lock().unwrap()
}