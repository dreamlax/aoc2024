use utils::timer::Timer;
use std::fs::read;
use std::path::PathBuf;

struct Farm<'a> {
    map: &'a [u8],
    width: usize
}

const EMPTY: u16 = 0;

const NEIGHBOUR_UP: u16 = 0b0001;
const NEIGHBOUR_RIGHT: u16 = 0b0010;
const NEIGHBOUR_DOWN: u16 = 0b0100;
const NEIGHBOUR_LEFT: u16 = 0b1000;

#[derive(Clone,Debug)]
struct Cell {
    id: u16,
    neighbours: u16,
}

impl Default for Cell {
    fn default() -> Self {
        Self {
            id: EMPTY,
            neighbours: 0
        }
    }
}

impl<'a> From<&'a [u8]> for Farm<'a> {
    fn from(map: &'a [u8]) -> Self {
        let width = map
            .iter()
            .position(|ch| *ch == b'\n')
            .map(|w| w + 1)
            .unwrap_or(map.len());

        Farm {
            map,
            width
        }
    }
}

impl<'a> Farm<'a> {
    fn find_plot(&self, current_pos: usize, id: u16, analysis: &mut [Cell]) {
        let plot_name = self.map[current_pos];
        analysis[current_pos].id = id;
        analysis[current_pos].neighbours =
            [
                (NEIGHBOUR_LEFT, -1), 
                (NEIGHBOUR_RIGHT, 1),
                (NEIGHBOUR_DOWN, self.width as isize),
                (NEIGHBOUR_UP, -(self.width as isize))
            ]
            .iter()
            .filter_map(|next|
                current_pos
                    .checked_add_signed(next.1)
                    .filter(|next_pos| *next_pos < self.map.len() && self.map[*next_pos] == plot_name)
                    .map(|next_pos| (next.0, next_pos))
            )
            .fold(0u16, |acc, (neighbour_bitmask, next_pos)| {
                if analysis[next_pos].id == EMPTY {
                    self.find_plot(next_pos, id, analysis);
                }
                
                if analysis[next_pos].id == id {
                    acc | neighbour_bitmask
                }
                else {
                    acc
                }
            });
    }
    
    fn identify_plots(&self, analysis: &mut [Cell]) -> u16 {
        let mut i = 0;
        let mut id = 0;
        while i < self.map.len() {
            if analysis[i].id != EMPTY || !self.map[i].is_ascii_alphabetic() {
                // already analysed
                i += 1;
                continue;
            }

            id += 1;
            self.find_plot(i, id, analysis);
        }

        id
    }

    fn count_fence_runs(&self, analysis: &[Cell], fence_runs: &mut [usize], area_tally: &mut [usize]) {
        // go from left to right, looking for fence runs
        // the '\n' character in the stream actually helps here since it
        // automatically terminates our run-length detection
        for x in 0..analysis.len() - 1 {
            if analysis[x].id == EMPTY {
                continue;
            }

            area_tally[analysis[x].id as usize] += 1;

            let this_fence_up = analysis[x].neighbours & NEIGHBOUR_UP == 0;
            let next_fence_up = analysis[x+1].neighbours & NEIGHBOUR_UP == 0;
            if this_fence_up && !(analysis[x].id == analysis[x+1].id && next_fence_up) {
                fence_runs[analysis[x].id as usize] += 1;
            }

            let this_fence_down = analysis[x].neighbours & NEIGHBOUR_DOWN == 0;
            let next_fence_down = analysis[x+1].neighbours & NEIGHBOUR_DOWN == 0;
            if this_fence_down && !(analysis[x].id == analysis[x+1].id && next_fence_down) {
                fence_runs[analysis[x].id as usize] += 1;
            }
        }

        // finding runs downwards is a bit trickier because we don't have the
        // additional buffer, so we have an edge case we need to take into
        // account
        for x in 0..self.width {
            for y in (x..analysis.len()).step_by(self.width) {
                if analysis[y].id == EMPTY {
                    continue;
                }

                let this_fence_left = analysis[y].neighbours & NEIGHBOUR_LEFT == 0;
                let this_fence_right = analysis[y].neighbours & NEIGHBOUR_RIGHT == 0;

                let (next_fence_left, next_fence_right) = {
                    let next_y = y + self.width;
                    if next_y >= analysis.len() || analysis[y].id != analysis[next_y].id {
                        (false, false)
                    } else {
                        (analysis[next_y].neighbours & NEIGHBOUR_LEFT == 0, analysis[next_y].neighbours & NEIGHBOUR_RIGHT == 0)
                    }
                };

                if this_fence_left && !next_fence_left { 
                    fence_runs[analysis[y].id as usize] += 1;
                }
                if this_fence_right && !next_fence_right {
                    fence_runs[analysis[y].id as usize] += 1;
                }
            }
        }
    }
}

fn main() {
    let _timer = Timer::new();

    let path: PathBuf = std::env::args_os()
        .nth(1)
        .expect("Should have file argument")
        .into();

    let data = read(path)
        .expect("Should be able to read from input file");
    
    let farm: Farm = data.as_slice().into();

    let mut analysis = vec![Cell::default(); farm.map.len()];
    let max_id = farm.identify_plots(&mut analysis);

    let answer: usize = if cfg!(feature = "part2") {
        let mut fence_runs = vec![0usize; max_id as usize + 1];
        let mut area_tally = vec![0usize; max_id as usize + 1];

        farm.count_fence_runs(&analysis, &mut fence_runs, &mut area_tally);

        area_tally
            .iter()
            .zip(fence_runs.iter())
            .map(|(area, fence_runs)| area * fence_runs)
            .sum()
    }
    else {
        let mut fence_tally = vec![0usize; max_id as usize + 1];
        let mut area_tally = vec![0usize; max_id as usize + 1];
        for cell in &analysis {
            if cell.id == EMPTY {
                continue;
            }
            fence_tally[cell.id as usize] += (4 - cell.neighbours.count_ones()) as usize;
            area_tally[cell.id as usize] += 1;
        }

        area_tally
            .iter()
            .zip(fence_tally.iter())
            .map(|(area, fence)| area * fence)
            .sum()
    };
    
    println!("Answer: {answer}");
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_fence_run() {
        let x = Farm::from(b"AAAAA\nBBBBB\n".as_slice());
        let mut analysis = vec![Cell::default(); x.map.len()];
        let max_id = x.identify_plots(&mut analysis);
        let mut field_runs = vec![0usize; max_id as usize + 1];
        let mut area_tally = vec![0usize; max_id as usize + 1];

        x.count_fence_runs(&analysis, &mut field_runs, &mut area_tally);

        assert_eq!(field_runs[1], 4);
        assert_eq!(field_runs[2], 4);
        assert_eq!(area_tally[1], 5);
        assert_eq!(area_tally[2], 5);
    }
}
