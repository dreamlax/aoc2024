use utils::timer::Timer;
use std::collections::HashSet;
use std::fs::read;
use std::path::PathBuf;

fn count_all_routes(mountain: &[u8], width: usize, current: usize) -> usize {
    let current_value = mountain[current];
    if current_value == b'9' {
        return 1;
    }

    let mut routes = 0;

    if let Some(left) = current.checked_sub(1) {
        if mountain[left] == current_value + 1 {
            routes += count_all_routes(mountain, width, left);
        }
    }

    if let Some(up) = current.checked_sub(width) {
        if mountain[up] == current_value + 1 {
            routes += count_all_routes(mountain, width, up);
        }
    }

    let right = current + 1;
    if right < mountain.len() && mountain[right] == current_value + 1 {
        routes += count_all_routes(mountain, width, right);
    }

    let down = current + width;
    if down < mountain.len() && mountain[down] == current_value + 1 {
        routes += count_all_routes(mountain, width, down);
    }

    routes
}

fn find_summits(mountain: &[u8], width: usize, current: usize, summits: &mut HashSet<usize>) {
    let current_value = mountain[current];
    if current_value == b'9' {
        summits.insert(current);
        return;
    }

    [-1, 1, width as isize, -(width as isize)]
        .iter()
        .filter_map(|next|
            current
                .checked_add_signed(*next)
                .filter(|next| *next < mountain.len() && mountain[*next] == current_value + 1)
        )
        .for_each(|next| find_summits(mountain, width, next, summits));
}

fn main() {
    let _timer = Timer::new();

    let path: PathBuf = std::env::args_os()
        .skip(1)
        .next()
        .expect("Should have file argument")
        .into();

    let mountain = read(path)
        .expect("Should be able to read from path");

    let width = mountain
        .iter()
        .position(|ch| *ch == b'\n')
        .map(|w| w + 1)
        .unwrap_or(mountain.len());

    let answer = if cfg!(feature = "part2") {
        mountain
            .iter()
            .enumerate()
            .filter_map(|(idx, ch)| if *ch == b'0' {
                Some(count_all_routes(&mountain, width, idx))
            } else {
                None
            })
            .sum()
    }
    else {
        mountain
            .iter()
            .enumerate()
            .filter(|(_idx, ch)| **ch == b'0')
            .fold(0usize, |acc, (idx, _ch)| {
                let mut summits = HashSet::new();
                find_summits(&mountain, width, idx, &mut summits);
                acc + summits.len()
            })
    };

    println!("Answer: {answer}");
}
