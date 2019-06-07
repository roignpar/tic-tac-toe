use tokio::net::UdpFramed;

mod errors;
mod msg;

use msg::*;

pub struct Multiplayer {
    peer_messages: UdpFramed<MsgCodec>,
}
