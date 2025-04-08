use std::sync::atomic::{AtomicUsize, Ordering};

use crate::*;

static SEQ_ID: AtomicUsize = AtomicUsize::new(0);

type OnEndedCB = Box<dyn Fn() + Send>;

struct Listener {
    on_ended: Option<OnEndedCB>,
}

static LISTENERS: Lazy<Mutex<HashMap<usize, Listener>>> = Lazy::new(|| Mutex::new(HashMap::new()));


#[derive(Eq, PartialEq)]
pub struct AudioHandle {
    id: usize,
}

impl AudioHandle {
    pub(crate) fn set_on_ended(&mut self, on_ended: Option<OnEndedCB>) {
        LISTENERS.lock().unwrap().insert(self.id, Listener { on_ended });
    }
}

impl Drop for AudioHandle {
    fn drop(&mut self) {
        LISTENERS.lock().unwrap().remove(&self.id);
    }
}

pub struct RustAudioServiceJava {}

impl RustAudioServiceJava {
    pub fn audio_new(env: &mut JNIEnv, url: &str) -> Result<AudioHandle> {
        let handle = AudioHandle { id: SEQ_ID.fetch_add(1, Ordering::SeqCst) };
        env.call_method(get_ctx(), "audioNew", "(ILjava/lang/String;)V", &[
            (handle.id as i32).into(),
            (&env.new_string(url)?).into(),
        ])?;
        Ok(handle)
    }

    pub fn audio_play(env: &mut JNIEnv, handle: &AudioHandle) -> Result<()> {
        env.call_method(get_ctx(), "audioPlay", "(I)V", &[
            (handle.id as i32).into(),
        ])?;
        Ok(())
    }

    pub fn audio_pause(env: &mut JNIEnv, handle: &AudioHandle) -> Result<()> {
        env.call_method(get_ctx(), "audioPause", "(I)V", &[
            (handle.id as i32).into(),
        ])?;
        Ok(())
    }

    pub fn audio_set_current_time(env: &mut JNIEnv, handle: &AudioHandle, position: u128) -> Result<()> {
        env.call_method(get_ctx(), "audioSetCurrentTime", "(II)V", &[
            (handle.id as i32).into(),
            (position as i32).into(),
        ])?;
        Ok(())
    }

    pub fn audio_destroy(env: &mut JNIEnv, handle: &AudioHandle) -> Result<()> {
        env.call_method(get_ctx(), "audioDestroy", "(I)V", &[
            (handle.id as i32).into(),
        ])?;
        Ok(())
    }
}

static CTX: OnceCell<GlobalRef> = OnceCell::new();

fn get_ctx() -> &'static GlobalRef { CTX.get().unwrap() }

android_fn!(com_fujipod, fujipod, audio_RustAudioService, create, [JObject]);
#[allow(non_snake_case)]
pub unsafe fn create(env: JNIEnv, _: JClass, context: JObject) {
    let context = env.new_global_ref(context).unwrap();
    CTX.get_or_init(move || { context });
}

android_fn!(com_fujipod, fujipod, audio_RustAudioService, onEnded, [i32]);
#[allow(non_snake_case)]
pub unsafe fn onEnded(_: JNIEnv, _: JClass, id: i32) {
    if let Some(Listener { on_ended: Some(cb) }) = LISTENERS.lock().unwrap().get(&(id as usize)) {
        cb();
    }
}