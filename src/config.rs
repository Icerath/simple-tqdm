use indicatif::ProgressFinish;
use std::borrow::Cow;

#[derive(Debug)]
pub struct Config {
    pub desc: Cow<'static, str>,
    pub total: Option<f64>,
    pub leave: bool,
    pub disable: bool,
    pub unit: Cow<'static, str>,
    pub unit_scale: f64,
    pub postfix: Cow<'static, str>,

    pub progress_chars: Cow<'static, str>,
}

impl Config {
    pub fn new() -> Self {
        Self::default()
    }
    pub fn with_prefix(mut self, prefix: impl Into<Cow<'static, str>>) -> Self {
        self.desc = prefix.into();
        self
    }
    pub fn with_desc(self, desc: impl Into<Cow<'static, str>>) -> Self {
        self.with_prefix(desc.into() + ": ")
    }
    pub fn with_total(mut self, total: impl Into<f64>) -> Self {
        self.total = Some(total.into());
        self
    }
    pub fn with_leave(mut self, leave: bool) -> Self {
        self.leave = leave;
        self
    }
    pub fn with_disable(mut self, disable: bool) -> Self {
        self.disable = disable;
        self
    }
    pub fn with_unit(mut self, unit: impl Into<Cow<'static, str>>) -> Config {
        self.unit = unit.into();
        self
    }
    pub fn with_scale(mut self, scale: impl Into<f64>) -> Config {
        self.unit_scale = scale.into();
        self
    }

    pub fn with_postfix(mut self, postfix: impl Into<Cow<'static, str>>) -> Config {
        self.postfix = (", ".to_owned() + &postfix.into()).into();
        self
    }

    pub fn with_progress_chars(mut self, progress_chars: impl Into<Cow<'static, str>>) -> Config {
        self.progress_chars = progress_chars.into();
        self
    }

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
            desc: " ".into(),
            leave: true,
            disable: false,
            unit: "it".into(),
            unit_scale: 1.0,
            postfix: "".into(),

            progress_chars: "█▉▊▋▌▍▎▏ ".into(),
        }
    }
}
