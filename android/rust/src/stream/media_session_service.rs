use crate::*;
use crate::stream::native_rust_stream_plugin::RustNativeStreamPlugin;
use crate::stream::stream_client::{get_stream_client, PlayState};

pub struct MediaSessionService {}

impl MediaSessionService {
    pub fn new(env: &mut JNIEnv, title: &str) -> Result<()> {
        env.call_method(get_ctx(), "new", "(Ljava/lang/String;)V", &[
            (&env.new_string(title)?).into(),
        ])?;
        Ok(())
    }

    pub fn play(env: &mut JNIEnv) -> Result<()> {
        env.call_method(get_ctx(), "play", "()V", &[])?;
        Ok(())
    }

    pub fn pause(env: &mut JNIEnv) -> Result<()> {
        env.call_method(get_ctx(), "pause", "()V", &[])?;
        Ok(())
    }

    pub fn stop(env: &mut JNIEnv) -> Result<()> {
        env.call_method(get_ctx(), "stop", "()V", &[])?;
        Ok(())
    }

    pub fn set_duration(env: &mut JNIEnv, duration: i32) -> Result<()> {
        env.call_method(get_ctx(), "setDuration", "(I)V", &[
            duration.into(),
        ])?;
        Ok(())
    }

    pub fn set_position(env: &mut JNIEnv, position: i32) -> Result<()> {
        env.call_method(get_ctx(), "setPosition", "(I)V", &[
            position.into(),
        ])?;
        Ok(())
    }
}


static CTX: OnceCell<GlobalRef> = OnceCell::new();

fn get_ctx() -> &'static GlobalRef { CTX.get().unwrap() }

android_fn!(com_fujipod, fujipod, mediaSession_MediaSessionService, create, [JObject]);
#[allow(non_snake_case)]
pub unsafe fn create(env: JNIEnv, _: JClass, context: JObject) {
    let context = env.new_global_ref(context).unwrap();
    CTX.get_or_init(move || { context });
}

android_fn!(com_fujipod, fujipod, mediaSession_MediaSessionService, nativeOnPlay, []);
#[allow(non_snake_case)]
pub unsafe fn nativeOnPlay(_: JNIEnv, _: JClass) {
    let _ = get_stream_client().play(&None, None, None);
}

android_fn!(com_fujipod, fujipod, mediaSession_MediaSessionService, nativeOnPause, []);
#[allow(non_snake_case)]
pub unsafe fn nativeOnPause(_: JNIEnv, _: JClass) {
    let _ = get_stream_client().pause();
}

android_fn!(com_fujipod, fujipod, mediaSession_MediaSessionService, nativeOnStop, []);
#[allow(non_snake_case)]
pub unsafe fn nativeOnStop(mut env: JNIEnv, _: JClass) {
    let _ = get_stream_client().stop();
    let _ = RustNativeStreamPlugin::on_request_close(&mut env);
}


android_fn!(com_fujipod, fujipod, mediaSession_MediaSessionService, nativeOnNext, []);
#[allow(non_snake_case)]
pub unsafe fn nativeOnNext(_: JNIEnv, _: JClass) {
    let _ = get_stream_client().next();
}

android_fn!(com_fujipod, fujipod, mediaSession_MediaSessionService, nativeOnPrevious, []);
#[allow(non_snake_case)]
pub unsafe fn nativeOnPrevious(_: JNIEnv, _: JClass) {
    let _ = get_stream_client().previous();
}

android_fn!(com_fujipod, fujipod, mediaSession_MediaSessionService, nativeOnSeek, [i32]);
#[allow(non_snake_case)]
pub unsafe fn nativeOnSeek(_: JNIEnv, _: JClass, position: i32) {
    let _ = get_stream_client().seek_absolute(position as u128);
}