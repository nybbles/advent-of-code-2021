use bitvec::prelude::*;
use num_derive::FromPrimitive;
use num_traits::FromPrimitive;

const BYTE_SIZE: usize = 8;
const MIN_PACKET_SIZE: usize = 6;
const PACKET_VERSION_SIZE: usize = 3;
const PACKET_TYPE_SIZE: usize = 3;
const LITERAL_GROUP_SIZE: usize = 4;

type Literal = u32;
type PacketScalarType = u8;
type PacketVersion = u8;
type OpType = PacketScalarType;

struct PacketHeader {
    packet_version: u8,
    packet_type: u8,
}

enum Packet {
    Literal(PacketHeader, Literal),
    Operator(PacketHeader, Vec<Packet>),
}

#[derive(FromPrimitive)]
enum PacketType {
    Literal = 4,
}

type D16BitSlice = BitSlice<Msb0, u8>;

fn decode_literal(bits: &D16BitSlice) -> (Literal, &D16BitSlice) {
    println!("START decode_literal {}", bits.to_string());
    let mut result = bitvec![Msb0,u8;];
    let mut ngroups = 0;
    let mut zero_bit_group_found = false;

    for (i, chunk) in bits.chunks_exact(LITERAL_GROUP_SIZE + 1).enumerate() {
        let more_groups = chunk[0];
        let group_bits = &chunk[1..LITERAL_GROUP_SIZE + 1];

        result.extend(group_bits);
        ngroups = i + 1;

        if !more_groups {
            zero_bit_group_found = true;
            break;
        }
    }

    assert!(zero_bit_group_found);

    let remaining_bits = &bits[ngroups * (LITERAL_GROUP_SIZE + 1)..];
    println!("END decode_literal {}", result.load_be::<Literal>());
    return (result.load_be::<Literal>(), remaining_bits);
}

fn decode_operator_for_bits(bits: &D16BitSlice) -> (Vec<Packet>, &D16BitSlice) {
    println!("START decode_operator_for_bits {}", bits.to_string());
    const TOTAL_LENGTH_IN_BITS_SIZE: usize = 15;
    let total_length = bits[..TOTAL_LENGTH_IN_BITS_SIZE].load_be::<usize>();
    let mut remaining_bits =
        &bits[TOTAL_LENGTH_IN_BITS_SIZE..TOTAL_LENGTH_IN_BITS_SIZE + total_length];
    let all_remaining_bits = &bits[TOTAL_LENGTH_IN_BITS_SIZE + total_length..];

    println!(
        "decode_operator_for_bits encoded bits length {}, length {}, remaining bits {}",
        bits[..TOTAL_LENGTH_IN_BITS_SIZE].to_string(),
        total_length,
        remaining_bits.to_string()
    );

    let mut result: Vec<Packet> = Vec::new();

    while remaining_bits.len() >= MIN_PACKET_SIZE {
        let (new_packet, bits) = decode_packet(remaining_bits);
        remaining_bits = bits;
        result.push(new_packet);
    }

    println!(
        "END decode_operator_for_bits {}",
        all_remaining_bits.to_string()
    );
    (result, all_remaining_bits)
}

fn decode_operator_by_subpackets(bits: &D16BitSlice) -> (Vec<Packet>, &D16BitSlice) {
    println!("START decode_operator_by_subpackets {}", bits.to_string());
    const NUMBER_OF_SUBPACKETS_SIZE: usize = 11;
    let nsubpackets = bits[..NUMBER_OF_SUBPACKETS_SIZE].load_be::<usize>();
    let mut remaining_bits = &bits[NUMBER_OF_SUBPACKETS_SIZE..];

    println!(
        "decode_operator_for_bits nsubpacket bits {}, nsubpackets {}, remaining bits {}",
        bits[..NUMBER_OF_SUBPACKETS_SIZE].to_string(),
        nsubpackets,
        remaining_bits.to_string()
    );

    let mut result: Vec<Packet> = Vec::new();

    for _ in 0..nsubpackets {
        let (new_packet, bits) = decode_packet(remaining_bits);
        remaining_bits = bits;
        result.push(new_packet);
    }

    println!(
        "END decode_operator_by_subpackets {}",
        remaining_bits.to_string()
    );
    (result, remaining_bits)
}

fn decode_operator(bits: &D16BitSlice) -> (Vec<Packet>, &D16BitSlice) {
    let rest = &bits[1..];
    println!("START decode_operator {}, {}", bits[0], rest.to_string());
    let result = if bits[0] {
        decode_operator_by_subpackets(rest)
    } else {
        decode_operator_for_bits(rest)
    };

    println!("END decode_operator {}", result.1);
    return result;
}

fn decode_packet(packet_bits: &D16BitSlice) -> (Packet, &D16BitSlice) {
    const LITERAL_START: usize = PACKET_VERSION_SIZE + PACKET_TYPE_SIZE;

    let packet_type = packet_bits[PACKET_VERSION_SIZE..LITERAL_START].load_be::<u8>();
    let packet_header = PacketHeader {
        packet_version: packet_bits[0..PACKET_VERSION_SIZE].load_be::<u8>(),
        packet_type: packet_type,
    };
    let packet_contents = &packet_bits[LITERAL_START..];

    println!(
        "START decode_packet version {}, type {} -> {}, {}",
        packet_header.packet_version,
        &packet_bits[PACKET_VERSION_SIZE..LITERAL_START],
        packet_header.packet_type,
        packet_bits.to_string()
    );

    match FromPrimitive::from_u8(packet_type) {
        Some(PacketType::Literal) => {
            let (literal, remaining_bits) = decode_literal(packet_contents);
            println!("END decode_packet {}", remaining_bits.to_string());
            return (Packet::Literal(packet_header, literal), remaining_bits);
        }
        None => {
            let (operator, remaining_bits) = decode_operator(packet_contents);
            println!("END decode_packet {}", remaining_bits.to_string());
            return (Packet::Operator(packet_header, operator), remaining_bits);
        }
    }
}

fn decode_hex(input: &str) -> Vec<u8> {
    match hex::decode(input) {
        Ok(bits) => bits,
        Err(_e) => panic!("unable to decode hex"),
    }
}

fn decode_packet_from_hex(input: &str) -> (Packet, BitVec<Msb0>) {
    let decoded = decode_hex(input);
    let bits = (&decoded).view_bits::<Msb0>();
    let (packet, remaining_bits) = decode_packet(bits);
    let remaining_bits_bv = remaining_bits.iter().collect();
    (packet, remaining_bits_bv)
}

fn compute_version_sum(packet: &Packet) -> u32 {
    match packet {
        Packet::Literal(packet_header, _literal) => packet_header.packet_version.into(),
        Packet::Operator(packet_header, packets) => {
            let packet_version_sum: u32 = packet_header.packet_version.into();
            let subpacket_version_sums: u32 = packets
                .into_iter()
                .map(|packet| compute_version_sum(packet))
                .sum();
            packet_version_sum + subpacket_version_sums
        }
    }
}

#[test]
fn test() {
    let inputs = [
        "D2FE28",
        "38006F45291200",
        "EE00D40C823060",
        "8A004A801A8002F478",
        "620080001611562C8802118E34",
        "C0015000016115A2E0802F182340",
        "A0016C880162017C3686B18A3D4780",
    ];

    for input in inputs {
        println!("START Testing {}", input);
        let (packet, remaining_bits) = decode_packet_from_hex(input);
        println!("Packet version sum {}", compute_version_sum(&packet));
        println!("END Testing {}\n", input);
    }
}

fn main() {
    // let input = "D2FE28";
    // let input = "38006F45291200";
    // let input = "EE00D40C823060";
    // let input = "8A004A801A8002F478";
    // let input = "620080001611562C8802118E34";
    // let input = "C0015000016115A2E0802F182340";
    let input = "A0016C880162017C3686B18A3D4780";

    let (packet, remaining_bits) = decode_packet_from_hex(input);
    println!("Packet version sum {}", compute_version_sum(&packet));
}
