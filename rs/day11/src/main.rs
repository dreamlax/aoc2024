use utils::timer::Timer;
use std::collections::HashMap;
use std::fs::read_to_string;
use std::path::PathBuf;

// u32 - NG - must be too small
// u64 - works
// u128 - also works but much slower
type StoneValue = u64;

fn blink(stone: StoneValue) -> (usize, [StoneValue; 2]) {
    if stone == 0 {
        return (1, [1, 0]);
    }
    else {
        let digits = stone.ilog10() + 1;
        if digits % 2 == 0 {
            let divisor = (10 as StoneValue).pow(digits / 2);
            return (2, [stone / divisor, stone % divisor]);
        }
    }

    (1, [stone * 2024, 0])
}

fn count_stones_for_blinks(stone: StoneValue, blinks: usize, cache: &mut HashMap<(usize, StoneValue),usize>) -> usize {
    if let Some(result) = cache.get(&(blinks, stone)) {
        return *result;
    }

    let (count, new_stones) = blink(stone);
    if blinks == 1 {
        cache.insert((blinks, stone), count);
        return count;
    }

    let result = new_stones
        .iter()
        .take(count)
        .map(|stone| count_stones_for_blinks(*stone, blinks - 1, cache))
        .sum();

    cache.insert((blinks, stone), result);

    result
}

fn main() {
    let _timer = Timer::new();

    let path: PathBuf = std::env::args_os()
        .nth(1)
        .expect("Should have file argument")
        .into();

    let stones = read_to_string(path)
        .expect("Should be able to read from path")
        .split_ascii_whitespace()
        .map(|s| s.parse())
        .collect::<Result<Vec<StoneValue>,_>>()
        .expect("Numbers in input should be parsable");

    let mut cache = HashMap::new();
    let mut answer = 0;
    let blinks = if cfg!(feature = "part2") { 75 } else { 25 };

    for stone in stones {
        answer += count_stones_for_blinks(stone, blinks, &mut cache);
    }

    println!("Answer: {answer}");
}
