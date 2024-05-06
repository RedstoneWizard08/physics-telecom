use std::cmp::max;
use crate::util::format_radix;

pub fn parse_binary(bin: impl AsRef<str>) -> u8 {
    bin.as_ref()
        .chars()
        .rev()
        .enumerate()
        .map(|(i, c)| (if c == '1' { 1 } else { 0 }) * 2_u8.pow(i as u32))
        .sum()
}

pub fn to_binary(s: impl AsRef<str>) -> Vec<String> {
    s.as_ref().chars().map(|c| char_to_binary(c)).collect()
}

pub fn char_to_binary(ch: char) -> String {
    left_pad(format_radix(ch as u8, 2), '0', 8)
}

// Glad this isn't its own crate :p
pub fn left_pad(s: impl AsRef<str>, ch: char, num: usize) -> String {
    format!(
        "{}{}",
        (0..max(0, num - s.as_ref().len()))
            .map(|_| ch)
            .collect::<String>(),
        s.as_ref()
    )
}

#[cfg(test)]
pub mod test {
    use crate::binary::{parse_binary, to_binary};
    use test::Bencher;

    pub const BENCH_ITERS: usize = 1000;

    #[test]
    pub fn can_parse_binary() {
        assert_eq!(parse_binary("01001010"), 0b01001010);
        assert_eq!(parse_binary("01101001"), 0b01101001);
        assert_eq!(parse_binary("11010110"), 0b11010110);
        assert_eq!(parse_binary("00110110"), 0b00110110);
        assert_eq!(parse_binary("11010011"), 0b11010011);
    }

    #[test]
    pub fn can_convert_strings() {
        assert_eq!(
            to_binary("hello!"),
            vec!["01101000", "01100101", "01101100", "01101100", "01101111", "00100001",]
        )
    }

    #[bench]
    pub fn bench_binary_parse(b: &mut Bencher) {
        b.iter(|| {
            (0..BENCH_ITERS).for_each(|_| {
                parse_binary("01001010");
            });
        });
    }
}
