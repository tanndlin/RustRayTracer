use rayon::prelude::{IndexedParallelIterator, IntoParallelIterator, ParallelIterator};
use std::io::Write;
use std::sync::Arc;
use std::sync::atomic::{AtomicUsize, Ordering};
use std::time::Instant;

pub fn progress_bar<I>(iterable: I) -> impl ParallelIterator<Item = I::Item>
where
    I: IntoParallelIterator,
    I::Iter: IndexedParallelIterator,
{
    let par_iter = iterable.into_par_iter();
    let total = par_iter.len();
    let counter = Arc::new(AtomicUsize::new(0));
    let start = Instant::now();
    let max_width = 40;

    par_iter.map(move |item| {
        let i = counter.fetch_add(1, Ordering::SeqCst);
        let elapsed = start.elapsed().as_secs_f64();
        let rate = if elapsed > 0.0 {
            (i + 1) as f64 / elapsed
        } else {
            0.0
        };
        let percent = if total > 0 {
            (i + 1) as f64 / total as f64 * 100.0
        } else {
            0.0
        };
        let eta = if rate > 0.0 {
            (total - (i + 1)) as f64 / rate
        } else {
            0.0
        };
        let bar_width = ((i + 1) as f64 / total as f64 * max_width as f64).round() as usize;
        let bar = format!("[{:<width$}]", "=".repeat(bar_width), width = max_width);
        print!(
            "\r{} {:>6.2}% | Elapsed: {:>6.1}s | Rate: {:>6.1} it/s | ETA: {:>6.1}s ",
            bar, percent, elapsed, rate, eta
        );
        std::io::stdout().flush().unwrap();
        item
    })
}
