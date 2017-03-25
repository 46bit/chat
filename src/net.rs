use std::io;
use bytes::BytesMut;
use tokio_io::codec::{Encoder, Decoder, Framed};
use tokio_core::net::TcpStream;

pub type StringTransport = Framed<TcpStream, StringCodec>;

/// This just passes through data.
#[derive(Clone, Copy, Debug)]
pub struct StringCodec;

impl Decoder for StringCodec {
    type Item = String;
    type Error = io::Error;

    fn decode(&mut self, buf: &mut BytesMut) -> io::Result<Option<String>> {
        if buf.is_empty() {
            Ok(None)
        } else {
            Ok(Some(String::from_utf8_lossy(buf.take().as_ref()).into_owned()))
        }
    }
}

impl Encoder for StringCodec {
    type Item = String;
    type Error = io::Error;

    fn encode(&mut self, msg: String, buf: &mut BytesMut) -> io::Result<()> {
        buf.extend(msg.bytes());
        Ok(())
    }
}
