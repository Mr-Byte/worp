#![allow(dead_code)]

use crate::runtime::{
    core::{TypeInstance, Value},
    error::RuntimeError,
};
use std::{fmt::Display, ops::Deref};

#[derive(Debug)]
pub struct Range {
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

            Ok(Value::boxed(Range::new(*lower, *upper)))
        } else {
            Err(RuntimeError::InvalidFunctionArgs(2, args.len()))
        }
    }
}

#[derive(Debug)]
pub struct RangeInclusive {
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

            Ok(Value::boxed(RangeInclusive::new(*lower, *upper)))
        } else {
            Err(RuntimeError::InvalidFunctionArgs(2, args.len()))
        }
    }
}
