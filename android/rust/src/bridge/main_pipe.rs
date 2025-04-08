use crate::*;
use once_cell::sync::Lazy;
use crossbeam_channel::*;
use std::os::unix::prelude::*;

pub enum NativeMessage {
    A
}

static CHANNEL: Lazy<(Sender<NativeMessage>, Receiver<NativeMessage>)> = Lazy::new(|| bounded(8));
pub static MAIN_PIPE: Lazy<[RawFd; 2]> = Lazy::new(|| {
    let mut pipe: [RawFd; 2] = Default::default();
    unsafe { libc::pipe(pipe.as_mut_ptr()) };
    pipe
});

pub struct MainPipe<'a> {
    pub env: JNIEnv<'a>,
    pub activity: GlobalRef,
    // pub webview: Option<GlobalRef>,
    // pub webchrome_client: GlobalRef,
}

impl MainPipe<'_> {
    pub fn send(message: NativeMessage) {
        let size = std::mem::size_of::<bool>();
        if let Ok(()) = CHANNEL.0.send(message) {
            unsafe { libc::write(MAIN_PIPE[1], &true as *const _ as *const _, size) };
        }
    }

    pub fn recv(&mut self) -> Result<(), JniError> {
        log!(Level::Info, "BIBI receive");

        let env = &mut self.env;
        let activity = self.activity.as_obj();
        if let Ok(message) = CHANNEL.1.recv() {

            log!(Level::Info, "BIBI receive msg");
            // do stuff with env
            match message {
                NativeMessage::A => {
                    //
                    log!(Level::Info, "BIBI receive A");
                    let _ = env.call_method(activity, "foo", "()V", &[]);
                }
            }
        }
        Ok(())
    }
}
