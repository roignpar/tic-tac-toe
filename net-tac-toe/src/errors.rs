use failure::Fail;

#[derive(Debug, Fail)]
pub enum MultiplayerError {
    #[fail(display = "Received message of unknown type ({}) from peer", msg_type)]
    UnknownMsgType { msg_type: u8 },

    #[fail(display = "Incomplete message data received from peer")]
    IncompleteMsgData,
}
