macro_rules! op {
    (OP_MUL, $lhs:expr, $rhs:expr) => {
        $lhs * $rhs
    };
    (OP_DIV, $lhs:expr, $rhs:expr) => {
        $lhs / $rhs
    };
    (OP_REM, $lhs:expr, $rhs:expr) => {
        $lhs % $rhs
    };
    (OP_ADD, $lhs:expr, $rhs:expr) => {
        $lhs + $rhs
    };
    (OP_SUB, $lhs:expr, $rhs:expr) => {
        $lhs - $rhs
    };
    (OP_EQ, $lhs:expr, $rhs:expr) => {
        $lhs == $rhs
    };
    (OP_NEQ, $lhs:expr, $rhs:expr) => {
        $lhs != $rhs
    };
    (OP_GT, $lhs:expr, $rhs:expr) => {
        $lhs > $rhs
    };
    (OP_GTE, $lhs:expr, $rhs:expr) => {
        $lhs >= $rhs
    };
    (OP_LT, $lhs:expr, $rhs:expr) => {
        $lhs < $rhs
    };
    (OP_LTE, $lhs:expr, $rhs:expr) => {
        $lhs <= $rhs
    };
}

#[macro_export]
macro_rules! arithmetic_op {
    ($stack:expr, $op:ident) => {
        match ($stack.pop(), $stack.pop()) {
            (Value::Int(lhs), Value::Int(rhs)) => $stack.push(Value::Int(op!($op, lhs, rhs))),
            (Value::Float(lhs), Value::Float(rhs)) => $stack.push(Value::Float(op!($op, lhs, rhs))),
            (lhs, rhs) => $stack.push(lhs.get(&ValueKey::Symbol($op))?.call(&[lhs, rhs])?),
        }
    };
}

#[macro_export]
macro_rules! comparison_op {
    ($stack:expr, OP_EQ) => {
        match ($stack.pop(), $stack.pop()) {
            (Value::None(_), Value::None(_)) => $stack.push(Value::Bool(true)),
            (Value::None(_), _) => $stack.push(Value::Bool(false)),
            (_, Value::None(_)) => $stack.push(Value::Bool(false)),
            (Value::Unit(_), Value::Unit(_)) => $stack.push(Value::Bool(true)),
            (Value::Unit(_), _) => $stack.push(Value::Bool(false)),
            (_, Value::Unit(_)) => $stack.push(Value::Bool(false)),
            (Value::Bool(lhs), Value::Bool(rhs)) => $stack.push(Value::Bool(op!(OP_EQ, lhs, rhs))),
            (Value::Int(lhs), Value::Int(rhs)) => $stack.push(Value::Bool(op!(OP_EQ, lhs, rhs))),
            (Value::Float(lhs), Value::Float(rhs)) => $stack.push(Value::Bool(op!(OP_EQ, lhs, rhs))),
            (lhs, rhs) => $stack.push(lhs.get(&ValueKey::Symbol(OP_EQ))?.call(&[lhs, rhs])?),
        }
    };

    ($stack:expr, OP_NEQ) => {
        match ($stack.pop(), $stack.pop()) {
            (Value::None(_), Value::None(_)) => $stack.push(Value::Bool(false)),
            (Value::None(_), _) => $stack.push(Value::Bool(true)),
            (_, Value::None(_)) => $stack.push(Value::Bool(true)),
            (Value::Unit(_), Value::Unit(_)) => $stack.push(Value::Bool(false)),
            (Value::Unit(_), _) => $stack.push(Value::Bool(true)),
            (_, Value::Unit(_)) => $stack.push(Value::Bool(true)),
            (Value::Bool(lhs), Value::Bool(rhs)) => $stack.push(Value::Bool(op!(OP_NEQ, lhs, rhs))),
            (Value::Int(lhs), Value::Int(rhs)) => $stack.push(Value::Bool(op!(OP_NEQ, lhs, rhs))),
            (Value::Float(lhs), Value::Float(rhs)) => $stack.push(Value::Bool(op!(OP_NEQ, lhs, rhs))),
            (lhs, rhs) => $stack.push(lhs.get(&ValueKey::Symbol(OP_NEQ))?.call(&[lhs, rhs])?),
        }
    };

    ($stack:expr, $op:ident) => {
        match ($stack.pop(), $stack.pop()) {
            (Value::Bool(lhs), Value::Bool(rhs)) => $stack.push(Value::Bool(op!($op, lhs, rhs))),
            (Value::Int(lhs), Value::Int(rhs)) => $stack.push(Value::Bool(op!($op, lhs, rhs))),
            (Value::Float(lhs), Value::Float(rhs)) => $stack.push(Value::Bool(op!($op, lhs, rhs))),
            (lhs, rhs) => $stack.push(lhs.get(&ValueKey::Symbol($op))?.call(&[lhs, rhs])?),
        }
    };
}

#[macro_export]
macro_rules! unary_op {
    ($bytecode:expr, $stack:expr, $op:expr) => {{
        let value = $stack.pop();
        let result = value.get(&ValueKey::Symbol($op))?.call(&[value])?;
        $stack.push(result);
    }};
}
