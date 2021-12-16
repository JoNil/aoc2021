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
    fn sum_versions(&self) -> i32 {
        match self {
            Packet::Literal { version, .. } => *version as i32,
            Packet::Operator {
                version, packets, ..
            } => *version as i32 + packets.iter().map(|p| p.sum_versions()).sum::<i32>(),
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

fn solve(input: &str) -> i32 {
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

    packet.sum_versions()
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
        let input = "8A004A801A8002F478";
        assert_eq!(crate::solve(input), 16);

        let input = "620080001611562C8802118E34";
        assert_eq!(crate::solve(input), 12);

        let input = "C0015000016115A2E0802F182340";
        assert_eq!(crate::solve(input), 23);

        let input = "A0016C880162017C3686B18A3D4780";
        assert_eq!(crate::solve(input), 31);
    }
}
