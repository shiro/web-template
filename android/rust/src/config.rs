use crate::*;

pub const fn public_host() -> &'static str { dotenv!("PUBLIC_HOST") }

pub const fn public_port() -> &'static str { dotenv!("PUBLIC_PORT") }

pub fn https_enabled() -> bool { dotenv!("HTTPS_ENABLED") == "1" }

pub fn log_level() -> &'static str { dotenv!("LOG_LEVEL") }

pub fn public_proto() -> &'static str { if https_enabled() { "https" } else { "http" } }

pub fn target_url() -> String {
    format!("{}://{}:{}", public_proto(), public_host(), public_port())
}
