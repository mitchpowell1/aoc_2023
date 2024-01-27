use nom::{
    bytes::complete::{tag, take_until},
    character::complete::{alpha1, one_of},
    combinator::{opt, rest},
    multi::fold_many0,
    sequence::{pair, preceded, terminated, tuple},
    IResult,
};
use rustc_hash::FxHashMap;
use tinyvec::ArrayVec;

use super::Executor;
use std::collections::VecDeque;
use std::fmt::Write;

#[derive(Debug, Default)]
struct Arena {
    modules: Vec<Module>,
    signal_queue: VecDeque<Signal>,
    broadcaster: usize,
    terminal: usize,
    module_high_pulse_watcher: Option<usize>,
}

impl Arena {
    fn populate(&mut self) {
        for i in 0..self.modules.len() {
            let outputs = self.modules[i].get_outputs();
            for o in outputs {
                if let Some(Module::Conjunction(Conjunction { inputs, .. })) =
                    self.modules.get_mut(o)
                {
                    *inputs |= 1 << i;
                }
            }
        }
    }

    fn reset(&mut self) {
        for module in &mut self.modules {
            match module {
                Module::Conjunction(Conjunction { input_signals, .. }) => {
                    *input_signals = 0;
                }
                Module::FlipFlop(FlipFlop { state, .. }) => *state = FlipFlopState::Off,
                // No reset required for untyped modules
                _ => {}
            }
        }
        self.module_high_pulse_watcher = None;
        self.signal_queue.clear();
    }

    fn press_button(&mut self) -> (u32, u32, bool) {
        let Self {
            modules,
            signal_queue,
            broadcaster,
            ..
        } = self;

        signal_queue.push_back(Signal {
            origin: *broadcaster,
            destination: *broadcaster,
            pulse: Pulse::Low,
        });
        let mut low_count = 0;
        let mut high_count = 0;
        let mut watched_module_pulsed_high = false;
        while let Some(signal) = signal_queue.pop_front() {
            match signal.pulse {
                Pulse::High => {
                    if let Some(m) = self.module_high_pulse_watcher {
                        if m == signal.origin {
                            watched_module_pulsed_high = true;
                        }
                    }
                    high_count += 1
                }
                Pulse::Low => low_count += 1,
            }
            let outputs = modules[signal.destination].process_signal(signal);
            for s in outputs.into_iter().flatten() {
                signal_queue.push_back(s);
            }
        }
        (high_count, low_count, watched_module_pulsed_high)
    }
}

#[derive(Debug, Clone)]
enum Pulse {
    High,
    Low,
}

#[derive(Debug, Clone)]
struct Signal {
    pulse: Pulse,
    origin: usize,
    destination: usize,
}

#[derive(Debug, Copy, Clone, PartialEq, Eq)]
enum FlipFlopState {
    On,
    Off,
}

#[derive(Debug, Clone)]
struct FlipFlop {
    state: FlipFlopState,
    outputs: ArrayVec<[usize; 8]>,
}

#[derive(Debug, Clone)]
struct Conjunction {
    inputs: u64,
    input_signals: u64,
    outputs: ArrayVec<[usize; 8]>,
}

#[derive(Debug, Clone)]
struct Untyped {
    outputs: ArrayVec<[usize; 8]>,
}

#[derive(Debug, Clone)]
enum Module {
    FlipFlop(FlipFlop),
    Conjunction(Conjunction),
    Untyped(Untyped),
}

impl Module {
    fn process_signal(&mut self, signal: Signal) -> [Option<Signal>; 8] {
        let mut out = std::array::from_fn(|_| None);
        match self {
            Module::FlipFlop(FlipFlop { state, outputs }) => match signal {
                Signal {
                    pulse: Pulse::Low, ..
                } => {
                    if matches!(state, FlipFlopState::On) {
                        *state = FlipFlopState::Off;
                        for i in 0..outputs.len() {
                            out[i] = Some(Signal {
                                destination: outputs[i],
                                origin: signal.destination,
                                pulse: Pulse::Low,
                            })
                        }
                    } else {
                        *state = FlipFlopState::On;
                        for i in 0..outputs.len() {
                            out[i] = Some(Signal {
                                destination: outputs[i],
                                origin: signal.destination,
                                pulse: Pulse::High,
                            })
                        }
                    }
                }
                // If we receive a high pulse do nothing
                _ => {}
            },
            // Conjunction modules (prefix &) remember the type of the most recent pulse received from each of their
            // connected input modules; they initially default to remembering a low pulse for each input.
            // When a pulse is received, the conjunction module first updates its memory for that input.
            // Then, if it remembers high pulses for all inputs, it sends a low pulse; otherwise, it sends a high pulse.
            Module::Conjunction(Conjunction {
                outputs,
                inputs,
                input_signals,
            }) => {
                let Signal { origin, pulse, .. } = signal;
                match pulse {
                    Pulse::Low => *input_signals &= !(1 << origin),
                    Pulse::High => *input_signals |= 1 << origin,
                };
                let output_pulse = if inputs == input_signals {
                    Pulse::Low
                } else {
                    Pulse::High
                };

                for i in 0..outputs.len() {
                    out[i] = Some(Signal {
                        destination: outputs[i],
                        origin: signal.destination,
                        pulse: output_pulse.clone(),
                    })
                }
            }
            Module::Untyped(Untyped { outputs }) => {
                for i in 0..outputs.len() {
                    out[i] = Some(Signal {
                        destination: outputs[i],
                        origin: signal.destination,
                        pulse: signal.pulse.clone(),
                    })
                }
            }
        };
        out
    }
}

impl Module {
    fn parse<'a>(
        input: &'a str,
        labels: &'a FxHashMap<&'a str, usize>,
        num_modules: usize,
    ) -> IResult<&'a str, Module> {
        let (input, (module_type_char, raw_outputs)) = tuple((
            opt(one_of("%&")),
            preceded(pair(take_until("-> "), tag("-> ")), rest),
        ))(input)?;

        let (_, outputs) = fold_many0(
            terminated(alpha1::<&str, _>, opt(tag(", "))),
            ArrayVec::new,
            |mut acc, v: &str| {
                acc.push(*labels.get(v).unwrap_or(&num_modules));
                acc
            },
        )(raw_outputs)?;

        let module = match module_type_char {
            Some('&') => Module::Conjunction(Conjunction {
                inputs: 0,
                input_signals: 0,
                outputs,
            }),
            Some('%') => Module::FlipFlop(FlipFlop {
                state: FlipFlopState::Off,
                outputs,
            }),
            _ => Module::Untyped(Untyped { outputs }),
        };
        Ok((input, module))
    }

    fn get_outputs(&self) -> ArrayVec<[usize; 8]> {
        match self {
            Module::Untyped(Untyped { outputs })
            | Module::FlipFlop(FlipFlop { outputs, .. })
            | Module::Conjunction(Conjunction { outputs, .. }) => outputs.clone(),
        }
    }
}

#[derive(Default)]
pub struct Day20 {
    arena: Arena,
}

impl Executor for Day20 {
    fn parse(&mut self, input: String) {
        let mut labels = FxHashMap::default();
        let mut num_modules = 0;
        for (i, line) in input.lines().enumerate() {
            let label = line.split_whitespace().next().unwrap();
            labels.insert(label.trim_start_matches(['%', '&']), i);
            num_modules += 1;
        }
        for line in input.lines() {
            self.arena
                .modules
                .push(Module::parse(line, &labels, num_modules).unwrap().1);
        }
        // Push a sink state for any unused modules
        self.arena.modules.push(Module::Untyped(Untyped {
            outputs: ArrayVec::new(),
        }));
        self.arena.broadcaster = labels["broadcaster"];
        self.arena.terminal = self.arena.modules.len() - 1;
        self.arena.populate();
    }

    fn part_one(&mut self, output_buffer: &mut dyn Write) {
        let mut low_total = 0;
        let mut high_total = 0;
        for _ in 0..1000 {
            let (low, high, _) = self.arena.press_button();
            low_total += low;
            high_total += high;
        }
        let total = low_total * high_total;
        _ = write!(output_buffer, "P1: {total}");
    }

    fn part_two(&mut self, output_buffer: &mut dyn Write) {
        let terminal_lead = &self
            .arena
            .modules
            .iter()
            .position(|m| m.get_outputs().contains(&self.arena.terminal))
            .unwrap();
        let lead_inputs = &self
            .arena
            .modules
            .iter_mut()
            .enumerate()
            .filter(|(_i, m)| m.get_outputs().contains(terminal_lead))
            .map(|(i, _m)| i)
            .collect::<Vec<_>>();
        let mut frequencies = lead_inputs.iter().map(|_| 0u64).collect::<Vec<_>>();
        for i in 0..lead_inputs.len() {
            self.arena.reset();
            let mut count = 0;
            let input = lead_inputs[i];
            self.arena.module_high_pulse_watcher = Some(input);
            frequencies[i] = loop {
                let (_, _, pulsed_high) = self.arena.press_button();
                count += 1;
                if pulsed_high {
                    break count;
                }
            };
        }
        let out = frequencies
            .into_iter()
            .map(u128::from)
            .reduce(num::integer::lcm)
            .unwrap();
        _ = write!(output_buffer, "P2 {out}");
    }
}
