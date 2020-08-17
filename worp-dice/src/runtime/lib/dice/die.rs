use crate::runtime::{
    core::{TypeInstance, Value},
    error::RuntimeError,
    lib::{List, Range, RangeInclusive, TypeDiceSet, TypeInt},
};
use rand::{distributions::Uniform, thread_rng, Rng as _};
use std::fmt::Display;

#[derive(Debug, Clone)]
enum DieDistribution {
    Range(Uniform<i64>),
    List(Uniform<i64>, List),
}

#[derive(Debug, Clone)]
pub struct Die {
    distribution: DieDistribution,
}

impl Die {
    pub fn with_distribution(distribution: impl Into<Uniform<i64>>) -> Self {
        Self {
            distribution: DieDistribution::Range(distribution.into()),
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
            let die = match_type! { value,
                as_int: i64 => Die::with_distribution(1..=*as_int),
                as_list: List => Die::with_list(as_list.clone()),
                as_range: Range => Die::with_distribution((*as_range).clone()),
                as_range_inclusive: RangeInclusive => Die::with_distribution((*as_range_inclusive).clone()),
                _ => return Err(RuntimeError::InvalidType(TypeInt::NAME, value.instance_type().name().clone()))
            };

            Ok(Value::new(die))
        } else {
            Err(RuntimeError::InvalidFunctionArgs(1, args.len()))
        }
    }

    fn op_add(this: Value, rhs: Value) -> Result<Value, RuntimeError> {
        let _this = this.try_value::<Die>()?;

        if rhs.is_instance_of_any(&[&*TypeInt::instance(), &*TypeDie::instance(), &*TypeDiceSet::instance()]) {

        }

        todo!()
    }

    fn roll(this: Value) -> Result<Value, RuntimeError> {
        let this = this.try_value::<Die>()?;

        Ok(this.roll())
    }
}
