use std::net::SocketAddrV4;

use rasta_rs::{message::Message, RastaListener};

fn on_receive(msg: Message) {
    dbg!(msg.data());
}

fn main() {
    let addr: SocketAddrV4 = "127.0.0.1:8888".parse().unwrap();
    let mut conn = RastaListener::try_new(addr).unwrap();
    conn.listen(on_receive).unwrap();
}
