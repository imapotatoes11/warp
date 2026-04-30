use crate::channel::ChannelState;
use warp_core::channel::Channel;

pub fn apply_oss_server_root_override() {
    if !matches!(ChannelState::channel(), Channel::Oss) {
        return;
    }

    if let Ok(url) = std::env::var(warp_cli::SERVER_ROOT_URL_OVERRIDE_ENV) {
        if let Err(e) = ChannelState::override_server_root_url(url) {
            eprintln!("Error: Invalid server root URL: {e:#}");
        }
    }
}
