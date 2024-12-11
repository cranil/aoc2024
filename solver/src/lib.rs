#![allow(dead_code)]

use std::time::{Duration, Instant};

pub struct Stats {
    pub min: f64,
    pub max: f64,
    pub mean: f64,
    pub median: f64,
    pub std_dev: f64,
    pub histogram: Vec<(f64, f64, usize)>,
}

const BOLD: &str = "\x1B[1m";
const UNDERLINE: &str = "\x1B[4m";
const BLINK: &str = "\x1B[5m";
const ITALIC: &str = "\x1B[3m";
const RESET: &str = "\x1B[0m";

fn calculate_stats(durations: &[Duration]) -> Stats {
    let n = durations.len();
    let min = durations.first().unwrap().as_nanos() as f64 - 0.00001;
    let max = durations.last().unwrap().as_nanos() as f64 + 0.00001;
    let mean =
        durations.iter().map(|d| d.as_nanos() as f64).sum::<f64>() / n as f64;
    let median = durations[n / 2].as_nanos() as f64;
    let std_dev = durations
        .iter()
        .map(|d| {
            let diff = d.as_nanos() as f64 - mean;
            diff * diff
        })
        .sum::<f64>()
        / durations.len() as f64;
    let std_dev = std_dev.sqrt();
    let diff = max - min;
    let num_bins = (n as f64).sqrt().ceil() as usize;
    let bounds = (1..=num_bins)
        .map(|i| {
            (
                min + (diff * i as f64 / num_bins as f64),
                min + (diff * (i + 1) as f64 / num_bins as f64),
            )
        })
        .collect::<Vec<_>>();
    let histogram = durations
        .iter()
        .map(|d| {
            bounds
                .iter()
                .position(|&(_, m)| (d.as_nanos() as f64) < m)
                .unwrap_or_else(|| {
                    panic!("Could not find bin: {}", d.as_nanos())
                })
        })
        .fold(vec![0; num_bins], |mut acc, i| {
            acc[i] += 1;
            acc
        });
    let histogram = histogram
        .into_iter()
        .enumerate()
        .map(|(i, count)| {
            let (l, h) = bounds[i];
            (l, h, count)
        })
        .collect::<Vec<_>>();
    Stats {
        min,
        max,
        mean,
        median,
        std_dev,
        histogram,
    }
}

pub fn time_it_n<F: Fn() -> R, R>(f: F, n: usize) -> (R, Stats) {
    let mut durations = Vec::with_capacity(n);
    for _ in 0..n {
        let start = Instant::now();
        f();
        let duration = start.elapsed();
        durations.push(duration);
    }
    durations.sort();
    let stats = calculate_stats(&durations);
    (f(), stats)
}

fn color(r: u8, g: u8, b: u8) -> String {
    format!("\x1B[38;2;{};{};{}m", r, g, b)
}

fn color_map(value: f64) -> String {
    let start = (255.0, 0.0, 0.0);
    let end = (0.0, 255.0, 0.0);
    let r = start.0 + (end.0 - start.0) * value;
    let g = start.1 + (end.1 - start.1) * value;
    let b = start.2 + (end.2 - start.2) * value;

    let r = r.round() as u8;
    let g = g.round() as u8;
    let b = b.round() as u8;

    color(r, g, b)
}

fn histogram_horizontal(histogram: &[(f64, f64, usize)]) -> Vec<String> {
    let hblocks = ['▁', '▂', '▃', '▄', '▅', '▆', '▇', '█'];
    if histogram.len() < 10 {
        return Vec::new();
    }
    let mut lines: Vec<String> = vec![String::new(); 10];
    let histmax = histogram.iter().map(|&(_, _, v)| v).max().unwrap() as f64;
    let l = histogram.first().unwrap().0;
    let h = histogram.last().unwrap().1;
    lines[0].push_str(&format!("{ITALIC}{:>4}{RESET}", histmax));
    lines[1].push_str("    ╭─");
    for _ in 0..histogram.len() + 1 {
        lines[1].push('─');
    }
    lines[1].push_str("─╮");
    for (i, line) in lines.iter_mut().skip(2).rev().enumerate() {
        line.push_str("    │ ┼");
        for (_, _, count) in histogram.iter() {
            let count_print =
                ((64.0 / histmax) * (*count as f64)).ceil() as usize;
            let color = color_map(count_print as f64 / 64.0);
            if count_print > 8 * i {
                line.push_str(&color);
                if count_print - 8 * i >= 8 {
                    line.push(hblocks[7]);
                } else {
                    line.push(hblocks[count_print % 8]);
                }
                line.push_str("\x1B[m");
            } else {
                line.push(' ');
            }
        }
        line.push_str(" │");
    }
    lines.push(String::new());
    let line = lines.last_mut().unwrap();
    line.push_str("    │ ");
    for _ in 0..histogram.len() + 1 {
        line.push('┼');
    }
    line.push_str(" │");
    lines.push(String::new());
    let line = lines.last_mut().unwrap();
    line.push_str("   0╰─");
    for _ in 0..histogram.len() + 1 {
        line.push('─');
    }
    line.push_str("─╯");

    let cols = histogram.len() + 1;
    let smallest = print_time(l);
    let biggest = print_time(h);

    lines.push(format!("    {ITALIC}{}{RESET}", smallest));
    for _ in 0..cols - count_prinable_chars(&smallest) {
        lines.last_mut().unwrap().push(' ');
    }
    lines
        .last_mut()
        .unwrap()
        .push_str(&format!("{ITALIC}{}{RESET}", biggest));
    lines
}

fn print_hhistogram(hist: &[(f64, f64, usize)]) {
    let lines = histogram_horizontal(hist);
    for line in lines.iter() {
        println!("{}", line);
    }
}

fn print_time(time_ns: f64) -> String {
    if time_ns < 1000.0 {
        format!("{:0.2}ns", time_ns)
    } else if time_ns < 1000000.0 {
        format!("{:0.2}μs", time_ns / 1000.0)
    } else if time_ns < 1000000000.0 {
        format!("{:0.2}ms", time_ns / 1000000.0)
    } else {
        format!("{:0.2}s", time_ns / 1000000000.0)
    }
}

fn count_prinable_chars(s: &str) -> usize {
    let mut count = 0;
    let mut escape = false;
    for c in s.chars() {
        match c {
            '\x1b' => {
                escape = true;
            }
            'm' if escape => {
                escape = false;
            }

            _ => {
                if !escape {
                    count += 1;
                }
            }
        }
    }
    count
}

pub fn timer_n<'a, I, F: Fn(&'a I) -> R, R, const N: usize>(
    fs: [F; N],
    input: &'a I,
    n: usize,
) where
    R: std::fmt::Display,
    I: ?Sized, // size is not necessary at compile time
{
    for (count, f) in fs.into_iter().enumerate() {
        let (result, stats) = time_it_n(|| f(input), n);
        let duration = stats.median;
        let hist = stats.histogram;
        print_hhistogram(&hist);
        let to_print = format!(
            "Result {count}: {color_result}{BOLD}{UNDERLINE}{result}{RESET} calculated in {color_duration}{duration}±{range}{RESET} with {n} iterations",
            color_result = color(70, 100, 220),
            color_duration = color(70, 100, 220),
            count = count + 1,
            duration = print_time(duration),
            range = print_time(3.0 * stats.std_dev),
        );

        print!("┌");
        for _ in 0..count_prinable_chars(&to_print) {
            print!("─");
        }
        println!("┐");

        println!("│{to_print}│", to_print = to_print);
        print!("└");
        for _ in 0..count_prinable_chars(&to_print) {
            print!("─");
        }
        println!("┘");
        println!();
    }
}

pub fn solver<'a, I, F: Fn(&'a I) -> R, R, const N: usize>(
    fs: [F; N],
    input: &'a I,
) where
    R: std::fmt::Display,
    I: ?Sized,
{
    let args = std::env::args().collect::<Vec<String>>();
    if args.contains(&"time".to_string()) {
        if args.len() > 2 {
            let n = args[2].parse::<usize>().unwrap();
            timer_n(fs, input, n);
        } else {
            timer_n(fs, input, 100);
        }
    } else {
        for (count, f) in fs.into_iter().enumerate() {
            let result = f(input);
            println!("Result {count}: {result}", count = count + 1);
        }
    }
}
