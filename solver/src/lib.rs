use std::time::{Duration, Instant};

pub fn time_it_n<F: Fn() -> R, R>(
    f: F,
    n: usize,
) -> (R, Duration, Vec<(f64, f64, usize)>) {
    let mut durations = Vec::with_capacity(n);
    for _ in 0..n {
        let start = Instant::now();
        f();
        let duration = start.elapsed();
        durations.push(duration);
    }
    durations.sort();

    let min_duration = durations.first().unwrap().as_nanos() as f64 - 0.00001;
    let max_duration = durations.last().unwrap().as_nanos() as f64 + 0.00001;
    let diff = max_duration - min_duration;
    let num_bins = (n as f64).sqrt().ceil() as usize;
    let bounds = (1..=num_bins)
        .map(|i| {
            (
                min_duration + (diff * i as f64 / num_bins as f64),
                min_duration + (diff * (i + 1) as f64 / num_bins as f64),
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
    let duration = *durations.get(durations.len() / 2).unwrap();
    (f(), duration, histogram)
}

fn print_vblock(mut block: usize) {
    let vblocks = ['▏', '▎', '▍', '▌', '▋', '▊', '▉', '█'];
    print!(" ");
    while block >= 8 {
        print!("{}", vblocks[7]);
        block -= 8;
    }
    print!("{}", vblocks[block]);
}

fn print_time(time_ns: f64) -> String {
    if time_ns < 1000.0 {
        format!("{: >8.2}ns ", time_ns)
    } else if time_ns < 1000000.0 {
        format!("{: >8.2}us ", time_ns / 1000.0)
    } else if time_ns < 1000000000.0 {
        format!("{: >8.2}ms ", time_ns / 1000000.0)
    } else {
        format!("{: >8.2}s ", time_ns / 1000000000.0)
    }
}

fn print_vhistogram(histogram: &[(f64, f64, usize)]) {
    if histogram.len() < 10 {
        return;
    }
    let l = histogram.first().unwrap().0;
    let h = histogram.last().unwrap().1;
    println!("{}", print_time(l));
    for (_, _, count) in histogram.iter() {
        for _ in 0..14 {
            print!(" ");
        }
        print_vblock(*count);
        println!(" {}", count);
    }
    println!("{}", print_time(h));
}

fn count_prinable_chars(s: &str) -> usize {
    let mut count = 0;
    let mut escape = false;
    for c in s.chars() {
        if escape && c == 'm' {
            escape = false;
            continue;
        }
        if c == '\x1b' {
            escape = true;
            continue;
        }
        if !escape {
            count += 1;
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
        let (result, duration, hist) = time_it_n(|| f(input), n);
        print_vhistogram(&hist);
        let to_print = format!(
            "Result {count}: \x1B[38;2;140;30;220m\x1B[1m{result}\x1b[0m in {duration}ms with {n} iterations",
            count = count + 1,
            duration = duration.as_micros() as f64 / 1000.0,
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
