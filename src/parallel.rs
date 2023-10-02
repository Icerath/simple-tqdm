use crate::{progress_bar, Config};
use indicatif::{ParallelProgressIterator, ProgressBarIter};
use rayon::prelude::*;

/// Wraps a Rayon parallel iterator.
pub trait ParTqdm<I>: Sized {
    /// Wraps a Rayon parallel iterator.
    /// Allows configuration of the progress bar.
    fn tqdm_config(self, _: Config) -> ProgressBarIter<I>;
    /// Wraps a Rayon parallel iterator.
    fn tqdm(self) -> ProgressBarIter<I> {
        self.tqdm_config(Config::default())
    }
}

impl<I: IndexedParallelIterator> ParTqdm<I> for I {
    fn tqdm_config(self, config: Config) -> ProgressBarIter<I> {
        let len = self.len();
        self.progress_with(progress_bar(config, len))
    }
}

/// Equivalent to using the ParTqdm trait method.
pub fn par_tqdm<I: IndexedParallelIterator>(iter: I) -> ProgressBarIter<I> {
    iter.tqdm()
}
