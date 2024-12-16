use std::fs::read_to_string;
use std::path::PathBuf;
use utils::timer::Timer;

fn is_safe_part_1(report: &[i32]) -> bool {
    let mut diffs = report
        .windows(2)
        .map(|v| v[0] - v[1]);

    diffs
        .clone()
        .all(|d| (1..=3).contains(&d))
    ||
    diffs
        .all(|d| (-3..0).contains(&d))
}

fn is_up(x: i32, y: i32) -> bool {
    let diff = x - y;
    (-3..0).contains(&diff)
}

fn is_down(x: i32, y: i32) -> bool {
    let diff = x - y;
    (1..=3).contains(&diff)
}

fn scan_report<T>(report: &[i32], checker: T) -> bool
    where T: Fn(i32, i32) -> bool {
    let mut scan = [0, 1, 2];
    let mut outlier: Option<usize> = None;

    loop {
        if scan[1] == report.len() {
            return true;
        }
        else if scan[2] == report.len() {
            return checker(report[scan[0]], report[scan[1]]) || outlier.is_none();
        }

        let scan_results = [
            checker(report[scan[0]], report[scan[1]]),
            checker(report[scan[1]], report[scan[2]]),
            checker(report[scan[0]], report[scan[2]]),
        ];

        match scan_results {
            [true, true, _] => {
                // no outliers, check the next three digits
                scan[0] = scan[2];
                scan[1] = scan[0] + 1;
                scan[2] = scan[0] + 2;
            },
            [false, true, true] => {
                // first digit is the outlier
                if outlier.is_some() {
                    return false;
                }
                outlier = Some(scan[0]);
                scan[0] += 1;
                scan[1] += 1;
                scan[2] += 1;
            },
            [_, false, true] => {
                // second digit is the outlier
                if outlier.is_some() {
                    return false;
                }
                outlier = Some(scan[1]);
                scan[1] += 1;
                scan[2] += 1;
            },
            [true, false, false] => {
                // third digit is the outlier
                if outlier.is_some() {
                    return false;
                }
                outlier = Some(scan[2]);
                scan[2] += 1;
            },
            [false, _, false] => {
                // two outliers, cannot be safe
                return false
            }
        }
    }
}

fn is_safe_part_2(report: &[i32]) -> bool {
    assert!(report.len() >= 3, "Need at least three values in the report");

    // filter out the case where the outlier is on the ends
    if is_safe_part_1(&report[1..]) || is_safe_part_1(&report[..report.len()-1]) {
        return true;
    }

    for checker in [is_up, is_down] {
        if scan_report(report, checker) {
            return true;
        }
    }

    false
}


fn main() {
    let _t = Timer::new();
    
    let path: PathBuf = std::env::args_os()
        .nth(1)
        .expect("Should have file argument")
        .into();

    let input = read_to_string(path)
        .expect("Should be able to read from path");

    let reports = input
        .lines()
        .map(|l| l
            .split_ascii_whitespace()
            .map(|v| v.parse::<i32>())
            .collect::<Result<Vec<i32>,_>>()
            .expect("Should be able to parse line")
        )
        .collect::<Vec<Vec<i32>>>();

    let safe = if !cfg!(feature = "part2") {
        reports
            .iter()
            .filter(|r| is_safe_part_1(r))
            .count()
    }
    else {
        reports
            .iter()
            .filter(|r| is_safe_part_2(r))
            .count()
    };

    println!("Answer: {safe}");
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_safe() {
        let report = [1, 2, 3, 4, 5];
        assert!(is_safe_part_1(&report), "Should be safe");
        assert!(is_safe_part_2(&report), "Should be safe");
    }

    #[test]
    fn test_safe_outlier() {
        let safe = is_safe_part_2(&[8, 6, 4, 4, 1]);
        assert!(safe, "Should be safe");
    }

    #[test]
    fn test_safe_outlier_2() {
        let safe = is_safe_part_2(&[1, 50, 2, 3, 4]);
        assert!(safe, "Should be safe");
    }

    #[test]
    fn test_unsafe() {
        let safe = is_safe_part_2(&[9, 7, 6, 2, 1]);
        assert!(!safe, "Should be unsafe");
    }

    #[test]
    fn test_safe_another() {
        let safe = is_safe_part_2(&[3, 1, 2, 3, 4]);
        assert!(safe, "Should be safe");
    }

    #[test]
    fn test_unsafe_7_long() {
        let safe = is_safe_part_2(&[36, 38, 36, 39, 42, 43, 40]);
        assert!(!safe, "Should be unsafe");
    }

    #[test]
    fn test_unsafe_edge_case() {
        let safe = is_safe_part_2(&[29, 28, 27, 25, 26, 25, 22, 20]);
        assert!(safe, "Should be safe");
    }
}
