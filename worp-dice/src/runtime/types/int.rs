use crate::runtime::object::{reflection::Type, ObjectBase};
use std::rc::Rc;

impl ObjectBase for i64 {
    fn reflect_type(&self) -> Rc<dyn Type> {
        todo!()
    }
}

// thread_local! {
//     static OPERATIONS: HashMap<ObjectKey, ObjectInstance> = hashmap! [
//         ObjectKey::Symbol(OP_NEG) => ObjectRef::new(Func::new_func1(negate)),
//         ObjectKey::Symbol(OP_MUL) => ObjectRef::new(Func::new_func2(mul)),
//         ObjectKey::Symbol(OP_DIV) => ObjectRef::new(Func::new_func2(div)),
//         ObjectKey::Symbol(OP_REM) => ObjectRef::new(Func::new_func2(rem)),
//         ObjectKey::Symbol(OP_ADD) => ObjectRef::new(Func::new_func2(add)),
//         ObjectKey::Symbol(OP_SUB) => ObjectRef::new(Func::new_func2(sub)),
//         ObjectKey::Symbol(OP_GT) => ObjectRef::new(Func::new_func2(gt)),
//         ObjectKey::Symbol(OP_LT) => ObjectRef::new(Func::new_func2(lt)),
//         ObjectKey::Symbol(OP_GTE) => ObjectRef::new(Func::new_func2(gte)),
//         ObjectKey::Symbol(OP_LTE) => ObjectRef::new(Func::new_func2(lte)),
//         ObjectKey::Symbol(OP_EQ) => ObjectRef::new(Func::new_func2(eq)),
//         ObjectKey::Symbol(OP_NE) => ObjectRef::new(Func::new_func2(ne)),
//         ObjectKey::Symbol(FN_TO_STRING) => ObjectRef::new(Func::from_raw_func1(to_string)),
//     ];

//     static TYPE_DATA: TypeData = TypeData::new(TY_INT, Vec::new());
// }

// fn negate(arg: &i64) -> i64 {
//     -arg
// }

// fn mul(lhs: &i64, rhs: &i64) -> i64 {
//     lhs * rhs
// }

// fn div(lhs: &i64, rhs: &i64) -> i64 {
//     lhs / rhs
// }

// fn rem(lhs: &i64, rhs: &i64) -> i64 {
//     lhs % rhs
// }

// fn add(lhs: &i64, rhs: &i64) -> i64 {
//     lhs + rhs
// }

// fn sub(lhs: &i64, rhs: &i64) -> i64 {
//     lhs - rhs
// }

// fn gt(lhs: &i64, rhs: &i64) -> bool {
//     lhs > rhs
// }

// fn lt(lhs: &i64, rhs: &i64) -> bool {
//     lhs < rhs
// }

// fn gte(lhs: &i64, rhs: &i64) -> bool {
//     lhs >= rhs
// }

// fn lte(lhs: &i64, rhs: &i64) -> bool {
//     lhs <= rhs
// }

// fn eq(lhs: &i64, rhs: &i64) -> bool {
//     lhs == rhs
// }

// fn ne(lhs: &i64, rhs: &i64) -> bool {
//     lhs != rhs
// }

// #[cfg(test)]
// mod test {
//     use super::*;

//     #[test]
//     fn test_add_with_two_ints() -> Result<(), RuntimeError> {
//         let lhs = ObjectInstance::new(40);
//         let rhs = ObjectInstance::new(2);
//         let result = lhs.get(&ObjectKey::Symbol(OP_ADD))?.call(vec![lhs, rhs].as_slice())?;

//         assert_eq!(42, *result.value::<i64>().unwrap());

//         Ok(())
//     }

//     #[test]
//     fn test_add_with_lhs_int_rhs_none() -> Result<(), RuntimeError> {
//         let lhs = ObjectInstance::new(40);
//         let rhs = ObjectInstance::NONE;
//         let result = lhs.get(&ObjectKey::Symbol(OP_ADD))?.call(vec![lhs, rhs].as_slice());

//         assert!(result.is_err());

//         Ok(())
//     }

//     #[test]
//     fn test_eq_with_two_ints() -> Result<(), RuntimeError> {
//         let lhs = ObjectInstance::new(40);
//         let rhs = ObjectInstance::new(2);
//         let result = lhs.get(&ObjectKey::Symbol(OP_EQ))?.call(vec![lhs, rhs].as_slice())?;

//         assert_eq!(false, *result.value::<bool>().unwrap());

//         Ok(())
//     }
// }
