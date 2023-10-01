use indicatif::ProgressFinish;
use std::{borrow::Cow, fmt, ops::Mul};

#[derive(Clone, Copy)]
pub enum Number {
    Float(f64),
    Int(i64),
}

#[derive(Debug)]
pub struct Config {
    pub desc: Cow<'static, str>,
    pub total: Option<u64>,
    pub leave: bool,
    pub disable: bool,
    pub unit: Cow<'static, str>,
    pub unit_scale: Number,
    pub postfix: Cow<'static, str>,
}

impl Config {
    pub fn new() -> Self {
        Self::default()
    }
    pub fn with_prefix<S>(mut self, prefix: S) -> Self
    where
        S: Into<Cow<'static, str>>,
    {
        self.desc = prefix.into();
        self
    }
    pub fn with_desc<S>(self, desc: S) -> Self
    where
        S: Into<Cow<'static, str>>,
    {
        self.with_prefix(desc.into() + ": ")
    }
    pub fn with_total(mut self, total: usize) -> Self {
        self.total = Some(total as u64);
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
    pub fn with_unit<S>(mut self, unit: S) -> Config
    where
        S: Into<Cow<'static, str>>,
    {
        self.unit = unit.into();
        self
    }
    pub fn with_scale(mut self, scale: impl Into<Number>) -> Config {
        self.unit_scale = scale.into();
        self
    }

    pub fn with_postfix<S>(mut self, postfix: S) -> Config
    where
        S: Into<Cow<'static, str>>,
    {
        self.postfix = (", ".to_owned() + &postfix.into()).into();
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
            unit_scale: Number::Int(1),
            postfix: "".into(),
        }
    }
}

impl From<f64> for Number {
    fn from(value: f64) -> Self {
        Self::Float(value)
    }
}

impl From<i64> for Number {
    fn from(value: i64) -> Self {
        Self::Int(value)
    }
}

impl fmt::Debug for Number {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::Float(float) => write!(f, "{float:?}"),
            Self::Int(int) => write!(f, "{int:?}"),
        }
    }
}

impl Mul<u64> for Number {
    type Output = Number;
    fn mul(self, rhs: u64) -> Self::Output {
        match self {
            Number::Int(int) => Number::Int(int * rhs as i64),
            Number::Float(float) => Number::Float(float * rhs as f64),
        }
    }
}

impl Mul<f64> for Number {
    type Output = f64;
    fn mul(self, rhs: f64) -> Self::Output {
        match self {
            Number::Int(int) => int as f64 * rhs,
            Number::Float(float) => float * rhs,
        }
    }
}
