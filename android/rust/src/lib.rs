#![allow(warnings)]
#![feature(result_option_inspect)]

use std::collections::HashMap;
use std::ops::{Deref, DerefMut};
use std::sync::Arc;
use std::sync::{Mutex, MutexGuard};
use std::thread;
use std::thread::JoinHandle;
use std::time::{Duration, Instant};

use anyhow::anyhow as err;
use anyhow::Result;
pub use jni::{
    self,
    errors::Error as JniError,
    objects::{GlobalRef, JClass, JMap, JObject, JString},
    sys::jobject,
    JNIEnv, JavaVM,
};
use log::{info, log, Level, LevelFilter};
use once_cell::sync::Lazy;
use once_cell::sync::OnceCell;
use tao_macros::android_fn;

// use dotenv_codegen::proc_macro_hack_dotenv as dotenv;

mod bridge;
// mod clamp;
// mod config;
// mod native_rust_config_plugin;
// mod option_interval;
// mod option_sleep;
// mod stream;

fn init_logging() {
    android_logger::init_once(
        android_logger::Config::default()
            .with_max_level(LevelFilter::Trace)
            .with_tag(env!("ANDROID_APPID")),
    );
}

android_fn!(com_app, app, MainActivity, create, [JObject]);
#[allow(non_snake_case)]
pub unsafe fn create(env: JNIEnv, this: JClass, activity: JObject) {
    init_logging();
    bridge::create(env, this, activity);

    info!("hi from rust MMMMMMM");
}
