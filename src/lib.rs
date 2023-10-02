mod config;

pub use config::Config;
use config::Number;
use indicatif::{ProgressBar, ProgressBarIter, ProgressDrawTarget, ProgressState, ProgressStyle};
use std::{borrow::Cow, fmt::Write};

#[cfg(feature = "rayon")]
mod parallel;
#[cfg(feature = "rayon")]
pub use parallel::*;

pub trait Tqdm<I>: Sized {
    fn tqdm_config(self, config: Config) -> ProgressBarIter<I>;
    fn tqdm(self) -> ProgressBarIter<I> {
        self.tqdm_config(Config::default())
    }
}

impl<I: ExactSizeIterator> Tqdm<I> for I {
    fn tqdm_config(self, config: Config) -> ProgressBarIter<I> {
        progress_bar(config, self.len()).wrap_iter(self)
    }
}

pub const PROGRESS_CHARS: &str = "█▉▊▋▌▍▎▏ ";

pub fn tqdm<I: Tqdm<I>>(iter: I) -> ProgressBarIter<I> {
    iter.tqdm()
}

fn progress_bar(config: Config, iter_len: usize) -> ProgressBar {
    let len = config.total.unwrap_or(iter_len as u64);
    let bar = ProgressBar::new(len)
        .with_finish(config.progress_finish())
        .with_prefix(config.desc)
        .with_style(style(config.unit, config.unit_scale, config.postfix));
    if config.disable {
        bar.set_draw_target(ProgressDrawTarget::hidden());
    }
    bar
}

fn style(unit: Cow<'static, str>, unit_scale: Number, postfix: Cow<'static, str>) -> ProgressStyle {
    ProgressStyle::with_template(
        "{prefix}{percent}|{wide_bar}| {pos}/{len} [{elapsed}<{eta}, {per_sec}{postfix}]",
    )
    .unwrap()
    .with_key(
        "per_sec",
        move |state: &ProgressState, w: &mut dyn Write| {
            let _ = write!(w, "{:.2}{}/s", unit_scale * state.per_sec(), unit);
        },
    )
    .with_key("percent", |state: &ProgressState, w: &mut dyn Write| {
        let _ = write!(w, "{: >3}%", (state.fraction() * 100.0) as i32);
    })
    .with_key("elapsed", |state: &ProgressState, w: &mut dyn Write| {
        let duration = state.elapsed();
        let minutes = duration.as_secs() / 60;
        let seconds = duration.as_secs() % 60;
        let _ = write!(w, "{minutes:0>2}:{seconds:0>2}");
    })
    .with_key("eta", |state: &ProgressState, w: &mut dyn Write| {
        let duration = state.eta();
        let minutes = duration.as_secs() / 60;
        let seconds = duration.as_secs() % 60;
        let _ = write!(w, "{minutes:0>2}:{seconds:0>2}");
    })
    .with_key("pos", move |state: &ProgressState, w: &mut dyn Write| {
        let _ = write!(w, "{:?}", unit_scale * state.pos());
    })
    .with_key("len", move |state: &ProgressState, w: &mut dyn Write| {
        let _ = write!(w, "{:?}", unit_scale * state.len().unwrap_or(state.pos()));
    })
    .with_key("postfix", move |_: &ProgressState, w: &mut dyn Write| {
        let _ = write!(w, "{}", postfix);
    })
    .progress_chars(PROGRESS_CHARS)
}
