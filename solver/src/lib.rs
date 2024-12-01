use std::time::{Duration, Instant};

pub fn time_it_n<F: Fn() -> R, R>(f: F, n: usize) -> (R, Duration) {
    let start = Instant::now();
    let mut durations = Vec::with_capacity(n);
    for _ in 0..n {
        f();
        let duration = start.elapsed();
        durations.push(duration);
    }
    let duration = durations.iter().sum::<Duration>() / n as u32;
    (f(), duration)
}
pub fn timer_n<'a, F: Fn(&'a str) -> R, R, const N: usize>(fs: [F; N], input: &'a str, n: usize)
where
    R: std::fmt::Display,
{
    for (count, f) in fs.into_iter().enumerate() {
        let (result, duration) = time_it_n(|| f(input), n);
        println!(
            "Result {count}: {result} in {duration}ms",
            count = count + 1,
            duration = duration.as_millis()
        );
    }
}

pub fn solver<'a, F: Fn(&'a str) -> R, R, const N: usize>(fs: [F; N], input: &'a str)
where
    R: std::fmt::Display,
{
    let args = std::env::args().collect::<Vec<String>>();
    if args.contains(&"time".to_string()) {
        timer_n(fs, input, 50);
    } else {
        for (count, f) in fs.into_iter().enumerate() {
            let result = f(input);
            println!("Result {count}: {result}", count = count + 1);
        }
    }
}
