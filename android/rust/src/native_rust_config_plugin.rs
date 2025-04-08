use jni::sys::jstring;
use serde::{Deserialize, Serialize};

use crate::*;

#[derive(Serialize, Deserialize)]
struct Config {
    PUBLIC_HOST: String,
    PUBLIC_PORT: String,
    HTTPS_ENABLED: bool,
    LOG_LEVEL: String,
}

android_fn!(com_fujipod, fujipod, plugins_ConfigCapacitorPlugin, config, [], jstring);
pub unsafe fn config(env: JNIEnv, _: JClass) -> jstring {
    env.new_string(
        serde_json::to_string(&Config {
            PUBLIC_HOST: config::public_host().to_string(),
            PUBLIC_PORT: config::public_port().to_string(),
            HTTPS_ENABLED: config::https_enabled(),
            LOG_LEVEL: config::log_level().to_string(),
        }).unwrap()
    ).unwrap().into_raw()
}