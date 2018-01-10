extern crate jekuthiel;
extern crate tokio_core;
extern crate tokio_io;
extern crate futures;

use futures::{Future, Stream};
use tokio_io::AsyncRead;
use tokio_io::io::*;
use tokio_core::net::TcpStream;
use tokio_core::reactor::Core;

use std::net::{IpAddr, Ipv4Addr, SocketAddr};

use jekuthiel::packets::*;

struct Receiver {

}



pub fn entry_point() {
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