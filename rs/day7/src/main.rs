use utils::timer::Timer;
use std::fs::read_to_string;
use std::path::PathBuf;

fn can_add_or_mul_to_total(total: i64, stack: &[i64]) -> bool {
    let Some((last, substack)) = stack.split_last() else {
        return total == 0;
    };

    (total >= *last && can_add_or_mul_to_total(total - *last, substack)) ||
    (total % *last == 0) && can_add_or_mul_to_total(total / *last, substack)
}

fn can_add_or_mul_or_concat_to_total(total: i64, stack: &[i64]) -> bool {
    let Some((last, substack)) = stack.split_last() else {
        return total == 0;
    };

    (total >= *last && can_add_or_mul_or_concat_to_total(total - *last, substack)) ||
    (total % *last == 0) && can_add_or_mul_or_concat_to_total(total / *last, substack) ||
    ({
        // concatenating is op1 * 10^(num digits of op2) + op2
        // this can be undone by subtracting op2 and then dividing by 10^(num digits of op2)
        // if (total - op2) is not a multiple of 10^(num digits of op2) then concatenation could not have occurred
        let digits = 10i64.pow(last.ilog10() + 1);
        let next_total = total - *last;
        next_total % digits == 0 && can_add_or_mul_or_concat_to_total(next_total / digits, substack)
    })
}

fn main() {
    let _timer = Timer::new();

    let path: PathBuf = std::env::args_os()
        .nth(1)
        .expect("Should have file argument")
        .into();

    let input = read_to_string(path)
        .expect("Should be able to read from path");

    let function = if cfg!(feature = "part2") {
        can_add_or_mul_or_concat_to_total
    }
    else {
        can_add_or_mul_to_total
    };

    let answer: i64 = input
        .lines()
        .map(|line| line
            .split_once(':')
            .expect("Line is missing colon"))
        .map(|(total, values)| (
            total
                .parse::<i64>()
                .expect("Total is not numeric"),
            values
                .split_ascii_whitespace()
                .map(|v| v.parse::<i64>().expect("Operand is not numeric"))
                .collect::<Vec<i64>>()
        ))
        .filter(|(total, values)| function(*total, values))
        .map(|(total, _)| total)
        .sum();

    println!("Answer: {answer}");
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_1() {
        assert!(can_add_or_mul_to_total(190, &[10, 19]), "should succeed");
    }

    #[test]
    fn test_2() {
        assert!(can_add_or_mul_to_total(3267, &[81, 40, 27]), "should succeed");
    }

    #[test]
    fn test_3() {
        assert!(can_add_or_mul_to_total(292, &[11, 6, 16, 20]), "should succeed");
    }

    #[test]
    fn test_4() {
        assert!(can_add_or_mul_to_total(10, &[2, 5, 1]), "should succeed");
    }

    #[test]
    fn test_5() {
        assert!(can_add_or_mul_to_total(11, &[2, 5, 1]), "should succeed");
    }

    #[test]
    fn test_ng_1() {
        assert!(!can_add_or_mul_to_total(161011, &[16, 10, 13]), "should fail");
    }

    #[test]
    fn test_ng_2() {
        assert!(!can_add_or_mul_to_total(156, &[15, 6]), "should fail");
    }

    #[test]
    fn test_6() {
        assert!(can_add_or_mul_or_concat_to_total(156, &[15, 6, 1]), "should succeed");
    }

    #[test]
    fn test_7() {
        assert!(can_add_or_mul_or_concat_to_total(7290, &[6, 8, 6, 15]), "should succeed");
    }
}
