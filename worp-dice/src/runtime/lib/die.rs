use super::List;
use super::TypeInt;
use crate::runtime::{
    core::{Type, TypeInstanceBase, Value},
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

impl TypeInstanceBase for Die {
    fn reflect_type(&self) -> Rc<dyn Type> {
        TypeDie::instance()
    }
}

decl_type! {
    type TypeDie = "Die";

    constructor(&self, args: &[Value]) {
        if let [value] = args {
            match_type! { value,
                as_int: i64 => Ok(Value::new(Die::with_sides(*as_int))),
                as_list: List => Ok(Value::new(Die::with_list(as_list.clone()))),
                _ => Err(RuntimeError::InvalidType(TypeInt::NAME, value.reflect_type().name().clone()))
            }
        } else {
            Err(RuntimeError::InvalidFunctionArgs(1, args.len()))
        }
    }

    fn roll(this: Value) -> Result<Value, RuntimeError> {
        let this = this.try_value::<Die>(&TypeDie::NAME)?;

        Ok(this.roll())
    }
}
