#![allow(dead_code)]

use crate::runtime::{
    core::{TypeInstance, Value},
    error::RuntimeError,
};
use gc::{Finalize, Trace};
use std::{fmt::Display, ops::Deref};

#[derive(Debug, Trace, Finalize)]
pub struct Range {
    #[unsafe_ignore_trace]
    range: std::ops::Range<i64>,
}

impl Range {
    fn new(lower: i64, upper: i64) -> Self {
        Self { range: lower..upper }
    }
}

impl Display for Range {
    fn fmt(&self, fmt: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(fmt, "{}..{}", self.range.start, self.range.end)
    }
}

impl Deref for Range {
    type Target = std::ops::Range<i64>;
    fn deref(&self) -> &Self::Target {
        &self.range
    }
}

impl TypeInstance for Range {}

decl_type! {
    impl TypeRange for Range as "Range";

    constructor(&self, args: &[Value]) {
        if let [ lower, upper] = args {
            let lower = lower.try_value::<i64>()?;
            let upper = upper.try_value::<i64>()?;

            if lower > upper {
                return Err(RuntimeError::RangeError(*lower, *upper));
            }

            Ok(Value::new(Range::new(*lower, *upper)))
        } else {
            Err(RuntimeError::InvalidFunctionArgs(2, args.len()))
        }
    }
}

#[derive(Debug, Trace, Finalize)]
pub struct RangeInclusive {
    #[unsafe_ignore_trace]
    range: std::ops::RangeInclusive<i64>,
}

impl RangeInclusive {
    fn new(lower: i64, upper: i64) -> Self {
        Self { range: lower..=upper }
    }
}

impl Deref for RangeInclusive {
    type Target = std::ops::RangeInclusive<i64>;
    fn deref(&self) -> &Self::Target {
        &self.range
    }
}

impl Display for RangeInclusive {
    fn fmt(&self, fmt: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(fmt, "{}..={}", self.range.start(), self.range.end())
    }
}

impl TypeInstance for RangeInclusive {}

decl_type! {
    impl TypeRangeInclusive for RangeInclusive as "RangeInclusive";

    constructor(&self, args: &[Value]) {
        if let [ lower, upper] = args {
            let lower = lower.try_value::<i64>()?;
            let upper = upper.try_value::<i64>()?;

            if lower > upper {
                return Err(RuntimeError::RangeError(*lower, *upper));
            }

            Ok(Value::new(RangeInclusive::new(*lower, *upper)))
        } else {
            Err(RuntimeError::InvalidFunctionArgs(2, args.len()))
        }
    }
}
