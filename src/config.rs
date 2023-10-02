use indicatif::ProgressFinish;
use std::borrow::Cow;

/// Configuration for the Tqdm and ParTqdm progress bars.
#[derive(Debug)]
pub struct Config {
    /// Placed at the start of the progress bar (default = "")
    pub prefix: Cow<'static, str>,
    /// Expected number of iterations (default = iter.len())
    pub total: Option<f64>,
    /// Whether or not to leave the progress bar behind after completion (default = true)
    pub leave: bool,
    /// Hides the progress bar (default = false)
    pub disable: bool,
    /// Measurement type used for it/s (default = "it")
    pub unit: Cow<'static, str>,
    /// Scales the it/s, position, and total numbers. (default = 1)
    pub unit_scale: f64,
    /// Placed at the end of the progress bar (default = "")
    pub postfix: Cow<'static, str>,
    /// Characters used for displaying the progress bar.
    pub progress_chars: Cow<'static, str>,
}

impl Config {
    pub fn new() -> Self {
        Self::default()
    }
    /// Placed at the start of the progress bar (default = "")
    pub fn with_prefix(mut self, prefix: impl Into<Cow<'static, str>>) -> Self {
        self.prefix = prefix.into();
        self
    }
    /// Placed at the start of the progress bar (default = "")
    /// Like tqdm this method adds ": " to the end of the prefix.
    pub fn with_desc(self, desc: impl Into<Cow<'static, str>>) -> Self {
        self.with_prefix(desc.into() + ": ")
    }
    /// Expected number of iterations (default = iter.len())
    pub fn with_total(mut self, total: impl Into<f64>) -> Self {
        self.total = Some(total.into());
        self
    }
    /// Whether or not to leave the progress bar behind after completion (default = true)
    pub fn with_leave(mut self, leave: bool) -> Self {
        self.leave = leave;
        self
    }
    /// Hides the progress bar (default = false)
    pub fn with_disable(mut self, disable: bool) -> Self {
        self.disable = disable;
        self
    }
    /// Measurement type used for it/s (default = "it")
    pub fn with_unit(mut self, unit: impl Into<Cow<'static, str>>) -> Config {
        self.unit = unit.into();
        self
    }
    /// Scales the it/s, position, and total numbers. (default = 1)
    pub fn with_scale(mut self, scale: impl Into<f64>) -> Config {
        self.unit_scale = scale.into();
        self
    }
    /// Placed at the end of the progress bar (default = "")
    pub fn with_postfix(mut self, postfix: impl Into<Cow<'static, str>>) -> Config {
        self.postfix = (", ".to_owned() + &postfix.into()).into();
        self
    }
    pub fn with_progress_chars(mut self, progress_chars: impl Into<Cow<'static, str>>) -> Config {
        self.progress_chars = progress_chars.into();
        self
    }
    /// Characters used for displaying the progress bar.
    pub(crate) fn progress_finish(&self) -> ProgressFinish {
        if self.leave {
            ProgressFinish::AndLeave
        } else {
            ProgressFinish::AndClear
        }
    }
}

impl Default for Config {
    fn default() -> Self {
        Self {
            total: None,
            prefix: "".into(),
            leave: true,
            disable: false,
            unit: "it".into(),
            unit_scale: 1.0,
            postfix: "".into(),

            progress_chars: "█▉▊▋▌▍▎▏ ".into(),
        }
    }
}
