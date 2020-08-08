use super::func::Func;
use crate::runtime::{
    core::{
        symbol::{
            common::{lib::TY_BOOL, operators::*},
            Symbol,
        },
        Type, TypeInstanceBase, Value, ValueKey,
    },
    error::RuntimeError,
};
use maplit::hashmap;
use std::{collections::HashMap, rc::Rc};

thread_local! {
    static TYPE: Rc<TypeBool> = Default::default();
}

#[derive(Debug)]
struct TypeBool {
    name: Symbol,
    instance_members: HashMap<ValueKey, Value>,
}

impl Default for TypeBool {
    fn default() -> Self {
        Self {
            name: TY_BOOL,
            instance_members: hashmap! [
                ValueKey::Symbol(OP_NOT) => Value::new(Func::new_func1(not)),
                ValueKey::Symbol(OP_AND) => Value::new(Func::new_func2(and)),
                ValueKey::Symbol(OP_OR) => Value::new(Func::new_func2(or)),
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

impl Type for TypeBool {
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

impl TypeInstanceBase for bool {
    fn reflect_type(&self) -> Rc<dyn Type> {
        TYPE.with(Clone::clone)
    }
}

fn not(value: Value) -> Result<Value, RuntimeError> {
    let value = value.try_value::<bool>(&TY_BOOL)?;

    Ok(Value::new(!value))
}

fn eq(lhs: Value, rhs: Value) -> Result<Value, RuntimeError> {
    let lhs = lhs.try_value::<bool>(&TY_BOOL)?;
    let rhs = rhs.value::<bool>();
    let result = match rhs {
        Some(rhs) => lhs == rhs,
        None => false,
    };

    Ok(Value::new(result))
}

fn ne(lhs: Value, rhs: Value) -> Result<Value, RuntimeError> {
    let lhs = lhs.try_value::<bool>(&TY_BOOL)?;
    let rhs = rhs.value::<bool>();
    let result = match rhs {
        Some(rhs) => lhs != rhs,
        None => true,
    };

    Ok(Value::new(result))
}

fn and(lhs: Value, rhs: Value) -> Result<Value, RuntimeError> {
    let lhs = lhs.try_value::<bool>(&TY_BOOL)?;
    let rhs = rhs.try_value::<bool>(&TY_BOOL)?;

    Ok(Value::new(*lhs && *rhs))
}

fn or(lhs: Value, rhs: Value) -> Result<Value, RuntimeError> {
    let lhs = lhs.try_value::<bool>(&TY_BOOL)?;
    let rhs = rhs.try_value::<bool>(&TY_BOOL)?;

    Ok(Value::new(*lhs || *rhs))
}

fn gt(lhs: Value, rhs: Value) -> Result<Value, RuntimeError> {
    let lhs = lhs.try_value::<bool>(&TY_BOOL)?;
    let rhs = rhs.try_value::<bool>(&TY_BOOL)?;

    Ok(Value::new(lhs > rhs))
}

fn gte(lhs: Value, rhs: Value) -> Result<Value, RuntimeError> {
    let lhs = lhs.try_value::<bool>(&TY_BOOL)?;
    let rhs = rhs.try_value::<bool>(&TY_BOOL)?;

    Ok(Value::new(lhs >= rhs))
}

fn lt(lhs: Value, rhs: Value) -> Result<Value, RuntimeError> {
    let lhs = lhs.try_value::<bool>(&TY_BOOL)?;
    let rhs = rhs.try_value::<bool>(&TY_BOOL)?;

    Ok(Value::new(lhs < rhs))
}

fn lte(lhs: Value, rhs: Value) -> Result<Value, RuntimeError> {
    let lhs = lhs.try_value::<bool>(&TY_BOOL)?;
    let rhs = rhs.try_value::<bool>(&TY_BOOL)?;

    Ok(Value::new(lhs <= rhs))
}
