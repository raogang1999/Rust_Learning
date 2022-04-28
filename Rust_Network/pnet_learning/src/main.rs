use pnet::datalink::Channel::Ethernet;
use pnet::datalink::{self,NetworkInterface};
use pnet::packet::ethernet::{EthernetPacket, MutableEthernetPacket};
use pnet::packet::ip::IpNextHeaderProtocols;
use pnet::packet::tcp::TcpPackt;
use pnet::packet::Packet;
use pnet::packet::ipv4::Ipv4Packet
use std::env;
use pnet::packet::ethernet::Ethernet;
use pnet::packet::ethernet::EtherTypes;
use crate::EtherTypes::Ipv4;

fn main() {
    let interface_name = env::args().nth(1).unwrap();
    let interfaces = datalink::interfaces();
    let interface = interfaces
        .into_iter()
        .filter(|iface:&NetworkInterface|iface == interface_name)
        .next()
        .expect("Input fault interface");
    let (_tx,mut rx) = match datalink::channel(&interface,Default::default()) {
        Ok(Ethernet(tx,rx))=>(tx,rx),
        Ok(_)=>panic!("Other"),
        Err(e) => eprint!("error{}",e),
    };
    loop {
        match rx.next() {
            Ok(packet)=>{
                let packet = EthernetPacket::new(packet).unwrap();
                handle_packet(&packet);
            },
            Err(E) =>panic!("ERR {}",E),
        }
    }


}
// fn handle_packet(ethernet: &EthernetPack){
//     match ethernet.get_ethernettype() {
//         EtherTypes::Ipv4 =>{
//             let header = Ipv4Pack::new(ethernet.palyoad());
//             if let Some(header) = header{
//                 match header.get_next_level_protocol() { }
//             }
//         }
//         _ => println!("ignoring");
//     }
// }
