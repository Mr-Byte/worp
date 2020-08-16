use super::List;
use super::{Range, RangeInclusive, TypeInt};
use crate::runtime::{
    core::{TypeInstance, Value},
    error::RuntimeError,
};
use rand::{distributions::Uniform, thread_rng, Rng as _};
use std::{fmt::Display, rc::Rc};

#[derive(Debug)]
enum DieDistribution {
    Range(Uniform<i64>),
    List(Uniform<i64>, List),
}

#[derive(Debug)]
pub struct Die {
    distribution: DieDistribution,
}

impl Die {
    pub fn with_sides(sides: i64) -> Self {
        Self {
            distribution: DieDistribution::Range(From::from(1..=sides)),
        }
    }

    pub fn with_range(range: &Range) -> Self {
        let range: std::ops::Range<i64> = (*range).clone();

        Self {
            distribution: DieDistribution::Range(From::from(range)),
        }
    }

    pub fn with_range_inclusive(range: &RangeInclusive) -> Self {
        let range: std::ops::RangeInclusive<i64> = (*range).clone();

        Self {
            distribution: DieDistribution::Range(From::from(range)),
        }
    }

    pub fn with_list(list: List) -> Self {
        Self {
            distribution: DieDistribution::List(From::from(0..(list.len() as i64)), list),
        }
    }

    pub fn roll(&self) -> Value {
        match &self.distribution {
            DieDistribution::Range(distribution) => Value::new(thread_rng().sample(distribution)),
            DieDistribution::List(distribution, values) => values[thread_rng().sample(distribution) as usize].clone(),
        }
    }
}

impl Display for Die {
    fn fmt(&self, fmt: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(fmt, "<<{}>>", self.roll().to_string())
    }
}

impl TypeInstance for Die {}

decl_type! {
    impl TypeDie for Die as "Die";

    constructor(&self, args: &[Value]) {
        if let [value] = args {
            match_type! { value,
                as_int: i64 => Ok(Value::new(Die::with_sides(*as_int))),
                as_list: List => Ok(Value::new(Die::with_list(as_list.clone()))),
                as_range: Range => Ok(Value::new(Die::with_range(as_range))),
                as_range_inclusive: RangeInclusive => Ok(Value::new(Die::with_range_inclusive(as_range_inclusive))),
                _ => Err(RuntimeError::InvalidType(TypeInt::NAME, value.instance_type().name().clone()))
            }
        } else {
            Err(RuntimeError::InvalidFunctionArgs(1, args.len()))
        }
    }

    fn roll(this: Value) -> Result<Value, RuntimeError> {
        let this = this.try_value::<Die>()?;

        Ok(this.roll())
    }
}
