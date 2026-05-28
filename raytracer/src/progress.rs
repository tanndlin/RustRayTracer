use indicatif::{ProgressBar, ProgressStyle};

pub fn make_progress_bar(total: u64) -> ProgressBar {
    let pb = ProgressBar::new(total);
    pb.set_style(
        ProgressStyle::default_bar()
            .template("{bar:40} {pos}/{len} | Elapsed: {elapsed} | ETA: {eta} | {per_sec}")
            .unwrap(),
    );
    pb
}
