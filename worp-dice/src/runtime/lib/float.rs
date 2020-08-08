use super::func::Func;
use crate::runtime::{
    core::{key::ValueKey, reflection::Type, value::Value, TypeInstanceBase},
    error::RuntimeError,
    symbol::{common::lib::TY_FLOAT, common::operators::*, Symbol},
};
use maplit::hashmap;
use std::{collections::HashMap, rc::Rc};

thread_local! {
    static TYPE: Rc<TypeFloat> = Default::default();
}

#[derive(Debug)]
struct TypeFloat {
    name: Symbol,
    instance_members: HashMap<ValueKey, Value>,
}

impl Default for TypeFloat {
    fn default() -> Self {
        Self {
            name: TY_FLOAT,
            instance_members: hashmap! [
                ValueKey::Symbol(OP_NEG) => Value::new(Func::new_func1(negate)),
                ValueKey::Symbol(OP_MUL) => Value::new(Func::new_func2(mul)),
                ValueKey::Symbol(OP_DIV) => Value::new(Func::new_func2(div)),
                ValueKey::Symbol(OP_REM) => Value::new(Func::new_func2(rem)),
                ValueKey::Symbol(OP_ADD) => Value::new(Func::new_func2(add)),
                ValueKey::Symbol(OP_SUB) => Value::new(Func::new_func2(sub)),
                ValueKey::Symbol(OP_EQ) => Value::new(Func::new_func2(eq)),
                ValueKey::Symbol(OP_NE) => Value::new(Func::new_func2(ne)),
                ValueKey::Symbol(OP_GT) => Value::new(Func::new_func2(gt)),
                ValueKey::Symbol(OP_GTE) => Value::new(Func::new_func2(gte)),
                ValueKey::Symbol(OP_LT) => Value::new(Func::new_func2(lt)),
                ValueKey::Symbol(OP_LTE) => Value::new(Func::new_func2(lte)),
            ],
        }
    }
}

impl Type for TypeFloat {
    fn name(&self) -> &Symbol {
        &self.name
    }

    fn impl_names(&self) -> &[&Symbol] {
        &[]
    }

    fn members(&self) -> &HashMap<ValueKey, Value> {
        &self.instance_members
    }
}

impl TypeInstanceBase for f64 {
    fn reflect_type(&self) -> Rc<dyn Type> {
        TYPE.with(Clone::clone)
    }
}

fn negate(value: Value) -> Result<Value, RuntimeError> {
    let value = value.try_value::<f64>(&TY_FLOAT)?;

    Ok(Value::new(-value))
}

fn mul(lhs: Value, rhs: Value) -> Result<Value, RuntimeError> {
    let lhs = lhs.try_value::<f64>(&TY_FLOAT)?;
    let rhs = rhs.try_value::<f64>(&TY_FLOAT)?;

    Ok(Value::new(lhs * rhs))
}

fn div(lhs: Value, rhs: Value) -> Result<Value, RuntimeError> {
    let lhs = lhs.try_value::<f64>(&TY_FLOAT)?;
    let rhs = rhs.try_value::<f64>(&TY_FLOAT)?;

    Ok(Value::new(lhs / rhs))
}

fn rem(lhs: Value, rhs: Value) -> Result<Value, RuntimeError> {
    let lhs = lhs.try_value::<f64>(&TY_FLOAT)?;
    let rhs = rhs.try_value::<f64>(&TY_FLOAT)?;

    Ok(Value::new(lhs % rhs))
}

fn add(lhs: Value, rhs: Value) -> Result<Value, RuntimeError> {
    let lhs = lhs.try_value::<f64>(&TY_FLOAT)?;
    let rhs = rhs.try_value::<f64>(&TY_FLOAT)?;

    Ok(Value::new(lhs + rhs))
}

fn sub(lhs: Value, rhs: Value) -> Result<Value, RuntimeError> {
    let lhs = lhs.try_value::<f64>(&TY_FLOAT)?;
    let rhs = rhs.try_value::<f64>(&TY_FLOAT)?;

    Ok(Value::new(lhs - rhs))
}

fn eq(lhs: Value, rhs: Value) -> Result<Value, RuntimeError> {
    let lhs = lhs.try_value::<f64>(&TY_FLOAT)?;
    let rhs = rhs.value::<f64>();
    let result = match rhs {
        Some(rhs) => lhs == rhs,
        None => false,
    };

    Ok(Value::new(result))
}

fn ne(lhs: Value, rhs: Value) -> Result<Value, RuntimeError> {
    let lhs = lhs.try_value::<f64>(&TY_FLOAT)?;
    let rhs = rhs.value::<f64>();
    let result = match rhs {
        Some(rhs) => lhs != rhs,
        None => true,
    };

    Ok(Value::new(result))
}

fn gt(lhs: Value, rhs: Value) -> Result<Value, RuntimeError> {
    let lhs = lhs.try_value::<f64>(&TY_FLOAT)?;
    let rhs = rhs.try_value::<f64>(&TY_FLOAT)?;

    Ok(Value::new(lhs > rhs))
}

fn gte(lhs: Value, rhs: Value) -> Result<Value, RuntimeError> {
    let lhs = lhs.try_value::<f64>(&TY_FLOAT)?;
    let rhs = rhs.try_value::<f64>(&TY_FLOAT)?;

    Ok(Value::new(lhs >= rhs))
}

fn lt(lhs: Value, rhs: Value) -> Result<Value, RuntimeError> {
    let lhs = lhs.try_value::<f64>(&TY_FLOAT)?;
    let rhs = rhs.try_value::<f64>(&TY_FLOAT)?;

    Ok(Value::new(lhs < rhs))
}

fn lte(lhs: Value, rhs: Value) -> Result<Value, RuntimeError> {
    let lhs = lhs.try_value::<f64>(&TY_FLOAT)?;
    let rhs = rhs.try_value::<f64>(&TY_FLOAT)?;

    Ok(Value::new(lhs <= rhs))
}
