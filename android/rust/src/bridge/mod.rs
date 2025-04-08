use crate::*;

mod main_pipe;
mod bridge_init;
mod native_run;

pub use bridge_init::create;
pub use main_pipe::{MainPipe, NativeMessage};
pub use native_run::NativeRun;


pub fn get_main_activity() -> JObject<'static> {
    let ctx = ndk_context::android_context();
    let context = ctx.context() as jobject;
    unsafe { JObject::from_raw(context) }
}