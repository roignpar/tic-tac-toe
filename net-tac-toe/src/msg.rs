/// `Msg` is used to communicate between peers playing tic-tac-toe.
/// `MsgCodec` en/decodes `Msg`s according to the following rules:
///     * first byte describes the type of message
///     * subsequent bytes represent data values specific to the type of message
///     * data values are separated by one byte with value 255
///     * string values are base64 encoded
///     * data values order is specific to each message type
///     * a new line character (`\n`) signifies the end of a message
use bytes::{BufMut, BytesMut};
use failure::Error;
use tokio::codec::{Decoder, Encoder};

use crate::errors::MultiplayerError;

/// Separator byte.
const SEP: u8 = 255;
const NL: u8 = b'\n';

// Msg type bytes;
const PLAYER_INTRO: u8 = 1;

#[derive(Clone, Debug, PartialEq)]
pub enum Msg {
    PlayerIntro { name: String },
}

impl Msg {
    fn type_byte(&self) -> u8 {
        match self {
            Msg::PlayerIntro { .. } => PLAYER_INTRO,
        }
    }
}

pub struct MsgCodec {
    /// How far has the incomplete encoded buffer been scanned.
    scanned_to: usize,
}

impl MsgCodec {
    pub fn new() -> Self {
        Self { scanned_to: 0 }
    }
}

impl Decoder for MsgCodec {
    type Item = Msg;
    type Error = Error;

    fn decode(&mut self, buf: &mut BytesMut) -> Result<Option<Self::Item>, Self::Error> {
        if let Some(nl_offset) = buf[self.scanned_to..].iter().position(|b| *b == NL) {
            let nl_index = self.scanned_to + nl_offset;

            let msg_buf = buf.split_to(nl_index + 1);

            let msg = match msg_buf[0] {
                PLAYER_INTRO => {
                    // exclude the new line char
                    let mut data = msg_buf[1..msg_buf.len() - 1].split(|b| *b == SEP);

                    let encoded_name = data.next().ok_or(MultiplayerError::IncompleteMsgData)?;
                    let name = String::from_utf8(base64::decode(encoded_name)?)?;

                    Msg::PlayerIntro { name }
                }
                msg_type => return Err(MultiplayerError::UnknownMsgType { msg_type }.into()),
            };

            self.scanned_to = 0;

            Ok(Some(msg))
        } else {
            self.scanned_to = buf.len();

            Ok(None)
        }
    }
}

impl Encoder for MsgCodec {
    type Item = Msg;
    type Error = Error;

    fn encode(&mut self, msg: Self::Item, buf: &mut BytesMut) -> Result<(), Self::Error> {
        use Msg::*;

        // 1 for msg type, 1 for new line
        buf.reserve(2);

        match msg {
            PlayerIntro { ref name } => {
                let encoded_name = base64::encode(&name);

                buf.reserve(encoded_name.len());

                buf.put(msg.type_byte());
                buf.put(encoded_name);
                buf.put(NL);
            }
        }

        Ok(())
    }
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn encode_decode() {
        use Msg::*;

        let mut codec = MsgCodec::new();

        let mut buf = BytesMut::new();
        let msgs = [
            PlayerIntro {
                name: "Player 1".to_owned(),
            },
            PlayerIntro {
                name: "".to_owned(),
            },
            PlayerIntro {
                name: "Player\nNewLine\n".to_owned(),
            },
        ];

        for msg in msgs.iter() {
            codec.encode(msg.clone(), &mut buf).unwrap();

            assert_eq!(codec.decode(&mut buf).unwrap().unwrap(), *msg);
        }
    }
}
