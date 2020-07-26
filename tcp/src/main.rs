extern crate etherparse;
use etherparse::*;
use std::net::Ipv4Addr;
use std::collections::HashMap;
use std::io;
use tun_tap;

mod tcp;
#[derive(Clone, Copy, Debug, Hash, Eq, PartialEq)]
struct Quad {
    src:(Ipv4Addr, u16),
    dst:(Ipv4Addr, u16),
}

fn main() -> io::Result<()> {
    let connections: HashMap<Quad, tcp::Connection> = Default::default();
    let nic = tun_tap::Iface::without_packet_info("tun0", tun_tap::Mode::Tun)?;
    let mut buf = [0u8; 1504];
    loop {
        let nbytes = nic.recv(&mut buf[..])?;
        let flags = u16::from_be_bytes([buf[0], buf[1]]);
        let proto = u16::from_be_bytes([buf[2], buf[3]]);
        if proto != 0x0800 {
            // See https://en.wikipedia.org/wiki/EtherType
            // no ipv4, skip
            eprintln!("skiping");
            continue;
        }
        match etherparse::Ipv4HeaderSlice::from_slice(&buf[4..nbytes]) {
            Ok(iph) => {
                let src = iph.source_addr();
                let dst = iph.destination_addr();
                if iph.protocol() != 0x06 {
                    eprintln!("BAD PROTOCAL");
                    // not tcp
                    continue;
                }

                match etherparse::TcpHeaderSlice::from_slice(&buf[iph.slice().len()..nbytes]) {
                    Ok(tcph) => {
                        use std::collections::hash_map::Entry;
                        let datai = iph.slice().len() + tcph.slice().len();
                        match connections.entry(Quad {
                            src: (src, tcph.source_port()),
                            dst: (dst, tcph.destination_port()),
                        }) {
                            Entry::Occupied(mut c) => {
                                c.get_mut().on_packet(&mut nic, iph, tcph, &buf[datai..nbytes])?;
                            }
                            Entry::Vacant(mut e) => {
                                if let Some(c) = tcp::Connection::
                            }

                        }
                    }

                }

            }
            Err(e) => {
                eprintln!("ignoring weird packet {:?}", e);

            }
        }
    }
}
