//! # MHC Server mockup
//!
//! Author: Rafał Michalski <rafal.michalski@rodstudio.pl>
//! All rights reserved, for internal use only.
//!
//! How to use:
//!
//! ```
//! mhc [optional ip:port for binding udp socket]
//! ```
//!
#[macro_use]
extern crate serde_derive;
extern crate bincode;

use std::io;
use std::env;
use std::net::UdpSocket;

use bincode::deserialize;

const DEFAULT_BIND_ADDRESS: &'static str = "0.0.0.0:5000";

#[derive(Serialize, Deserialize, PartialEq, Debug)]
struct MHCPacket {
    protocol_id : u8,
    size        : u8,
    command_type: u8,
    head        : u8,
    preset      : u8,
    signature   : u8
}

const MHC_PACKET_SIZE: usize = std::mem::size_of::<MHCPacket>();

impl MHCPacket {
    /// Validate MHC data
    fn validate(&self) -> io::Result<()> {
        const MHC_PACKET_SIZE_U8: u8 = MHC_PACKET_SIZE as u8;
        match self {
            MHCPacket {
                protocol_id : 0x15,
                size        : MHC_PACKET_SIZE_U8,
                command_type: 0x01, // we will use only “Goto” command type
                head        : 0x00, // only headless command
                preset      : _,    // any preset will do
                signature   : 0x66
            } => Ok(()),
            _ => Err(io::Error::new(io::ErrorKind::Other, "Invalid MHC data packet"))
        }
    }
}

fn main() -> io::Result<()> {
    /* get bind address from first command line argument or else use default one */
    let address = env::args().nth(1)
                  .unwrap_or_else(|| DEFAULT_BIND_ADDRESS.to_owned());

    println!("Binding to: {}", address);
    /* let's create our UDP socket */
    let socket = UdpSocket::bind(&address)?;

    /* our buffer for UDP data */
    let mut buf = [0; 4096];

    /* main loop forever */
    loop {
        /* wait and match data from socket */
        match socket.recv_from(&mut buf) {
            Ok((MHC_PACKET_SIZE, src)) => { /* the expected size is correct */
                /* this operation is a formality, a no-op behind the scenes when optimized */
                let mhc_packet: MHCPacket = deserialize(&buf[..6])
                                            .map_err(|e| io::Error::new(io::ErrorKind::Other, e))?;
                match mhc_packet.validate() {
                    Ok(_) => {
                        println!("Valid MHC packet from: {} data: {:?}", src, mhc_packet);
                        /* send back empty UDP dgram */
                        let mut buf = [];
                        socket.send_to(&mut buf, &src)?;
                    },
                    Err(_) => {
                        eprintln!("Invalid MHC packet from: {} data: {:?}", src, mhc_packet);
                    }
                }
            },
            Ok((amt, src)) => {
                eprintln!("Invalid packet size: {} bytes received from: {}", amt, src);
            },
            Err(err) => {
                eprintln!("Error receiving packet: {:?}", err);
            }
         }
    }
}
