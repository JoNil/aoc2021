use aoc2021::get_input;
use bit_field::BitArray;

#[allow(dead_code)]
#[derive(Debug)]
enum Packet {
    Literal {
        version: u8,
        value: u64,
    },
    Operator {
        version: u8,
        packet_type: u8,
        packets: Vec<Packet>,
    },
}

impl Packet {
    fn calculate(&self) -> i64 {
        match self {
            Packet::Literal { value, .. } => *value as i64,
            Packet::Operator {
                packet_type,
                packets,
                ..
            } => {
                let mut operands = packets.iter().map(|p| p.calculate());

                match *packet_type {
                    0 => operands.sum(),
                    1 => operands.product(),
                    2 => operands.min().unwrap(),
                    3 => operands.max().unwrap(),
                    5 => {
                        if operands.next().unwrap() > operands.next().unwrap() {
                            1
                        } else {
                            0
                        }
                    }
                    6 => {
                        if operands.next().unwrap() < operands.next().unwrap() {
                            1
                        } else {
                            0
                        }
                    }
                    7 => {
                        if operands.next().unwrap() == operands.next().unwrap() {
                            1
                        } else {
                            0
                        }
                    }
                    _ => {
                        panic!("Unknown type");
                    }
                }
            }
        }
    }
}

struct Parser {
    bit_index: usize,
    data: Vec<u8>,
}

impl Parser {
    fn new(mut data: Vec<u8>) -> Self {
        data.reverse();
        Self {
            bit_index: data.len() * 8,
            data,
        }
    }

    fn take_bits(&mut self, bits: usize) -> u8 {
        let res = self.data.get_bits((self.bit_index - bits)..self.bit_index);
        self.bit_index -= bits;
        res
    }

    fn take_literal(&mut self) -> u64 {
        let mut val = 0;
        let mut has_more = 1;

        while has_more > 0 {
            has_more = self.take_bits(1);
            let nibble = self.take_bits(4);

            val = (val << 4) | (nibble as u64);
        }

        val
    }

    fn take_packet(&mut self) -> Packet {
        let version = self.take_bits(3);
        let packet_type = self.take_bits(3);

        match packet_type {
            4 => {
                let value = self.take_literal();
                Packet::Literal { version, value }
            }
            _ => {
                if self.take_bits(1) == 1 {
                    let sub_packet_count = self.take_bits(8) as u32;
                    let sub_packet_count = sub_packet_count << 3 | self.take_bits(3) as u32;

                    let mut sub_packets = Vec::new();

                    for _ in 0..sub_packet_count {
                        sub_packets.push(self.take_packet());
                    }

                    Packet::Operator {
                        version,
                        packet_type,
                        packets: sub_packets,
                    }
                } else {
                    let sub_packet_bits = self.take_bits(8) as u32;
                    let sub_packet_bits = sub_packet_bits << 7 | self.take_bits(7) as u32;

                    let current_index = self.bit_index;

                    let mut sub_packets = Vec::new();

                    loop {
                        sub_packets.push(self.take_packet());

                        if current_index - self.bit_index == sub_packet_bits as usize {
                            break;
                        }
                    }

                    Packet::Operator {
                        version,
                        packet_type,
                        packets: sub_packets,
                    }
                }
            }
        }
    }
}

fn solve(input: &str) -> i64 {
    let mut parser = Parser::new(
        input
            .trim()
            .as_bytes()
            .chunks(2)
            .map(|s| {
                let s = std::str::from_utf8(s).unwrap();
                if s.len() == 1 {
                    u8::from_str_radix(s, 16).unwrap() << 4
                } else {
                    u8::from_str_radix(s, 16).unwrap()
                }
            })
            .collect(),
    );

    let packet = parser.take_packet();

    packet.calculate()
}

fn main() {
    let input = get_input();
    let start = std::time::Instant::now();
    let res = solve(&input);
    let end = start.elapsed();
    println!(
        "Day {} ({:?}): {}",
        aoc2021::get_program_name(),
        end.as_micros(),
        res
    );
}

#[cfg(test)]
mod test {
    #[test]
    fn test() {
        let input = "C200B40A82";
        assert_eq!(crate::solve(input), 3);

        let input = "04005AC33890";
        assert_eq!(crate::solve(input), 54);

        let input = "880086C3E88112";
        assert_eq!(crate::solve(input), 7);

        let input = "CE00C43D881120";
        assert_eq!(crate::solve(input), 9);

        let input = "D8005AC2A8F0";
        assert_eq!(crate::solve(input), 1);

        let input = "F600BC2D8F";
        assert_eq!(crate::solve(input), 0);

        let input = "9C005AC2F8F0";
        assert_eq!(crate::solve(input), 0);

        let input = "9C0141080250320F1802104A08";
        assert_eq!(crate::solve(input), 1);
    }
}
