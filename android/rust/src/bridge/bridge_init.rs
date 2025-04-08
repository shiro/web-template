use std::ffi::{CStr, CString};
use std::fs::File;
use std::io::{BufRead, BufReader};
use std::os::fd::{FromRawFd, RawFd};
use std::sync::Condvar;
use ndk::looper::{FdEvent, ThreadLooper};

use crate::*;
use crate::bridge::main_pipe::{MAIN_PIPE, MainPipe};

pub const NDK_GLUE_LOOPER_EVENT_PIPE_IDENT: i32 = 0;
pub const NDK_GLUE_LOOPER_INPUT_QUEUE_IDENT: i32 = 1;


pub unsafe fn create(
    env: JNIEnv,
    _jclass: JClass,
    activity_obj: JObject,
) {
    let activity = env.new_global_ref(activity_obj).unwrap();
    let vm = env.get_java_vm().unwrap();

    // this throws on some android versions since destroy doesn't get called on recents swipe
    let _ = std::panic::catch_unwind(|| {
        ndk_context::initialize_android_context(
            vm.get_java_vm_pointer() as *mut _,
            activity.as_raw() as *mut _,
        );
    });

    // let env = vm.attach_current_thread_as_daemon().unwrap();
    // let looper = ThreadLooper::for_thread().unwrap().into_foreign();
    // setup(env, &looper, activity);
}


// unsafe fn setup(env: JNIEnv, looper: &ForeignLooper, activity: GlobalRef) {
//     let mut main_pipe = MainPipe {
//         env,
//         activity,
//         // webview: None,
//         // webchrome_client: env.new_global_ref(webchrome_client).unwrap(),
//     };
//
//     looper
//         .add_fd_with_callback(MAIN_PIPE[0], FdEvent::INPUT, move |_| {
//             let size = std::mem::size_of::<bool>();
//             let mut wake = false;
//             if libc::read(MAIN_PIPE[0], &mut wake as *mut _ as *mut _, size) == size as libc::ssize_t {
//                 match main_pipe.recv() {
//                     Ok(_) => true,
//                     Err(_) => false,
//                 }
//             } else {
//                 false
//             }
//         })
//         .unwrap();
// }
