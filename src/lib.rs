use indicatif::{ProgressIterator, ProgressStyle, ProgressBarIter};

const PBAR_TEMPLATE: &str = "{msg} |{wide_bar}| {pos}/{len} [{elapsed_precise}>{eta_precise}]";

/// A very lazy wrapper for any ProgressIterator with known length.
/// 
/// Looks nice enough as a default.
/// 
/// Style heavily borrowing from [tqdm](https://github.com/tqdm/tqdm).
/// 
/// Example:
/// ```
/// for i in pbar(0..1_000_000, 1_000_000) {
/// // Do whatever
/// }
/// ```
pub fn pbar<I: ProgressIterator>(it: I, len: usize) -> ProgressBarIter<I>{
    let pbar = indicatif::ProgressBar::new(len as u64)
            .with_style(
                ProgressStyle::default_bar()
                    .template(PBAR_TEMPLATE)
            );
    it.progress_with(pbar)
}