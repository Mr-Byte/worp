use super::environment::Environment;
use crate::{
    runtime::{
        core::{
            symbol::{common::operators::*, Symbol},
            Type as _, Value, ValueKey,
        },
        error::RuntimeError,
        lib::{
            self, DiceString, List, Object, Range, RangeInclusive, TypeBool, TypeDiceSet, TypeDie, TypeNone, TypeRange,
            TypeRangeInclusive,
        },
    },
    syntax::{BinaryOperator, Literal, RangeOperator, SyntaxTree, UnaryOperator},
};
use std::{collections::HashMap, iter, rc::Rc};

#[inline]
pub fn eval(expr: &SyntaxTree, environment: &Rc<Environment>) -> Result<Value, RuntimeError> {
    eval_expression(expr, environment)
}

fn eval_expression(expr: &SyntaxTree, environment: &Rc<Environment>) -> Result<Value, RuntimeError> {
    match expr {
        SyntaxTree::Literal(literal, _) => eval_literal(literal, environment),
        SyntaxTree::SafeAccess(expr, field, _) => eval_safe_field_access(expr, field, environment),
        SyntaxTree::FieldAccess(expr, field, _) => eval_field_access(expr, field, environment),
        SyntaxTree::FunctionCall(expr, args, _) => eval_function_call(expr, args, environment),
        SyntaxTree::Index(expr, index, _) => eval_index(expr, index, environment),
        SyntaxTree::Unary(op, expr, _) => eval_unary(op, expr, environment),
        SyntaxTree::Binary(op, lhs, rhs, _) => eval_binary(op, lhs, rhs, environment),
        SyntaxTree::Range(op, lower, upper, _) => eval_range(op, lower, upper, environment),
        SyntaxTree::Conditional(condition, body, alternate, _) => {
            eval_conditional(condition, body, alternate.as_deref(), environment)
        }
        SyntaxTree::WhileLoop(condition, body, _) => eval_while_loop(condition, body, environment),
        SyntaxTree::ForLoop(variable, source, body, _) => eval_for_loop(variable, source, body, environment),
        SyntaxTree::VariableDeclaration(identifier, expr, _) => {
            eval_variable_declaration(identifier, expr, environment)
        }
        SyntaxTree::VariableAssignment(identifier, expr, _) => eval_variable_assignment(identifier, expr, environment),
        SyntaxTree::Block(statements, _) => {
            let environment = environment.scoped();
            let mut iter = statements.iter().peekable();
            loop {
                if let Some(statement) = iter.next() {
                    let result = eval_expression(statement, &environment)?;

                    if iter.peek().is_none() {
                        break Ok(result);
                    }
                } else {
                    unreachable!()
                }
            }
        }
    }
}

fn eval_literal(literal: &Literal, environment: &Rc<Environment>) -> Result<Value, RuntimeError> {
    match literal {
        Literal::Identifier(ref symbol) => environment.variable(symbol),
        Literal::None => Ok(Value::NONE),
        Literal::Integer(int) => Ok(Value::new(*int)),
        Literal::Float(float) => Ok(Value::new(*float)),
        Literal::String(string) => Ok(Value::new(Into::<DiceString>::into(string.clone()))),
        Literal::Boolean(bool) => Ok(Value::new(*bool)),
        Literal::List(list) => eval_list_literal(list, environment),
        Literal::Object(object) => eval_object_literal(object, environment),
    }
}

#[inline]
fn eval_list_literal(list: &[SyntaxTree], environment: &Rc<Environment>) -> Result<Value, RuntimeError> {
    let result: List = list
        .iter()
        .map(|expr| eval_expression(expr, environment))
        .collect::<Result<Vec<_>, _>>()?
        .into();

    Ok(Value::new(result))
}

#[inline]
fn eval_object_literal(
    object: &HashMap<ValueKey, SyntaxTree>,
    environment: &Rc<Environment>,
) -> Result<Value, RuntimeError> {
    let result = object
        .iter()
        .map(|(key, value)| Ok::<_, RuntimeError>((key.clone(), eval_expression(value, environment)?)))
        .collect::<Result<_, _>>()?;

    Ok(Value::new(Object::new(result)))
}

fn eval_function_call(
    expr: &SyntaxTree,
    args: &[SyntaxTree],
    environment: &Rc<Environment>,
) -> Result<Value, RuntimeError> {
    match expr {
        SyntaxTree::Literal(Literal::Identifier(target), _) => {
            let args = args
                .iter()
                .map(|arg| eval_expression(arg, environment))
                .collect::<Result<Vec<_>, _>>()?;

            if let Some(known_type) = environment.known_type(target)? {
                known_type.construct(&args)
            } else {
                environment.variable(target)?.call(&args)
            }
        }
        SyntaxTree::FieldAccess(this, method, _) => {
            let method = &ValueKey::Symbol(method.clone());
            eval_method_call(&method, this, args, environment)
        }
        SyntaxTree::Index(this, method, _) => {
            let method = eval_object_key(method, environment)?;
            eval_method_call(&method, this, args, environment)
        }
        _ => unreachable!(),
    }
}

#[inline]
fn eval_method_call(
    method: &ValueKey,
    this: &SyntaxTree,
    args: &[SyntaxTree],
    environment: &Rc<Environment>,
) -> Result<Value, RuntimeError> {
    let args = iter::once_with(|| eval_expression(this, environment))
        .chain(args.iter().map(|arg| eval_expression(arg, environment)))
        .collect::<Result<Vec<Value>, RuntimeError>>()?;

    match args.first() {
        Some(this) => {
            let this = this.get(method)?;
            this.call(&args)
        }
        None => Err(RuntimeError::NoSelfParameterProvided),
    }
}

#[inline]
fn eval_index(expr: &SyntaxTree, index: &SyntaxTree, environment: &Rc<Environment>) -> Result<Value, RuntimeError> {
    let target = eval_expression(expr, environment)?;
    let index = eval_object_key(index, environment)?;

    target.get(&index)
}

fn eval_unary(op: &UnaryOperator, expr: &SyntaxTree, environment: &Rc<Environment>) -> Result<Value, RuntimeError> {
    let object_ref = eval_expression(expr, environment)?;

    match op {
        UnaryOperator::Negate(_) => object_ref.get(&ValueKey::Symbol(OP_NEG))?.call(&[object_ref]),
        UnaryOperator::Not(_) => object_ref.get(&ValueKey::Symbol(OP_NOT))?.call(&[object_ref]),
        UnaryOperator::DiceRoll(_) => TypeDie::instance().construct(&[object_ref]),
    }
}

fn eval_binary(
    op: &BinaryOperator,
    lhs: &SyntaxTree,
    rhs: &SyntaxTree,
    environment: &Rc<Environment>,
) -> Result<Value, RuntimeError> {
    let lhs = eval_expression(lhs, environment)?;
    match op {
        BinaryOperator::LogicalAnd(_) => {
            if *lhs.try_value::<bool>()? {
                eval_expression(rhs, environment)?.assert_type(&TypeBool::NAME)
            } else {
                Ok(Value::new(false))
            }
        }
        BinaryOperator::LogicalOr(_) => {
            if !*lhs.try_value::<bool>()? {
                eval_expression(rhs, environment)?.assert_type(&TypeBool::NAME)
            } else {
                Ok(Value::new(true))
            }
        }
        op => {
            let rhs = eval_expression(rhs, environment)?;
            match op {
                BinaryOperator::DiceRoll(_) => {
                    let die = TypeDie::instance().construct(&[rhs])?;
                    TypeDiceSet::instance().construct(&[lhs, die])
                }
                BinaryOperator::Multiply(_) => lhs.get(&ValueKey::Symbol(OP_MUL))?.call(&[lhs, rhs]),
                BinaryOperator::Divide(_) => lhs.get(&ValueKey::Symbol(OP_DIV))?.call(&[lhs, rhs]),
                BinaryOperator::Remainder(_) => lhs.get(&ValueKey::Symbol(OP_REM))?.call(&[lhs, rhs]),
                BinaryOperator::Add(_) => lhs.get(&ValueKey::Symbol(OP_ADD))?.call(&[lhs, rhs]),
                BinaryOperator::Subtract(_) => lhs.get(&ValueKey::Symbol(OP_SUB))?.call(&[lhs, rhs]),
                BinaryOperator::Equals(_) => lhs.get(&ValueKey::Symbol(OP_EQ))?.call(&[lhs, rhs]),
                BinaryOperator::NotEquals(_) => lhs.get(&ValueKey::Symbol(OP_NE))?.call(&[lhs, rhs]),
                BinaryOperator::GreaterThan(_) => lhs.get(&ValueKey::Symbol(OP_GT))?.call(&[lhs, rhs]),
                BinaryOperator::GreaterThanOrEquals(_) => lhs.get(&ValueKey::Symbol(OP_GTE))?.call(&[lhs, rhs]),
                BinaryOperator::LessThan(_) => lhs.get(&ValueKey::Symbol(OP_LT))?.call(&[lhs, rhs]),
                BinaryOperator::LessThanOrEquals(_) => lhs.get(&ValueKey::Symbol(OP_LTE))?.call(&[lhs, rhs]),

                BinaryOperator::Coalesce(_) => {
                    if *lhs.instance_type().name() != TypeNone::NAME {
                        Ok(lhs)
                    } else {
                        Ok(rhs)
                    }
                }
                _ => unreachable!(),
            }
        }
    }
}

fn eval_range(
    op: &RangeOperator,
    lower: &SyntaxTree,
    upper: &SyntaxTree,
    environment: &Rc<Environment>,
) -> Result<Value, RuntimeError> {
    let lower = eval_expression(lower, environment)?;
    let upper = eval_expression(upper, environment)?;

    match op {
        RangeOperator::Exclusive(_) => TypeRange::instance().construct(&[lower, upper]),
        RangeOperator::Inclusive(_) => TypeRangeInclusive::instance().construct(&[lower, upper]),
    }
}

#[inline]
fn eval_safe_field_access(
    expr: &SyntaxTree,
    field: &Symbol,
    environment: &Rc<Environment>,
) -> Result<Value, RuntimeError> {
    let object_ref = eval_expression(expr, environment)?;

    if *object_ref.instance_type().name() != TypeNone::NAME {
        object_ref.get(&ValueKey::Symbol(field.clone()))
    } else {
        Ok(Value::NONE)
    }
}

#[inline]
fn eval_field_access(expr: &SyntaxTree, field: &Symbol, environment: &Rc<Environment>) -> Result<Value, RuntimeError> {
    let object_ref = eval_expression(expr, environment)?;
    object_ref.get(&ValueKey::Symbol(field.clone()))
}

#[inline]
fn eval_object_key(expr: &SyntaxTree, environment: &Rc<Environment>) -> Result<ValueKey, RuntimeError> {
    let index = eval_expression(expr, environment)?;

    if let Some(index) = index.value::<i64>() {
        Ok(ValueKey::Index(*index))
    } else if let Some(index) = index.value::<DiceString>() {
        let index: String = index.to_string();
        Ok(ValueKey::Symbol(Symbol::new(index)))
    } else {
        Err(RuntimeError::InvalidKeyType(index.instance_type().name().clone()))
    }
}

fn eval_conditional(
    condition: &SyntaxTree,
    body: &SyntaxTree,
    alternate: Option<&SyntaxTree>,
    environment: &Rc<Environment>,
) -> Result<Value, RuntimeError> {
    let condition_result = eval_expression(condition, environment)?;
    let condition = *condition_result
        .value::<bool>()
        .ok_or_else(|| RuntimeError::InvalidType(TypeBool::NAME, condition_result.instance_type().name().clone()))?;

    if condition {
        let environment = environment.scoped();
        eval_expression(body, &environment)
    } else if let Some(alternate) = alternate {
        let environment = environment.scoped();
        eval_expression(alternate, &environment)
    } else {
        Ok(Value::NONE)
    }
}

fn eval_while_loop(
    condition: &SyntaxTree,
    body: &SyntaxTree,
    environment: &Rc<Environment>,
) -> Result<Value, RuntimeError> {
    while *eval_expression(condition, environment)?.try_value::<bool>()? {
        eval_expression(body, environment)?;
    }

    Ok(Value::new(lib::None))
}

fn eval_for_loop(
    variable: &Symbol,
    source: &SyntaxTree,
    body: &SyntaxTree,
    environment: &Rc<Environment>,
) -> Result<Value, RuntimeError> {
    let source = eval_expression(source, environment)?;
    let environment = environment.scoped();
    environment.add_variable(variable.clone(), Value::new(lib::None))?;

    if let Some(as_list) = source.value::<List>() {
        for value in as_list.iter() {
            environment.set_variable(variable, value.clone())?;
            eval_expression(body, &environment)?;
        }
    } else if let Some(as_range) = source.value::<Range>() {
        for value in (*as_range).clone().into_iter() {
            environment.set_variable(variable, Value::new(value))?;
            eval_expression(body, &environment)?;
        }
    } else if let Some(as_range) = source.value::<RangeInclusive>() {
        for value in (*as_range).clone().into_iter() {
            environment.set_variable(variable, Value::new(value))?;
            eval_expression(body, &environment)?;
        }
    }

    Ok(Value::new(lib::None))
}

fn eval_variable_declaration(
    identifier: &Symbol,
    expr: &SyntaxTree,
    environment: &Rc<Environment>,
) -> Result<Value, RuntimeError> {
    let value = eval_expression(expr, environment)?;
    environment.add_variable(identifier.clone(), value.clone())?;
    Ok(value)
}

fn eval_variable_assignment(
    identifier: &Symbol,
    expr: &SyntaxTree,
    environment: &Rc<Environment>,
) -> Result<Value, RuntimeError> {
    let value = eval_expression(expr, environment)?;
    environment.set_variable(identifier, value.clone())?;
    Ok(value)
}
