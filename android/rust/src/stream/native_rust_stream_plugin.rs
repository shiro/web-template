use futures_util::TryFutureExt;
use crate::*;
use crate::stream::stream_client::{get_stream_client, PlayState};

pub struct RustNativeStreamPlugin {}

impl RustNativeStreamPlugin {
    pub fn on_progress_update(env: &mut JNIEnv, progress: i32) -> Result<()> {
        env.call_method(get_ctx().deref_mut(), "onProgressUpdate", "(I)V", &[
            progress.into(),
        ])?;
        Ok(())
    }

    pub fn on_total_duration_change(env: &mut JNIEnv, duration: i32) -> Result<()> {
        env.call_method(get_ctx().deref_mut(), "onTotalDurationChange", "(I)V", &[
            duration.into(),
        ])?;
        Ok(())
    }

    pub fn on_play_state_change(env: &mut JNIEnv, state: PlayState) -> Result<()> {
        env.call_method(get_ctx().deref_mut(), "onPlayStateChange", "(I)V", &[
            (state as i32).into(),
        ])?;
        Ok(())
    }

    pub fn on_stream_change(env: &mut JNIEnv, name: &str) -> Result<()> {
        env.call_method(get_ctx().deref_mut(), "onStreamChange", "(Ljava/lang/String;)V", &[
            (&env.new_string(name)?).into(),
        ])?;
        Ok(())
    }

    pub fn on_ui_event(env: &mut JNIEnv, event: &str) -> Result<()> {
        env.call_method(get_ctx().deref_mut(), "onUIEvent", "(Ljava/lang/String;)V", &[
            (&env.new_string(event)?).into(),
        ])?;
        Ok(())
    }

    pub fn on_ui_feed_message_timeline_change(env: &mut JNIEnv, event: &str) -> Result<()> {
        env.call_method(get_ctx().deref_mut(), "onUIFeedMessageTimelineChange", "(Ljava/lang/String;)V", &[
            (&env.new_string(event)?).into(),
        ])?;
        Ok(())
    }

    pub fn on_request_close(env: &mut JNIEnv) -> Result<()> {
        env.call_method(get_ctx().deref_mut(), "onRequestClose", "()V", &[])?;
        Ok(())
    }
}


static CTX: OnceCell<Mutex<GlobalRef>> = OnceCell::new();

fn get_ctx<'a>() -> MutexGuard<'a, GlobalRef> { CTX.get().unwrap().lock().unwrap() }

android_fn!(com_fujipod, fujipod, plugins_StreamCapacitorPlugin, create, [JObject]);
#[allow(non_snake_case)]
pub unsafe fn create(env: JNIEnv, _: JClass, context: JObject) {
    let context = env.new_global_ref(context).unwrap();
    match CTX.get() {
        Some(ctx) => {
            *ctx.lock().unwrap() = context;
        }
        None => {
            CTX.get_or_init(move || { Mutex::new(context) });
        }
    };
}

android_fn!(com_fujipod, fujipod, plugins_StreamCapacitorPlugin, play, [JObject, JObject, JObject]);
pub unsafe fn play(mut env: JNIEnv, _: JClass, stream_name: JObject, parameters: JObject, cookies: JObject) {
    let stream_name = env.get_string(&stream_name.into())
        .ok()
        .and_then(|x| x.to_str().ok().map(str::to_string));

    let parameters = env.get_string(&parameters.into())
        .ok()
        .and_then(|x| x.to_str().ok().and_then(|x| serde_json::from_str(x).ok()));

    let cookies = env.get_string(&cookies.into())
        .ok()
        .and_then(|x| x.to_str().ok().map(str::to_string));

    let _ = get_stream_client().play(&stream_name, parameters, cookies);
}

android_fn!(com_fujipod, fujipod, plugins_StreamCapacitorPlugin, pause, []);
pub unsafe fn pause(_: JNIEnv, _: JClass) {
    get_stream_client().pause();
}

android_fn!(com_fujipod, fujipod, plugins_StreamCapacitorPlugin, stop, []);
pub unsafe fn stop(_: JNIEnv, _: JClass) {
    get_stream_client().stop();
}

android_fn!(com_fujipod, fujipod, plugins_StreamCapacitorPlugin, seek, [f32]);
pub unsafe fn seek(_: JNIEnv, _: JClass, position: f32) {
    get_stream_client().seek(position);
}