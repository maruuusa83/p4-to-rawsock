// Copyright (c) 2018 Daichi Teruya @maruuusa83
// This project is released under the MIT license
// https://github.com/maruuusa83/p4-to-rawsock/blob/master/LISENCE
extern crate libc;
extern crate hexdump;

mod socket;

use socket::{Domain, SockType, Protocol, CSockAddr, SockAddrLL};
use socket::{c_socket, c_flush_recv_buf, get_interface_index, get_interface_hwaddr, set_promiscuous_mode, c_bind};

fn main() {
    let ifname = "lo";

    let sock = c_socket(Domain::Packet, SockType::Raw, Protocol::HtonsEthPAll).unwrap();
    let ifindex = get_interface_index(&sock, ifname).unwrap();
    let sock_addr: CSockAddr = get_interface_hwaddr(&sock, ifname).unwrap();
    set_promiscuous_mode(&sock, ifname);

    let a = SockAddrLL::for_bind(ifindex);
    let t = c_bind(&sock, &a, 6).unwrap();

    c_flush_recv_buf(&sock);

    let mut buffer: [u8; 2048] = [0u8; 2048];
    socket::c_recv(sock, &mut buffer);

    hexdump::hexdump(buffer.to_vec().as_slice());
}
