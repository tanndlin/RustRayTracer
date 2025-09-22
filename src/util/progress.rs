use rayon::prelude::{IndexedParallelIterator, IntoParallelIterator, ParallelIterator};
use std::collections::VecDeque;
use std::io::Write;
use std::sync::atomic::{AtomicUsize, Ordering};
use std::sync::{Arc, Mutex};
use std::time::{Duration, Instant};

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

    // Keep a rolling window of (timestamp, count) samples
    let history = Arc::new(Mutex::new(VecDeque::<(Instant, usize)>::new()));
    let window = Duration::from_secs(3);

    par_iter.map(move |item| {
        let i = counter.fetch_add(1, Ordering::SeqCst) + 1;
        let now = Instant::now();

        {
            let mut h = history.lock().unwrap();
            h.push_back((now, i));

            // Drop old samples outside the window
            while let Some(&(t, _)) = h.front() {
                if now.duration_since(t) > window {
                    h.pop_front();
                } else {
                    break;
                }
            }
        }

        let elapsed = start.elapsed().as_secs_f64();
        let (rate, eta) = {
            let h = history.lock().unwrap();
            if let (Some(&(old_t, old_count)), Some(&(new_t, new_count))) = (h.front(), h.back()) {
                let dt = new_t.duration_since(old_t).as_secs_f64();
                let dn = (new_count - old_count) as f64;
                if dt > 0.0 {
                    let rate = dn / dt;
                    let remaining = (total - i) as f64;
                    let eta = if rate > 0.0 { remaining / rate } else { 0.0 };
                    (rate, eta)
                } else {
                    (0.0, 0.0)
                }
            } else {
                (0.0, 0.0)
            }
        };

        let percent = if total > 0 {
            i as f64 / total as f64 * 100.0
        } else {
            0.0
        };
        let bar_width = ((i as f64 / total as f64) * max_width as f64).round() as usize;
        let bar = format!("[{:<width$}]", "=".repeat(bar_width), width = max_width);

        print!(
            "\r{} {:>6.2}% | Elapsed: {:>6.1}s | Rate: {:>6.1} it/s | ETA: {:>6.1}s ",
            bar, percent, elapsed, rate, eta
        );
        std::io::stdout().flush().unwrap();

        item
    })
}
