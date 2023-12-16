use rayon::prelude::*;
use tinyvec::ArrayVec;

use super::Executor;
use std::fmt::Write;

type SpringBacking = [HotSpringCondition; 128];
type GroupBacking = [u8; 32];

#[derive(Clone, Copy, Debug, PartialEq, Default)]
enum HotSpringCondition {
    #[default]
    Operational,
    Damaged,
    Unknown,
}

impl std::convert::From<char> for HotSpringCondition {
    fn from(c: char) -> Self {
        match c {
            '#' => HotSpringCondition::Damaged,
            '.' => HotSpringCondition::Operational,
            '?' => HotSpringCondition::Unknown,
            _ => unsafe { std::hint::unreachable_unchecked() },
        }
    }
}

#[derive(Debug)]
struct SpringData(ArrayVec<SpringBacking>, ArrayVec<GroupBacking>);
impl SpringData {
    fn parse(input: &str) -> Self {
        let mut spring_vec = ArrayVec::<_>::default();
        let mut group_vec = ArrayVec::<_>::default();

        let mut iter = input.split_whitespace();
        let springs = iter.next().unwrap();
        for c in springs.chars() {
            spring_vec.push(c.into())
        }
        let groups = iter.next().unwrap();
        for g in groups.split(',') {
            group_vec.push(g.parse().unwrap());
        }

        SpringData(spring_vec, group_vec)
    }
}

impl SpringData {
    fn count_possible_arrangements(&self) -> u64 {
        let mut current = [0u64; 128];
        let mut previous = [0u64; 128];
        let Self(springs, groups) = self;

        current[springs.len()] = 1;
        for j in (0..springs.len()).rev() {
            match springs[j] {
                HotSpringCondition::Damaged => {
                    break;
                }
                _ => current[j] = 1,
            }
        }
        let mut gap = 0;
        let mut start_position = springs.len() as isize;
        for i in (0..groups.len()).rev() {
            let group = groups[i];
            if start_position < group as isize {
                break;
            }
            std::mem::swap(&mut current, &mut previous);
            let mut isize_j = start_position - group as isize;
            let mut distance_from_operational = 0;
            current[isize_j as usize + 1] = 0;
            for i in
                isize_j + 1..=std::cmp::min(isize_j + group as isize, springs.len() as isize - 1)
            {
                if springs[i as usize] == HotSpringCondition::Operational {
                    break;
                }
                distance_from_operational += 1;
            }
            let mut has_written_nonzero = false;
            while isize_j >= 0 {
                let j = isize_j as usize;
                distance_from_operational += 1;
                match springs[j] {
                    HotSpringCondition::Operational => {
                        distance_from_operational = 0;
                        current[j] = current[j + 1];
                    }
                    HotSpringCondition::Damaged => {
                        if distance_from_operational >= group
                            && !matches!(
                                springs.get(j + group as usize),
                                Some(HotSpringCondition::Damaged),
                            )
                        {
                            current[j] =
                                previous[std::cmp::min(springs.len(), j + group as usize + gap)];
                        } else {
                            current[j] = 0;
                        }
                    }
                    HotSpringCondition::Unknown => {
                        let damaged_count =
                            previous[std::cmp::min(springs.len(), j + group as usize + gap)];
                        current[j] = current[j + 1];
                        if distance_from_operational >= group
                            && !matches!(
                                springs.get(j + group as usize),
                                Some(HotSpringCondition::Damaged),
                            )
                        {
                            current[j] += damaged_count;
                        }
                    }
                }
                if current[j] != 0 && !has_written_nonzero {
                    has_written_nonzero = true;
                    start_position = isize_j - 1;
                }
                isize_j -= 1;
            }
            gap = 1;
        }
        current[0]
    }
}

#[derive(Default, Debug)]
pub struct Day12 {
    spring_data: Vec<SpringData>,
}

impl Executor for Day12 {
    fn parse(&mut self, input: String) {
        for line in input.lines() {
            self.spring_data.push(SpringData::parse(line));
        }
    }

    fn part_one(&mut self, output_buffer: &mut dyn Write) {
        let out = self
            .spring_data
            .par_iter()
            .map(SpringData::count_possible_arrangements)
            .sum::<u64>();
        _ = write!(output_buffer, "P1: {out}");
    }

    fn part_two(&mut self, output_buffer: &mut dyn Write) {
        self.spring_data
            .par_iter_mut()
            .for_each(|SpringData(springs, groups)| {
                let original_springs_size = springs.len();
                let original_groups_size = groups.len();
                for _ in 0..4 {
                    springs.push(HotSpringCondition::Unknown);
                    for s in 0..original_springs_size {
                        springs.push(springs[s]);
                    }
                    for g in 0..original_groups_size {
                        groups.push(groups[g]);
                    }
                }
            });
        let out = self
            .spring_data
            .par_iter()
            .map(SpringData::count_possible_arrangements)
            .sum::<u64>();
        _ = write!(output_buffer, "P2: {out}");
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use tinyvec::array_vec;
    #[test]
    fn cardinality_no_values() {
        let spring_data = SpringData(array_vec![], array_vec![]);
        assert_eq!(spring_data.count_possible_arrangements(), 1);
    }

    #[test]
    fn cardinality_no_unknowns() {
        let spring_data = SpringData::parse("# 1");
        assert_eq!(spring_data.count_possible_arrangements(), 1);
    }
    #[test]
    fn cardinality_one_unknown() {
        let spring_data = SpringData::parse("?# 2");
        assert_eq!(spring_data.count_possible_arrangements(), 1);
    }

    #[test]
    fn cardinality_multiple_unknowns_simple() {
        let spring_data = SpringData::parse("??.?.? 1,1,1");
        assert_eq!(spring_data.count_possible_arrangements(), 2);
    }

    #[test]
    fn cardinality_unknowns_preceding_damaged() {
        let spring_data = SpringData::parse("?.# 1,1");
        assert_eq!(spring_data.count_possible_arrangements(), 1);
    }

    #[test]
    fn cardinality_unknowns_following_damaged() {
        let spring_data = SpringData::parse("#?? 1,1");
        assert_eq!(spring_data.count_possible_arrangements(), 1);
    }

    #[test]
    fn cardinality_corner_case_1() {
        let spring_data = SpringData::parse("???? 1,1");
        assert_eq!(spring_data.count_possible_arrangements(), 3);
    }

    #[test]
    fn cardinality_corner_case_2() {
        let spring_data = SpringData::parse("????.?? 2,2");
        assert_eq!(spring_data.count_possible_arrangements(), 3);
    }

    #[test]
    fn p1_sample_input() {
        let spring_data = SpringData::parse("???.### 1,1,3");
        assert_eq!(spring_data.count_possible_arrangements(), 1);

        let spring_data = SpringData::parse(".??..??...?##. 1,1,3");
        assert_eq!(spring_data.count_possible_arrangements(), 4);

        let spring_data = SpringData::parse("?#?#?#?#?#?#?#? 1,3,1,6");
        assert_eq!(spring_data.count_possible_arrangements(), 1);

        let spring_data = SpringData::parse("????.#...#... 4,1,1");
        assert_eq!(spring_data.count_possible_arrangements(), 1);

        let spring_data = SpringData::parse("????.######..#####. 1,6,5");
        assert_eq!(spring_data.count_possible_arrangements(), 4);

        let spring_data = SpringData::parse("?###???????? 3,2,1");
        assert_eq!(spring_data.count_possible_arrangements(), 10);
    }
}
