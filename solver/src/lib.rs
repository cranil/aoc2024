use std::time::{Duration, Instant};

pub fn time_it_n<F: Fn() -> R, R>(f: F, n: usize) -> (R, Duration) {
    let mut durations = Vec::with_capacity(n);
    for _ in 0..n {
        let start = Instant::now();
        f();
        let duration = start.elapsed();
        durations.push(duration);
    }
    durations.sort();
    let duration = *durations.get(durations.len() / 2).unwrap();
    (f(), duration)
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
        let (result, duration) = time_it_n(|| f(input), n);
        println!(
            "Result {count}: {result} in {duration}ms with {n} iterations",
            count = count + 1,
            duration = duration.as_millis(),
        );
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
