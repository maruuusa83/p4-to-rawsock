// Copyright (c) 2018 Daichi Teruya @maruuusa83
// This project is released under the MIT license
// https://github.com/maruuusa83/p4-to-rawsock/blob/master/LISENCE
extern crate libc;

use std::io::Error;
use std::mem;

#[link(name="ioctl", kind="static")]
extern {
    fn socket(domain: libc::c_int,
              sock_type: libc::c_int,
              protocol: libc::c_int
            ) -> libc::c_int;

    fn ioctl_siocgifindex(fd: libc::c_int,
                          ifr: *mut IfReq,
                         ) -> libc::c_int;

    fn ioctl_siocgifhwaddr(fd: libc::c_int,
                          ifr: *mut IfReq,
                         ) -> libc::c_int;

    fn ioctl_set_promisc(fd: libc::c_int,
                          ifr: *mut IfReq,
                         );

    fn bind(socket: libc::c_int,
            addr: *const SockAddrLL,
            addrlen: libc::socklen_t) -> libc::c_int;

    fn recv(socket: libc::c_int,
            buffer: *const libc::c_char,
            length: libc::c_int,
            flags: libc::c_int
           ) -> libc::c_int;

    fn htons(d: libc::c_int) -> libc::c_int;

    fn flush_recv_buf(socket: Socket);
}

// *** socket *** //
pub enum Domain {
    Unix,
    Local,
    Inet6,
    Ipx,
    Netlink,
    X25,
    Ax25,
    Atmpvc,
    Appletalk,
    Packet,
    Alg,
}

pub enum SockType {
    Stream,
    Dgram,
    Seqpacket,
    Raw,
    Rdm,
    Packet,
}

pub enum Protocol {
    HtonsEthPAll
}

pub type Socket = i32;
pub fn c_socket(_domain: Domain,
                _sock_type: SockType,
                _protocol: Protocol
                ) -> Result<Socket, Error> {
    let domain = match _domain {
        Domain::Unix          => libc::AF_UNIX,
        Domain::Local         => libc::AF_LOCAL,
        Domain::Inet6         => libc::AF_INET6,
        Domain::Ipx           => libc::AF_IPX,
        Domain::Netlink       => libc::AF_NETLINK,
        Domain::X25           => libc::AF_X25,
        Domain::Ax25          => libc::AF_AX25,
        Domain::Atmpvc        => libc::AF_ATMPVC,
        Domain::Appletalk     => libc::AF_APPLETALK,
        Domain::Packet        => libc::AF_PACKET,
        Domain::Alg           => libc::AF_ALG,
    };

    let sock_type = match _sock_type {
        SockType::Stream       => libc::SOCK_STREAM,
        SockType::Dgram        => libc::SOCK_DGRAM,
        SockType::Seqpacket    => libc::SOCK_SEQPACKET,
        SockType::Raw          => libc::SOCK_RAW,
        SockType::Rdm          => libc::SOCK_RDM,
        SockType::Packet       => libc::SOCK_PACKET,
    };

    unsafe {
        let protocol = match _protocol {
            Protocol::HtonsEthPAll => htons(libc::ETH_P_ALL)
        };

        let sock_no: i32 = socket(domain, sock_type, protocol);
        if sock_no < 0 {
            return Result::Err(Error::last_os_error());
        }
        else {
            return Result::Ok(sock_no);
        }
    }
}

// *** ioctl *** //
#[repr(C)]
struct IfReq {
    ifr_name : [libc::c_char; libc::IF_NAMESIZE],
    data: IfReqDat,
}

#[repr(C)]
union IfReqDat {
    hwaddr: CSockAddr,
    ifindex: libc::c_int,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct CSockAddr {
    len: u8,
    family: u8,
    pub addr: [u8; 64],
}

#[repr(C)]
pub struct SockAddrLL {
    pub family: u16,
    pub protocol: u16,
    pub ifindex: i32,
    pub htype: u16,
    pub pkttype: u8,
    pub halen: u8,
    pub addr: [u8; 8],
}
impl SockAddrLL {
    pub fn for_bind(ifindex: i32) -> SockAddrLL {
        let protocol: u16;
        unsafe {
            protocol = htons(libc::ETH_P_ALL) as u16;
        }

        SockAddrLL {
            family: libc::AF_PACKET as u16,
            protocol: protocol,
            ifindex: ifindex,
            htype: 0xffff as u16,
            pkttype: 0xff as u8,
            halen: 0xff as u8,
            addr: [0xff, 0xff, 0xff, 0xff, 0xff, 0xff, 0xff, 0xff],
        }
    }
}

impl IfReq {
    fn new() -> Self {
        let name: [libc::c_char; libc::IF_NAMESIZE] = [0 as libc::c_char; libc::IF_NAMESIZE];
        IfReq {
            ifr_name: name,
            data: IfReqDat{ ifindex: 0 },
        }
    }

    fn from_name(name: &str) -> Option<IfReq> {
        if name.len() >= libc::IF_NAMESIZE {
            None
        }
        else {
            let mut ifreq: IfReq = IfReq::new();
            for (i, c) in name.as_bytes().iter().enumerate() {
                ifreq.ifr_name[i] = *c as libc::c_char;
            }
            Some(ifreq)
        }
    }
}

pub fn get_interface_index(socket: &Socket, ifname: &str) -> Result<i32, Error> {
    unsafe {
        let mut ifr = IfReq::from_name(ifname).unwrap();
        let res = ioctl_siocgifindex(*socket, &mut ifr as *mut IfReq);
        if res < 0 {
            return Result::Err(Error::last_os_error());
        }
        else {
            return Result::Ok(ifr.data.ifindex);
        }
    }
}

pub fn get_interface_hwaddr(socket: &Socket, ifname: &str) -> Result<CSockAddr, Error> {
    unsafe {
        let mut ifr = IfReq::from_name(ifname).unwrap();
        let res = ioctl_siocgifhwaddr(*socket, &mut ifr as *mut IfReq);

        if res < 0 {
            return Result::Err(Error::last_os_error());
        }
        else {
            return Result::Ok(ifr.data.hwaddr);
        }
    }
}

pub fn set_promiscuous_mode(socket: &Socket, ifname: &str)
{
    unsafe{
        let mut ifr = IfReq::from_name(ifname).unwrap();
        ioctl_set_promisc(*socket, &mut ifr as *mut IfReq);
    }
}

// *** bind *** //
pub fn c_bind(socket: &Socket,
              addr: &SockAddrLL,
              addrlen: u32) -> Result<i32, Error> {
    unsafe {
        let res = bind(*socket, addr as *const SockAddrLL, mem::size_of::<SockAddrLL>() as u32);
        if res < 0 {
            println!("{}", res);
            return Result::Err(Error::last_os_error());
        }
        else {
            return Result::Ok(res);
        }
    }
}

// *** flush *** //
pub fn c_flush_recv_buf(socket: &Socket) {
    unsafe {
        flush_recv_buf(*socket);
    }
}

// *** recv *** //
pub fn c_recv(socket: i32, buffer: &mut [u8; 2048]) -> i32 {
    let res: i32;
    unsafe {
        res = recv(socket, buffer.as_ptr() as *const libc::c_char, buffer.len() as libc::c_int, 0);
    }

    res
}

