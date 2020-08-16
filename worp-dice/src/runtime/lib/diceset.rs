use super::{Die, List};
use crate::runtime::{
    core::{TypeInstance, Value},
    error::RuntimeError,
};
use std::{fmt::Display, iter};

#[derive(Debug)]
pub struct DiceSet {
    count: i64,
    die: Die,
}

impl DiceSet {
    fn with_die_count(die: Die, count: i64) -> Self {
        Self { die, count }
    }

    fn roll(&self) -> Value {
        let result: List = iter::repeat_with(|| self.die.roll()).take(self.count as usize).collect::<Vec<_>>().into();
        result.into()
    }
}

impl Display for DiceSet {
    fn fmt(&self, fmt: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        Display::fmt(&self.roll(), fmt)
    }
}

impl TypeInstance for DiceSet {}

decl_type! {
    impl TypeDiceSet for DiceSet as "DiceSet";

    constructor(&self, args: &[Value]) {
        if let [count, die] = args {
            let count = count.try_value::<i64>()?;
            let die = die.try_value::<Die>()?;
            let dice_set = DiceSet::with_die_count(die.clone(), *count);

            Ok(Value::new(dice_set))
        } else {
            Err(RuntimeError::InvalidFunctionArgs(1, args.len()))
        }
    }

    fn roll(this: Value) -> Result<Value, RuntimeError> {
        let this = this.try_value::<DiceSet>()?;

        Ok(this.roll())
    }
}
