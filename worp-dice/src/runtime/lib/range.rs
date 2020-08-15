#![allow(dead_code)]

use crate::runtime::{
    core::{TypeInstance, Value},
    error::RuntimeError,
};
use std::{fmt::Display, rc::Rc};

#[derive(Debug)]
enum RangeKind {
    Exclusive(std::ops::Range<i64>),
    Inclusive(std::ops::RangeInclusive<i64>),
}

#[derive(Debug)]
pub struct Range {
    kind: RangeKind,
}

impl Range {
    fn with_exclusive(lower: i64, upper: i64) -> Self {
        Range {
            kind: RangeKind::Exclusive(lower..upper),
        }
    }

    fn with_inclusive(lower: i64, upper: i64) -> Self {
        Range {
            kind: RangeKind::Inclusive(lower..=upper),
        }
    }
}

impl Display for Range {
    fn fmt(&self, _fmt: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        todo!()
    }
}

impl TypeInstance for Range {}

decl_type! {
    impl TypeRange for Range as "Range";

    constructor(&self, args: &[Value]) {
        if let [is_inclusive, lower, upper] = args {
            let _is_inclusive = is_inclusive.try_value::<bool>()?;
            let _lower = lower.try_value::<i64>()?;
            let _upper = upper.try_value::<i64>()?;

            todo!();
        } else {
            Err(RuntimeError::InvalidFunctionArgs(3, args.len()))
        }
    }
}
