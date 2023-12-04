use super::Executor;

use rustc_hash::FxHashMap;

#[derive(Default)]
pub struct Day3 {
    part_costs: FxHashMap<(usize, usize), Vec<u32>>,
    lines: Vec<Vec<char>>,
}

impl Executor for Day3 {
    fn parse(&mut self, input: String) {
        let lines: Vec<Vec<char>> = input.lines().map(|l| l.chars().collect()).collect();
        let mut current_num = None;
        let mut associated_symbol: Option<(usize, usize)> = None;
        let mut part_costs: FxHashMap<(usize, usize), Vec<u32>> = FxHashMap::default();
        for i in 0..lines.len() {
            for j in 0..lines[0].len() {
                if lines[i][j].is_ascii_digit() {
                    let n = current_num.get_or_insert(0);
                    *n *= 10;
                    *n += lines[i][j].to_digit(10).unwrap();

                    let i = i as isize;
                    let j = j as isize;
                    let potential_neighbors = [
                        (i - 1, j - 1),
                        (i, j - 1),
                        (i + 1, j - 1),
                        (i - 1, j),
                        (i + 1, j),
                        (i - 1, j + 1),
                        (i, j + 1),
                        (i + 1, j + 1),
                    ];

                    for (next_i, next_j) in potential_neighbors {
                        let mut bounds_exceeded = false;
                        bounds_exceeded |= next_i < 0 || next_i as usize >= lines[..].len();
                        bounds_exceeded |= next_j < 0 || next_j as usize >= lines[i as usize].len();
                        if bounds_exceeded {
                            continue;
                        }

                        let next_i = next_i as usize;
                        let next_j = next_j as usize;
                        let neighbor = lines[next_i][next_j];
                        if !neighbor.is_ascii_digit() && neighbor != '.' {
                            debug_assert!(
                                associated_symbol.is_none()
                                    || associated_symbol.unwrap() == (next_i, next_j)
                            );
                            associated_symbol = Some((next_i, next_j));
                        }
                    }
                } else {
                    if let (Some(n), Some((i, j))) = (current_num, associated_symbol) {
                        part_costs
                            .entry((i, j))
                            .and_modify(|v| v.push(n))
                            .or_insert(vec![n]);
                    }
                    current_num = None;
                    associated_symbol = None;
                }
            }
        }

        self.part_costs = part_costs;
        self.lines = lines;
    }

    fn part_one(&mut self) {
        println!(
            "P1: {}",
            self.part_costs.values().flat_map(|v| v.iter()).sum::<u32>()
        );
    }

    fn part_two(&mut self) {
        let gear_ratio_sum = self
            .part_costs
            .iter()
            .filter(|(&(i, j), costs)| self.lines[i][j] == '*' && costs.len() == 2)
            .map(|(_, c)| c[0] * c[1])
            .sum::<u32>();

        println!("P2: {}", gear_ratio_sum);
    }
}
