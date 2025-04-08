use std::collections::HashMap;

use jni::sys::jstring;
use serde::{Deserialize, Serialize};

use crate::*;
// use crate::messaging::message_types::NativeBridgeBackendMessage;

// #[derive(PartialEq, Serialize, Deserialize)]
// #[serde(tag = "action")]
// enum ClientMessage {
//     requestContent { iterationToken: i32 },
//     play {
//         streamName: String,
//         parameters: HashMap<String, String>,
//         iterationToken: i32,
//     },
// }
//
//
// #[derive(PartialEq, Serialize, Deserialize)]
// struct QueuedRecording {
//     duration: i32,
//     url: String,
// }
//
// #[derive(PartialEq, Serialize, Deserialize)]
// #[serde(tag = "type")]
// enum QueuedTrackItem {
//     recording {
//         id: i32,
//         value: QueuedRecording,
//         targetTime: f32,
//     },
//     message {
//         id: i32,
//         // value: String,
//         targetTime: f32,
//     },
//     ui {
//         id: i32,
//         // value: String,
//         targetTime: f32,
//     },
// }
//
// #[derive(PartialEq, Serialize, Deserialize)]
// #[serde(tag = "action")]
// enum ServerMessage {
//     queueTrackItems {
//         trackItems: Vec<QueuedTrackItem>,
//         iterationToken: i32,
//         atEnd: bool,
//     },
// }
//
// #[derive(Debug)]
// struct LoopQueuedRecording {
//     id: i32,
//     targetTime: f32,
//     duration: i32,
//     url: String,
// }
//
// // { action: "play"; streamName: string; parameters?: Record<string, any>; iterationToken: number; } |
// // { action: "pause"; } |
// // { action: "message"; message: StreamMessage; } |
// // { action: "syncClock"; relativeDistance: number; } |
// // { action: "stop"; }
//
// #[derive(Debug)]
// enum LoopCtxMessage {
//     TrackFinished,
//     Stop,
// }
//
// struct LoopCtx {
//     ws_handle: JoinHandle<Result<()>>,
//     msg_tx: tokio::sync::mpsc::UnboundedSender<LoopCtxMessage>,
// }
//
// pub struct StreamService {
//     event_loop_proxy: EventLoopProxy<BackendEvent>,
//     loop_ctx: Option<LoopCtx>,
//     // audio_map: HashMap<i32, NativeAudio>,
// }
//
// impl StreamService {
//     pub fn new(event_loop_proxy: EventLoopProxy<BackendEvent>) -> Self {
//         Self {
//             event_loop_proxy,
//             loop_ctx: Default::default(),
//             // audio_map: Default::default(),
//         }
//     }
//
//     pub fn acquire_wakelock(&self) {
//         self.event_loop_proxy.send_event(BackendEvent::NativeCallback(Box::new(
//             move |env: JNIEnv, activity: JObject, w: JObject| -> Result<()> {
//                 let service = env.get_field(activity, "streamService", "Lcom/fujipod/fujipod/audio/StreamService;")?.l()?;
//                 env.call_method(service, "acquireWakeLock", "()V", &[])?;
//                 Ok(())
//             }
//         ))).unwrap()
//     }
//
//     pub fn release_wakelock(&self) {
//         self.event_loop_proxy.send_event(BackendEvent::NativeCallback(Box::new(
//             move |env: JNIEnv, activity: JObject, w: JObject| -> Result<()> {
//                 let service = env.get_field(activity, "streamService", "Lcom/fujipod/fujipod/audio/StreamService;")?.l()?;
//                 env.call_method(service, "releaseWakeLock", "()V", &[])?;
//                 Ok(())
//             }
//         ))).unwrap()
//     }
//
//     pub fn audio_new(&self, id: i32, src: String) {
//         self.event_loop_proxy.send_event(BackendEvent::NativeCallback(Box::new(
//             move |env: JNIEnv, activity: JObject, w: JObject| -> Result<()> {
//                 let service = env.get_field(activity, "streamService", "Lcom/fujipod/fujipod/audio/StreamService;")?.l()?;
//                 if src.starts_with("http") {
//                     env.call_method(service, "audioNew", "(ILjava/lang/String;)V", &[
//                         id.into(),
//                         env.new_string(&src)?.into(),
//                     ])?;
//                 } else {
//                     env.call_method(service, "audioNew", "(ILjava/lang/String;)V", &[
//                         id.into(),
//                         JObject::null().into(),
//                     ])?;
//                 }
//                 Ok(())
//             }
//         ))).unwrap()
//     }
//
//     pub fn audio_set_src(&self, id: i32, src: String) {
//         self.event_loop_proxy.send_event(BackendEvent::NativeCallback(Box::new(
//             move |env: JNIEnv, activity: JObject, w: JObject| -> Result<()> {
//                 let service = env.get_field(activity, "streamService", "Lcom/fujipod/fujipod/audio/StreamService;")?.l()?;
//                 env.call_method(service, "audioSetSrc", "(ILjava/lang/String;)V", &[
//                     id.into(),
//                     env.new_string(&src)?.into(),
//                 ])?;
//                 Ok(())
//             }
//         ))).unwrap()
//     }
//
//     pub fn audio_set_current_time(&self, id: i32, value: i32) {
//         self.event_loop_proxy.send_event(BackendEvent::NativeCallback(Box::new(
//             move |env: JNIEnv, activity: JObject, w: JObject| -> Result<()> {
//                 let service = env.get_field(activity, "streamService", "Lcom/fujipod/fujipod/audio/StreamService;")?.l()?;
//                 env.call_method(service, "audioSetCurrentTime", "(II)V", &[
//                     id.into(),
//                     value.into(),
//                 ])?;
//                 Ok(())
//             }
//         ))).unwrap()
//     }
//
//     pub fn audio_play(&self, id: i32) {
//         self.event_loop_proxy.send_event(BackendEvent::NativeCallback(Box::new(
//             move |env: JNIEnv, activity: JObject, w: JObject| -> Result<()> {
//                 let service = env.get_field(activity, "streamService", "Lcom/fujipod/fujipod/audio/StreamService;")?.l()?;
//                 env.call_method(service, "audioPlay", "(I)V", &[id.into()])?;
//                 Ok(())
//             }
//         ))).unwrap()
//     }
//
//     pub fn audio_pause(&self, id: i32) {
//         self.event_loop_proxy.send_event(BackendEvent::NativeCallback(Box::new(
//             move |env: JNIEnv, activity: JObject, w: JObject| -> Result<()> {
//                 let service = env.get_field(activity, "streamService", "Lcom/fujipod/fujipod/audio/StreamService;")?.l()?;
//                 env.call_method(service, "audioPause", "(I)V", &[id.into()])?;
//                 Ok(())
//             }
//         ))).unwrap()
//     }
//
//     pub fn audio_load(&self, id: i32) {
//         self.event_loop_proxy.send_event(BackendEvent::NativeCallback(Box::new(
//             move |env: JNIEnv, activity: JObject, w: JObject| -> Result<()> {
//                 let service = env.get_field(activity, "streamService", "Lcom/fujipod/fujipod/audio/StreamService;")?.l()?;
//                 env.call_method(service, "audioLoad", "(I)V", &[id.into()])?;
//                 Ok(())
//             }
//         ))).unwrap()
//     }
//
//     pub fn on_finished_playing(&self, id: i32) {
//         let msg = NativeBridgeBackendMessage::AudioOnEnded { objId: id };
//         self.event_loop_proxy.send_event(BackendEvent::Message(
//             serde_json::to_string(&msg).unwrap()
//         )).unwrap();
//     }
//
//     pub fn go_back(&self) {
//         self.event_loop_proxy.send_event(BackendEvent::NativeCallback(Box::new(
//             move |env: JNIEnv, activity: JObject, w: JObject| -> Result<()> {
//                 env.call_method(w, "goBack", "()V", &[])?;
//                 Ok(())
//             }
//         ))).unwrap()
//     }
// }
//
// pub static STREAM_SERVICE: OnceCell<Mutex<StreamService>> = OnceCell::new();
//
//
// #[inline(always)]
// pub fn register() {
//     android_fn!(com_fujipod, fujipod, audio_StreamService, onFinishPlayingListener, i32);
//     android_fn!(com_fujipod, fujipod, TauriActivity, getUrl, JObject, jstring);
// }
//
// pub unsafe fn onFinishPlayingListener(env: JNIEnv, this: JClass, id: i32) {
//     let service = STREAM_SERVICE.get().unwrap().lock().unwrap();
//     service.on_finished_playing(id);
// }
//
// pub unsafe fn getUrl(env: JNIEnv, this: JClass, _: JObject) -> jstring {
//     env.new_string(
//         &format!("http://{}:{}", dotenv!("PUBLIC_HOST"), dotenv!("PUBLIC_PORT"))
//     ).unwrap().into_raw()
// }
