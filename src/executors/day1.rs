use super::Executor;

const DIGITS: [&str; 9] = [
    "one", "two", "three", "four", "five", "six", "seven", "eight", "nine",
];

#[derive(Default)]
pub struct Day1 {
    input: String,
}
impl Executor for Day1 {
    fn parse(&mut self, input: String) {
        self.input = input
    }

    fn part_one(&mut self) {
        let mut total = 0;
        for line in self.input.lines() {
            let mut last_digit = None;
            for c in line.chars() {
                if let Some(d) = c.to_digit(10) {
                    if last_digit.is_none() {
                        total += d * 10
                    }
                    last_digit = Some(d);
                }
            }
            total += last_digit.unwrap()
        }

        println!("P1: {total}");
    }

    fn part_two(&mut self) {
        let mut total = 0;
        for line in self.input.lines() {
            let mut last_digit = None;
            for (i, c) in line.chars().enumerate() {
                if let Some(d) = get_digit(c, &line[i..]) {
                    if last_digit.is_none() {
                        total += d * 10;
                    }
                    last_digit = Some(d);
                }
            }
            total += last_digit.unwrap()
        }
        println!("P2: {total}");
    }
}

fn get_digit(c: char, line: &str) -> Option<u32> {
    c.to_digit(10).or_else(|| {
        DIGITS
            .iter()
            .position(|&w| line.starts_with(w))
            .and_then(|i| Some(i as u32 + 1))
    })
}
