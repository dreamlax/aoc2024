use utils::timer::Timer;
use std::cmp::Ordering;
use std::collections::HashSet;
use std::fs::read_to_string;
use std::path::PathBuf;

fn main() {
    let _timer = Timer::new();

    let path: PathBuf = std::env::args_os()
        .nth(1)
        .expect("Should have file argument")
        .into();

    let input = read_to_string(path)
        .expect("Should be able to read from path");

    let (rules, mut updates, _) = input
        .lines()
        .fold((HashSet::new(), Vec::new(), false), |(mut rules, mut updates, rules_done), line| {
            if line.is_empty() {
                (rules, updates, true)
            }
            else if !rules_done {
                let Some((left, right)) = line.split_once('|') else {
                    panic!("Cannot parse line!");
                };

                rules.insert((
                    left.parse().expect("Left number not parsable"),
                    right.parse().expect("Right number not parsable")
                ));

                (rules, updates, rules_done)
            }
            else {
                updates.push(
                    line
                        .split(',')
                        .map(str::parse)
                        .collect::<Result<Vec<u32>,_>>()
                        .expect("Numbers should be comma-separated")
                    );

                (rules, updates, rules_done)
            }
        });

    let sum: u32 = if !cfg!(feature = "part2") {
        updates
            .iter()
            .filter(|update| update.is_sorted_by(|&a, &b| rules.contains(&(a, b))))
            .map(|update| update[update.len()/2])
            .sum()
    }
    else {
        updates
            .iter_mut()
            .filter(|update| !update.is_sorted_by(|&a, &b| rules.contains(&(a, b))))
            .map(|update| {
                update.sort_by(|&a, &b|
                    if rules.contains(&(a, b)) {
                        Ordering::Less
                    } else {
                        Ordering::Greater
                    }
                );
                update[update.len()/2]
            })
            .sum()
            
    };

    println!("Answer: {sum}");
}
