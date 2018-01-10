#![allow(dead_code)]
#![allow(non_camel_case_types)]
#![feature(conservative_impl_trait)]

extern crate byteorder;
extern crate bytes;
extern crate tokio_core;
extern crate tokio_io;
extern crate futures;

pub mod packets;
pub mod bindings;

use futures::*;
use tokio_io::AsyncRead;
use tokio_io::io::*;
use tokio_io::codec::{Encoder, Decoder};
use tokio_core::net::TcpStream;
use tokio_core::reactor::Core;

use std::net::{IpAddr, Ipv4Addr, SocketAddr};
use std::io;

use packets::BNetIncomingPacket;
use packets::BNetOutgoingPacket;
use packets::s2c::PacketReader;
use packets::PacketID;

use bytes::{BytesMut, Bytes, Buf, BufMut};

enum DecodeState {
    Header,
    Body(PacketID, usize)
}

struct BNetPCodec {
    state: DecodeState
}

impl<R: Buf> PacketReader<R> for BNetPCodec {
}

impl Encoder for BNetPCodec {
    type Item = BNetOutgoingPacket;
    type Error = io::Error;

    fn encode(&mut self, item: BNetOutgoingPacket, dst: &mut BytesMut) -> Result<(), Self::Error> {
        dst.put(&item.data);

        Ok(())
    }
}

impl Decoder for BNetPCodec {
    type Item = BNetIncomingPacket;
    type Error = io::Error;

    fn decode(&mut self, src: &mut BytesMut) -> Result<Option<Self::Item>, Self::Error> {
        if let DecodeState::Header = self.state {
            if src.len() < 4 {
                return Ok(None);
            }

            // deref first to allow reborrow
            let mut buf = io::Cursor::new(&mut *src);
            let (id, length) = self.read_header(&mut buf);

            self.state = DecodeState::Body(id, length);
        }

        if let DecodeState::Body(id, length) = self.state {
            if src.len() < length {
                return Ok(None);
            }
        
            let src = src.split_to(length);
        }

        Ok(Some(BNetIncomingPacket{lol: 0u8}))
    }
}

pub fn test() {
    let mut core = Core::new().unwrap();
    let connector_handle = core.handle();
    let ipaddr = IpAddr::V4(Ipv4Addr::new(127, 0, 0, 1));
    let endpoint = SocketAddr::new(ipaddr, 1234);
    let a = TcpStream::connect(&endpoint, &connector_handle).map(|x| {
        let (rx, tx) = x.split();

        let header = [0xFFu8];

        write_all(tx, &header);
    });

    core.run(a).unwrap();
}