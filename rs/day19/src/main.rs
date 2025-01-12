use utils::timer::Timer;
use std::collections::HashSet;
use std::fs::read_to_string;
use std::path::PathBuf;

fn count_possible_designs<'a>(design: &'a [u8], patterns: &HashSet<&[u8]>, max_length: usize) -> u64 {
    let mut dp = vec![0u64; design.len()+1];
    dp[0] = 1; // base case

    for idx in 1..dp.len() {
        for end in idx..(idx+max_length).min(dp.len()) {
            let subdesign = &design[idx-1..end];
            if patterns.contains(subdesign) {
                dp[end] += dp[end - subdesign.len()];
            }
        }
    }

    *dp.last().unwrap()
}

fn main() {
    let _timer = Timer::new();

    let path: PathBuf = std::env::args_os()
        .nth(1)
        .expect("First argument should be the filename of the input")
        .into();

    let data = read_to_string(path)
        .expect("Should be able to read from");

    let (patterns, designs) = data
        .split_once("\n\n")
        .expect("Input should have blank line");

    let patterns: HashSet<&[u8]> = patterns
        .split(", ")
        .map(|x| x.as_bytes())
        .collect();

    let max_length = patterns
        .iter()
        .map(|x| x.len())
        .max()
        .unwrap();
    
    let answer = if cfg!(feature = "part2") {
        designs
            .lines()
            .map(|x| count_possible_designs(x.as_bytes(), &patterns, max_length))
            .sum()
    }
    else {
        designs
            .lines()
            .map(|x| count_possible_designs(x.as_bytes(), &patterns, max_length) > 0)
            .filter(|x| *x)
            .count() as u64
    };
    
    println!("Answer: {answer}");
}

#[cfg(test)]
mod tests {
     use super::*;

    #[test]
    fn test_simple_case() {
        let mut patterns: HashSet<&[u8]> = HashSet::new();
        patterns.insert(b"br");
        patterns.insert(b"g");
        patterns.insert(b"gr");

        assert!(count_possible_designs(b"brgr", &patterns, 2) > 0);
    }

    #[test]
    fn test_simple_case_ng() {
        let mut patterns: HashSet<&[u8]> = HashSet::new();
        patterns.insert(b"br");
        patterns.insert(b"g");
        patterns.insert(b"gr");

        assert_eq!(count_possible_designs(b"brugr", &patterns, 2), 0);
    }

    #[test]
    fn test_simple_case_ng_2() {
        let mut patterns: HashSet<&[u8]> = HashSet::new();
        patterns.insert(b"rgb");
        patterns.insert(b"bwu");

        assert_eq!(count_possible_designs(b"rgbwu", &patterns, 3), 0);
    }
}
