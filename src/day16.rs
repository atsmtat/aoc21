struct Packet {
    version: u8,
    type_id: u8,
    data: Data,
}

enum Data {
    Literal(usize),
    Operator(Vec<Packet>),
}

fn parse_packet(bin_str: &str) -> (Packet, &str) {
    let version = u8::from_str_radix(&bin_str[0..3], 2).unwrap();
    let type_id = u8::from_str_radix(&bin_str[3..6], 2).unwrap();

    let (data, remain_str) = match type_id {
        4 => {
            let (literal, remain_str) = parse_literal(&bin_str[6..]);
            (Data::Literal(literal), remain_str)
        }
        _ => {
            let (packets, remain_str) = parse_operator(&bin_str[6..]);
            (Data::Operator(packets), remain_str)
        }
    };

    (
        Packet {
            version,
            type_id,
            data,
        },
        remain_str,
    )
}

fn parse_literal(bin_str: &str) -> (usize, &str) {
    let mut value = String::new();

    let mut ci = 0;
    loop {
        let grp_str = &bin_str[ci..ci + 5];
        ci += 5;
        value.push_str(&grp_str[1..5]);
        if &grp_str[0..1] == "0" {
            break;
        }
    }
    (usize::from_str_radix(&value, 2).unwrap(), &bin_str[ci..])
}

fn parse_operator(bin_str: &str) -> (Vec<Packet>, &str) {
    let mut sub_packets = vec![];
    let unparsed_str = match &bin_str[0..1] {
        "0" => {
            let mut pkts_len = usize::from_str_radix(&bin_str[1..16], 2).unwrap();
            let mut pkts_str = &bin_str[16..];
            while pkts_len > 0 {
                let (pkt, remain_str) = parse_packet(pkts_str);
                sub_packets.push(pkt);
                pkts_len -= pkts_str.len() - remain_str.len();
                pkts_str = remain_str;
            }
            pkts_str
        }

        "1" => {
            let pkts_count = usize::from_str_radix(&bin_str[1..12], 2).unwrap();
            let mut pkts_str = &bin_str[12..];
            for _ in 0..pkts_count {
                let (pkt, remain_str) = parse_packet(pkts_str);
                sub_packets.push(pkt);
                pkts_str = remain_str;
            }
            pkts_str
        }
        _ => unreachable!(),
    };
    (sub_packets, unparsed_str)
}

fn hex_to_bin_str(hex_str: &str) -> String {
    hex_str.chars().fold(String::new(), |mut acc, c| {
        let bin_str = match c {
            '0' => "0000",
            '1' => "0001",
            '2' => "0010",
            '3' => "0011",
            '4' => "0100",
            '5' => "0101",
            '6' => "0110",
            '7' => "0111",
            '8' => "1000",
            '9' => "1001",
            'A' => "1010",
            'B' => "1011",
            'C' => "1100",
            'D' => "1101",
            'E' => "1110",
            'F' => "1111",
            _ => "",
        };
        acc.push_str(bin_str);
        acc
    })
}

fn get_version_sum(pkt: &Packet) -> u32 {
    let mut sum = pkt.version as u32;
    sum += match &pkt.data {
        Data::Literal(_) => 0,
        Data::Operator(pkts) => pkts.iter().map(get_version_sum).sum(),
    };
    sum
}

fn evaluate(pkt: &Packet) -> u64 {
    match &pkt.data {
        Data::Literal(val) => *val as u64,
        Data::Operator(pkts) => {
            let val = match pkt.type_id {
                0 => pkts.iter().map(evaluate).sum::<u64>(),
                1 => pkts.iter().map(evaluate).product::<u64>(),
                2 => pkts.iter().map(evaluate).min().unwrap(),
                3 => pkts.iter().map(evaluate).max().unwrap(),
                5 => {
                    if evaluate(&pkts[0]) > evaluate(&pkts[1]) {
                        1
                    } else {
                        0
                    }
                }
                6 => {
                    if evaluate(&pkts[0]) < evaluate(&pkts[1]) {
                        1
                    } else {
                        0
                    }
                }
                7 => {
                    if evaluate(&pkts[0]) == evaluate(&pkts[1]) {
                        1
                    } else {
                        0
                    }
                }
                _ => unreachable!(),
            };
            val
        }
    }
}

fn part1(bin_str: &str) -> u32 {
    let bin_str = hex_to_bin_str(bin_str);
    let (pkt, _) = parse_packet(&bin_str);
    get_version_sum(&pkt)
}

fn part2(bin_str: &str) -> u64 {
    let bin_str = hex_to_bin_str(bin_str);
    let (pkt, _) = parse_packet(&bin_str);
    evaluate(&pkt)
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn puzzle_example_part1() {
        assert_eq!(16, part1("8A004A801A8002F478"));
        assert_eq!(12, part1("620080001611562C8802118E34"));
        assert_eq!(23, part1("C0015000016115A2E0802F182340"));
        assert_eq!(31, part1("A0016C880162017C3686B18A3D4780"));
    }

    #[test]
    fn puzzle_example_part2() {
        assert_eq!(3, part2("C200B40A82"));
        assert_eq!(54, part2("04005AC33890"));
        assert_eq!(7, part2("880086C3E88112"));
        assert_eq!(9, part2("CE00C43D881120"));
        assert_eq!(1, part2("D8005AC2A8F0"));
        assert_eq!(0, part2("F600BC2D8F"));
        assert_eq!(1, part2("9C0141080250320F1802104A08"));
    }
}

pub fn solve<I: Iterator<Item = String>>(mut lines: I, part: u8) {
    let input = lines.next().unwrap();
    match part {
        1 => println!("{}", part1(&input)),
        2 => println!("{}", part2(&input)),
        _ => unreachable!(),
    }
}
