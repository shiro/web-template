use crate::*;

pub struct NativeRun {
    vm: JavaVM,
}

impl NativeRun {
    pub fn new() -> Self {
        let ctx = ndk_context::android_context();
        let vm = unsafe { JavaVM::from_raw(ctx.vm().cast()) }.unwrap();
        vm.attach_current_thread_as_daemon().unwrap();

        Self { vm }
    }

    pub fn get_env(&self) -> JNIEnv {
        self.vm.get_env().unwrap()
    }
}

impl Drop for NativeRun {
    fn drop(&mut self) {
        unsafe { self.vm.detach_current_thread() };
    }
}
