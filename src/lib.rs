use indicatif::{ProgressIterator, ProgressStyle, ProgressBarIter};

const PBAR_TEMPLATE: &str = "{msg} |{wide_bar}| {pos}/{len} [{elapsed_precise}>{eta_precise}]";

/// A very lazy wrapper for any ProgressIterator.
/// 
/// Looks nice enough as a default.
/// 
/// Style heavily borrowing from [tqdm](https://github.com/tqdm/tqdm).
/// 
/// Example:
/// ```
/// use lazy_pbar::pbar_sized;
/// 
/// for i in pbar_sized(0..1_000_000, 1_000_000) {
/// // Do whatever
/// }
/// ```
pub fn pbar_sized<I: ProgressIterator>(it: I, len: usize) -> ProgressBarIter<I>{
    let pbar = indicatif::ProgressBar::new(len as u64)
            .with_style(
                ProgressStyle::default_bar()
                    .template(PBAR_TEMPLATE)
            );
    it.progress_with(pbar)
}

/// A very lazy wrapper for a ProgressIterator that is also an ExactSizeIterator.
/// 
/// Looks nice enough as a default.
/// 
/// Style heavily borrowing from [tqdm](https://github.com/tqdm/tqdm).
/// 
/// Example:
/// ```
/// use lazy_pbar::pbar;
/// 
/// for i in pbar(0..1_000_000) {
/// // Do whatever
/// }
/// ```
pub fn pbar<I: ProgressIterator + ExactSizeIterator>(it: I) -> ProgressBarIter<I>{
    let pbar = indicatif::ProgressBar::new(it.len() as u64)
            .with_style(
                ProgressStyle::default_bar()
                    .template(PBAR_TEMPLATE)
            );
    it.progress_with(pbar)
}