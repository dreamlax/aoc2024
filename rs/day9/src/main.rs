use utils::timer::Timer;
use std::collections::{BTreeMap, BTreeSet};
use std::fs::read;
use std::path::PathBuf;

const EMPTY: u16 = u16::MAX;

fn expand_disk(compacted_disk: &[u8]) -> Vec<u16> {
    compacted_disk
        .iter()
        .enumerate()
        .filter(|(_, ch)| ch.is_ascii_digit())
        .flat_map(|(idx, ch)| {
            (0..(ch - b'0'))
                .map(move |_| {
                    if idx % 2 == 1 { 
                        EMPTY
                    }
                    else {
                        (idx / 2) as u16
                    }
                })
        })
        .collect::<Vec<u16>>()
}

fn frag(disk: &mut [u16]) {
    let mut i = 0;
    let mut j = disk.len() - 1;

    loop {
        while i < disk.len() && disk[i] != EMPTY {
            i += 1;
        }
        while i < j && disk[j] == EMPTY {
            j -= 1;
        }

        disk.swap(i, j);
        i += 1;
        
        if j == 0 || i >= j {
            break;
        }
        
        j -= 1;
    }
}

fn get_disk_map(compacted_disk: &[u8]) -> (Vec<usize>, BTreeMap<usize, BTreeSet<usize>>) {
    let (file_lengths, free_space, _) = compacted_disk
        .iter()
        .enumerate()
        .filter(|(_, ch)| ch.is_ascii_digit())
        .map(|(idx, &ch)| (idx, (ch - b'0') as usize))
        .fold((Vec::new(), BTreeMap::new(), 0usize), |(mut file_lengths, mut free_space, acc), (idx, len)| {
            if idx % 2 == 1 {
                free_space.entry(len).or_insert(BTreeSet::new()).insert(acc);
            }
            else {
                file_lengths.push(len);
            }
            (file_lengths, free_space, acc + len)
        });
    (file_lengths, free_space)
}

fn defrag(disk: &mut [u16], file_lengths: &[usize], mut free_spaces: BTreeMap<usize, BTreeSet<usize>>) {
    let mut j = disk.len() - 1;

    'outer: loop {
        while disk[j] == EMPTY {
            j -= 1;
            if j == 0 {
                break 'outer;
            }
        }

        let file_id = disk[j];
        let file_length = file_lengths[file_id as usize];

        let (space_length, space_index) = {
            let Some((space_length, space_index)) = free_spaces
                .range(file_length..)
                .map(|(space_size, spaces)| (space_size, spaces.first()))
                .filter(|(_, index)| index.is_some_and(|&f| f < j))
                .map(|(space_length, index)| (space_length, index.unwrap()))
                .min_by_key(|(_, index)| **index) else {
                if j < file_length {
                    break 'outer;
                }
                j -= file_length;
                continue;
            };

            (*space_length, *space_index)
        };

        if space_index > j {
            if j < file_length {
                break 'outer;
            }
            j -= file_length;
            continue;
        }
                
        for i in space_index..space_index+file_length {
            disk.swap(i, j);
            j -= 1;
        }
        
        // bookkeeping
        let space_left = space_length - file_length;
        free_spaces.entry(space_length).and_modify(|indices| { indices.remove(&space_index); });
        if space_left > 0 {
            free_spaces.entry(space_left).or_default().insert(space_index + file_length);
        }
    }
}

fn checksum(disk: &[u16]) -> usize {
    disk
        .iter()
        .enumerate()
        .filter(|(_, &d)| d != EMPTY)
        .map(|(idx, &d)| idx * d as usize)
        .sum()
}

fn main() {
    let _timer = Timer::new();

    let path: PathBuf = std::env::args_os()
        .nth(1)
        .expect("Should have file argument")
        .into();

    let disk = read(path)
        .expect("Should be able to read from path");

    let mut expanded = expand_disk(&disk);
    
    if cfg!(feature = "part2") {
        let (file_lengths, free_space) = get_disk_map(&disk);
        defrag(&mut expanded, &file_lengths, free_space);
    }
    else {
        frag(&mut expanded);
    }

    let answer = checksum(&expanded);

    println!("Answer: {answer}");
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_expand() {
        let example = b"12345";
        let expanded = expand_disk(example);

        assert_eq!(expanded,
            vec![0, EMPTY, EMPTY, 1, 1, 1, EMPTY, EMPTY, EMPTY, EMPTY, 2, 2, 2, 2, 2]);
    }

    #[test]
    fn test_defrag() {
        let example = b"2333133121414131402";
        let mut expanded = expand_disk(example);
        let (file_lengths, free_space) = get_disk_map(example);
        defrag(&mut expanded, &file_lengths, free_space);
        let checksum = checksum(&expanded);
        assert_eq!(checksum, 2858);
    }
}
