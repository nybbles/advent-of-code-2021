use bitvec::prelude::*;
use num_derive::FromPrimitive;
use num_traits::FromPrimitive;

const BYTE_SIZE: usize = 8;
const MIN_PACKET_SIZE: usize = 6;
const PACKET_VERSION_SIZE: usize = 3;
const PACKET_TYPE_SIZE: usize = 3;
const LITERAL_GROUP_SIZE: usize = 4;

type Literal = u32;

struct PacketHeader {
    packet_version: u8,
    packet_type: u8,
}

enum Packet {
    Literal(PacketHeader, Literal),
    Unimplemented(PacketHeader),
}

#[derive(FromPrimitive)]
enum PacketType {
    Literal = 4,
}

fn decode_literal(bits: &BitSlice<Msb0, u8>) -> u32 {
    let mut result = bitvec![Msb0,u8;];

    for chunk in bits.chunks_exact(LITERAL_GROUP_SIZE + 1) {
        let more_groups = chunk[0];
        let group_bits = &chunk[1..LITERAL_GROUP_SIZE + 1];

        result.extend(group_bits);

        if !more_groups {
            return result.load_be::<u32>();
        }
    }

    panic!("Did not see group with zero bit");
}

fn decode_packet(packet: &[u8]) -> Packet {
    const LITERAL_START: usize = PACKET_VERSION_SIZE + PACKET_TYPE_SIZE;

    assert!(packet.len() * BYTE_SIZE > MIN_PACKET_SIZE);
    let packet_bits = packet.view_bits::<Msb0>();

    let packet_type = packet_bits[PACKET_VERSION_SIZE..LITERAL_START].load::<u8>();
    let packet_header = PacketHeader {
        packet_version: packet_bits[0..PACKET_VERSION_SIZE].load::<u8>(),
        packet_type: packet_type,
    };

    match FromPrimitive::from_u8(packet_type) {
        Some(PacketType::Literal) => {
            Packet::Literal(packet_header, decode_literal(&packet_bits[LITERAL_START..]))
        }
        None => Packet::Unimplemented(packet_header),
    }
}

fn main() {
    let input = "D2FE28";
    let decoded = match hex::decode(input) {
        Ok(bits) => bits,
        Err(_e) => panic!("unable to decode hex"),
    };
    let bits: &[u8] = &decoded;
    decode_packet(bits);

    println!("{}", bits.view_bits::<Msb0>().to_string());
}
