use crate::{progress_bar, Config};
use indicatif::{ParallelProgressIterator, ProgressBarIter};
use rayon::prelude::*;

pub trait ParTqdm<I>: Sized {
    fn tqdm_config(self, _: Config) -> ProgressBarIter<I>;
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

pub fn par_tqdm<I: IndexedParallelIterator>(iter: I) -> ProgressBarIter<I> {
    iter.tqdm()
}
