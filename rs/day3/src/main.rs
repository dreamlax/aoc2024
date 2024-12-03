use std::path::PathBuf;
use std::fs::read_to_string;
use utils::timer::Timer;

fn scan_line(mut line: &str) -> i64 {
    let mut sum: i64 = 0;

    while let Some(idx) = line.find("mul(") {
        line = &line[idx+4..];

        let comma = line.chars().take_while(char::is_ascii_digit).count();
        if comma == 0 || &line[comma..comma+1] != "," {
            line = &line[comma+1..];
            continue;
        }

        let terminator = comma+1 + line[comma+1..].chars().take_while(char::is_ascii_digit).count();
        if terminator == 0 || &line[terminator..terminator+1] != ")" {
            line = &line[terminator+1..];
            continue;
        }

        let operand1: i64 = line[0..comma].parse().unwrap();
        let operand2: i64 = line[comma+1..terminator].parse().unwrap();

        sum += operand1 * operand2;

        line = &line[terminator..];
    }

    sum
}

fn scan_line_limited(mut line: &str) -> i64 {
    let mut sum: i64 = 0;

    while let Some(dont_idx) = line.find("don't()") {
        sum += scan_line(&line[..dont_idx]);

        let Some(do_idx) = line[dont_idx..].find("do()") else {
            return sum;
        };

        line = &line[dont_idx+do_idx..];
    }

    sum + scan_line(&line)
}

fn main() {
    let _t = Timer::new();

    let path: PathBuf = std::env::args_os()
        .skip(1)
        .next()
        .expect("Should have file argument")
        .into();

    let input = read_to_string(path)
        .expect("Should be able to read from path");

    let answer: i64 = if cfg!(feature = "part2") {
        scan_line_limited(&input)
    }
    else {
        scan_line(&input)
    };

    println!("Answer: {answer}");
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_nothing() {
        let line = "asdfjanwrliuanerliuvbnfv";
        assert_eq!(scan_line(line), 0, "There are no mul() instructions");
    }

    #[test]
    fn test_something() {
        let line = "asdfasdfmul(10,2)adsfmul(20,1)afds";
        assert_eq!(scan_line(line), 40, "mul(10,2) + mul(20,1) == 40");
    }

    #[test]
    fn test_neighbouring() {
        let line = "asdfasfdmul(10,2)mul(20,1)asdf";
        assert_eq!(scan_line(line), 40, "mul(10,2) + mul(20,1) == 40");
    }

    #[test]
    fn test_start_and_end() {
        let line = "mul(20,1)asdffdsamul(10,2)";
        assert_eq!(scan_line(line), 40, "mul(20,1) + mul(10,2) == 40");
    }

    #[test]
    fn test_demo_input() {
        let line = "xmul(2,4)%&mul[3,7]!@^do_not_mul(5,5)+mul(32,64]then(mul(11,8)mul(8,5))";
        assert_eq!(scan_line(line), 161, "The demo case from AOC should equal 161");
    }

    #[test]
    fn test_demo_input_limited() {
        let line = "xmul(2,4)&mul[3,7]!^don't()_mul(5,5)+mul(32,64](mul(11,8)undo()?mul(8,5))";
        assert_eq!(scan_line_limited(line), 48, "The demo case from AOC should equal 161");
    }

    #[test]
    fn test_input_limited_multiple_donts() {
        let line = "mul(3,1)___don't()don't()don't()mul(4,1)do()mul(5,1)";
        assert_eq!(scan_line_limited(line), 8, "mul(3,1) + mul(5,1) == 8");
    }

    #[test]
    fn test_input_limited_multiple_dos() {
        let line = "mul(3,1)___don't()mul(4,1)do()do()don't()do()mul(5,1)";
        assert_eq!(scan_line_limited(line), 8, "mul(3,1) + mul(5,1) == 8");
    }

    #[test]
    fn test_input_limited_unbounded() {
        let line = "don't()mul(3,1)";
        assert_eq!(scan_line_limited(line), 0, "Should be 0");
    }
}
