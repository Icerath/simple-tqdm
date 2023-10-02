//! simple-tqdm is a small wrapper around indicatif that tries to be similar to python's [`tqdm`](https://pypi.org/project/tqdm/) library.
//!
//! tqdm comes with both a `tqdm` function and a
//!`Tqdm` trait depending on your preference.
//!
//! # Example
//! ```
//! use simple_tqdm::tqdm;
//!
//! for _ in tqdm(0..2 << 24) {}
//! ```
// TODO - gif`
//! Or if you'd like to customize the progress bar:
//!
//! ```
//! use simple_tqdm::{Tqdm, Config};
//!
//! let config = Config::new().with_unit("num");
//! for _ in (0..2 << 24).tqdm_config(config) {}
//! ```
//! <img src="https://github.com/Icerath/simple-tqdm/blob/main/screenshots/large.gif?raw=true">

//! # Parallel Iterators
//! tqdm also has optional support for parallel
//! iterators with [Rayon](https://github.com/rayon-rs/rayon). In your
//! `Cargo.toml`, use the "rayon" feature:
//!
//! ```toml
//! [dependencies]
//! simple-tqdm = {version = "*", features = ["rayon"]}
//! ```
//! And then use it like this:
//!
//! ```
//! use simple_tqdm::ParTqdm;
//! use rayon::prelude::*;
//!
//! let vec: Vec<_> = (0..100000).into_par_iter().tqdm().map(|i| i + 1).collect();
//! assert_eq!(vec[0], 1);
//! ```

mod config;

pub use config::Config;
use indicatif::{ProgressBar, ProgressBarIter, ProgressDrawTarget, ProgressState, ProgressStyle};
use std::{borrow::Cow, fmt::Write};

#[cfg(feature = "rayon")]
mod parallel;
#[cfg(feature = "rayon")]
pub use parallel::{par_tqdm, ParTqdm};

/// Wraps an iterator to display it's progress.
pub trait Tqdm<I>: Sized {
    /// Wraps an iterator to display it's progress.
    /// Allows configuration of the progress bar.
    fn tqdm_config(self, config: Config) -> ProgressBarIter<I>;
    /// Wraps an iterator to display it's progress.
    fn tqdm(self) -> ProgressBarIter<I> {
        self.tqdm_config(Config::default())
    }
}

impl<I: ExactSizeIterator> Tqdm<I> for I {
    fn tqdm_config(self, config: Config) -> ProgressBarIter<I> {
        progress_bar(config, self.len()).wrap_iter(self)
    }
}

/// Equivalent to using the Tqdm trait method.
pub fn tqdm<I: ExactSizeIterator>(iter: I) -> ProgressBarIter<I> {
    iter.tqdm()
}

fn progress_bar(config: Config, iter_len: usize) -> ProgressBar {
    let len = match config.total {
        Some(total) => (total * config.unit_scale) as u64,
        None => iter_len as u64,
    };

    let bar = ProgressBar::new(len)
        .with_finish(config.progress_finish())
        .with_prefix(config.prefix)
        .with_style(style(
            config.unit,
            config.unit_scale,
            config.postfix,
            &config.progress_chars,
            &config.colour,
        ));
    if config.disable {
        bar.set_draw_target(ProgressDrawTarget::hidden());
    }
    bar
}

fn style(
    unit: Cow<'static, str>,
    unit_scale: f64,
    postfix: Cow<'static, str>,
    progress_chars: &str,
    color: &str,
) -> ProgressStyle {
    ProgressStyle::with_template(
        &format!("{{prefix}}{{percent}}|{{wide_bar:.{color}}}| {{pos}}/{{len}} [{{elapsed}}<{{eta}}, {{per_sec}}{{postfix}}]",
    ))
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
        if unit_scale.round() == unit_scale {
            let _ = write!(w, "{:?}", unit_scale as i64 * state.pos() as i64);
        } else {
            let _ = write!(w, "{:?}", unit_scale * state.pos() as f64);
        }
    })
    .with_key("len", move |state: &ProgressState, w: &mut dyn Write| {
        if unit_scale.round() == unit_scale {
            let state_len = state.len().unwrap_or(state.pos()) as i64;
            let _ = write!(w, "{:?}", unit_scale as i64 * state_len);
        } else {
            let state_len = state.len().unwrap_or(state.pos()) as f64;
            let _ = write!(w, "{:?}", unit_scale * state_len);
        }
    })
    .with_key("postfix", move |_: &ProgressState, w: &mut dyn Write| {
        let _ = write!(w, "{}", postfix);
    })
    .progress_chars(progress_chars)
}
