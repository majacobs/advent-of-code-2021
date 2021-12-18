use std::collections::VecDeque;
use std::fs;

const LITERAL_PACKET_TYPE_ID: u32 = 0b100;

pub fn run() {
    let content = fs::read_to_string("input/day16").expect("Unable to read input");
    let packets = parse(content.trim_end());

    println!("Day 16");
    println!("  Problem 1: {}", problem1(&packets));
    println!("  Problem 2: {}", problem2(&packets));
}

fn parse(raw: &str) -> Vec<Packet> {
    let mut provider = BitProvider::new(raw);

    let mut packets = Vec::new();
    while !provider.exhausted {
        let (packet, _) = parse_one(&mut provider);
        packets.push(packet);
    }

    packets
}

fn parse_one(provider: &mut BitProvider) -> (Packet, u32) {
    let version = provider.get(3);
    let type_id = provider.get(3);
    let mut consumed = 6;

    if type_id == LITERAL_PACKET_TYPE_ID {
        let mut value = 0u64;
        loop {
            let v = provider.get(5);
            consumed += 5;
            value = (value << 4) | (v as u64 & 0b1111);
            //value.push(v);

            if (v >> 4) & 1 == 0 {
                break;
            }
        }

        (
            Packet {
                version,
                r#type: Type::Literal(value),
            },
            consumed,
        )
    } else {
        let length_type = provider.get(1);
        consumed += 1;
        let mut subpackets = Vec::new();
        if length_type == 0 {
            let subpacket_length = provider.get(15);
            consumed += 15;
            let mut total_subconsumed = 0;
            while total_subconsumed < subpacket_length {
                let (subpacket, subconsumed) = parse_one(provider);
                subpackets.push(subpacket);
                total_subconsumed += subconsumed;
            }
            assert_eq!(total_subconsumed, subpacket_length);
            consumed += total_subconsumed;
        } else {
            let num_subpackets = provider.get(11);
            consumed += 11;
            for _ in 0..num_subpackets {
                let (subpacket, subconsumed) = parse_one(provider);
                subpackets.push(subpacket);
                consumed += subconsumed;
            }
        }
        (
            Packet {
                version,
                r#type: Type::Operator {
                    type_id: type_id.into(),
                    subpackets,
                },
            },
            consumed,
        )
    }
}

fn problem1(packets: &[Packet]) -> u32 {
    let mut version_total = 0;

    for packet in packets {
        version_total += packet.version;
        if let Type::Operator {
            type_id: _,
            subpackets,
        } = &packet.r#type
        {
            version_total += problem1(subpackets);
        }
    }

    version_total
}

fn problem2(packets: &[Packet]) -> u64 {
    packets[0].eval()
}

struct BitProvider {
    bytes: VecDeque<u8>,
    exhausted: bool,
    current: Option<(u8, u32)>,
}

impl BitProvider {
    fn new(hex: &str) -> Self {
        let bytes: VecDeque<u8> = (0..hex.len())
            .step_by(2)
            .map(|i| u8::from_str_radix(&hex[i..i + 2], 16).unwrap())
            .collect();
        Self {
            bytes,
            exhausted: false,
            current: None,
        }
    }

    fn get(&mut self, mut num_bits: u32) -> u32 {
        assert!(num_bits <= u32::BITS);
        let mut value = 0u32;

        while num_bits > 0 {
            let (current, remaining) = match self.current {
                Some((c, r)) => (c, r),
                None => {
                    match self.bytes.pop_front() {
                        Some(b) => (b, 8),
                        //None => panic!(),
                        None => {
                            self.exhausted = true;
                            (0, 8)
                        }
                    }
                }
            };

            let to_provide = remaining.min(num_bits);
            value = (value << to_provide) | ((current >> (u8::BITS - to_provide)) as u32);
            num_bits -= to_provide;
            if remaining > to_provide {
                self.current = Some((current << to_provide, remaining - to_provide));
            } else {
                self.current = None;
            }
        }

        value
    }
}

#[derive(Debug)]
struct Packet {
    version: u32,
    r#type: Type,
}

impl Packet {
    fn eval(&self) -> u64 {
        match &self.r#type {
            Type::Literal(v) => *v,
            Type::Operator {
                type_id,
                subpackets,
            } => match type_id {
                TypeId::Sum => subpackets.iter().map(Packet::eval).sum(),
                TypeId::Product => subpackets.iter().map(Packet::eval).product(),
                TypeId::Minimum => subpackets.iter().map(Packet::eval).min().unwrap(),
                TypeId::Maximum => subpackets.iter().map(Packet::eval).max().unwrap(),
                TypeId::GreaterThan => {
                    if subpackets[0].eval() > subpackets[1].eval() {
                        1
                    } else {
                        0
                    }
                }
                TypeId::LessThan => {
                    if subpackets[0].eval() < subpackets[1].eval() {
                        1
                    } else {
                        0
                    }
                }
                TypeId::EqualTo => {
                    if subpackets[0].eval() == subpackets[1].eval() {
                        1
                    } else {
                        0
                    }
                }
            },
        }
    }
}

#[derive(Debug)]
enum Type {
    Literal(u64),
    Operator {
        type_id: TypeId,
        subpackets: Vec<Packet>,
    },
}

#[derive(Debug)]
enum TypeId {
    Sum,
    Product,
    Minimum,
    Maximum,
    GreaterThan,
    LessThan,
    EqualTo,
}

impl From<u32> for TypeId {
    fn from(v: u32) -> Self {
        match v {
            0 => Self::Sum,
            1 => Self::Product,
            2 => Self::Minimum,
            3 => Self::Maximum,
            5 => Self::GreaterThan,
            6 => Self::LessThan,
            7 => Self::EqualTo,
            _ => panic!(),
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn provider1() {
        let mut provider = BitProvider::new("D2FE28");
        assert_eq!(provider.get(3), 0b110);
        assert_eq!(provider.get(3), 0b100);
        assert_eq!(provider.get(5), 0b10111);
        assert_eq!(provider.get(5), 0b11110);
        assert_eq!(provider.get(5), 0b00101);
    }

    #[test]
    fn provider2() {
        let mut provider = BitProvider::new("38006F45291200");
        assert_eq!(provider.get(3), 0b001);
        assert_eq!(provider.get(3), 0b110);
        assert_eq!(provider.get(1), 0b0);
        assert_eq!(provider.get(15), 0b000000000011011);
        assert_eq!(provider.get(11), 0b11010001010);
        assert_eq!(provider.get(16), 0b0101001000100100);
    }

    #[test]
    fn example1_sample1() {
        let packets = parse("8A004A801A8002F478");
        assert_eq!(problem1(&packets), 16);
    }

    #[test]
    fn example1_sample2() {
        let packets = parse("620080001611562C8802118E34");
        assert_eq!(problem1(&packets), 12);
    }

    #[test]
    fn example1_sample3() {
        let packets = parse("C0015000016115A2E0802F182340");
        assert_eq!(problem1(&packets), 23);
    }

    #[test]
    fn example1_sample4() {
        let packets = parse("A0016C880162017C3686B18A3D4780");
        assert_eq!(problem1(&packets), 31);
    }

    #[test]
    fn example2_sample1() {
        let packets = parse("C200B40A82");
        assert_eq!(problem2(&packets), 3);
    }

    #[test]
    fn example2_sample2() {
        let packets = parse("04005AC33890");
        assert_eq!(problem2(&packets), 54);
    }

    #[test]
    fn example2_sample3() {
        let packets = parse("880086C3E88112");
        assert_eq!(problem2(&packets), 7);
    }

    #[test]
    fn example2_sample4() {
        let packets = parse("CE00C43D881120");
        assert_eq!(problem2(&packets), 9);
    }

    #[test]
    fn example2_sample5() {
        let packets = parse("D8005AC2A8F0");
        assert_eq!(problem2(&packets), 1);
    }

    #[test]
    fn example2_sample6() {
        let packets = parse("F600BC2D8F");
        assert_eq!(problem2(&packets), 0);
    }

    #[test]
    fn example2_sample7() {
        let packets = parse("9C005AC2F8F0");
        assert_eq!(problem2(&packets), 0);
    }

    #[test]
    fn example2_sample8() {
        let packets = parse("9C0141080250320F1802104A08");
        assert_eq!(problem2(&packets), 1);
    }
}
