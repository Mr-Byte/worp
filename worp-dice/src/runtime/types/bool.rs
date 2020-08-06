use crate::runtime::{
    error::RuntimeError,
    object::{instance::ObjectInstance, key::ObjectKey, reflection::Type, ObjectBase},
};
use std::rc::Rc;

thread_local! {
    static TYPE: Rc<TypeBool> = Rc::new(TypeBool {});
}

pub struct TypeBool {}

impl Type for TypeBool {
    fn construct(&self) -> Result<ObjectInstance, RuntimeError> {
        todo!()
    }

    fn type_name(&self) -> &crate::runtime::symbol::Symbol {
        todo!()
    }

    fn impl_names(&self) -> &[&crate::runtime::symbol::Symbol] {
        todo!()
    }

    fn instance_members(&self) -> &std::collections::HashMap<ObjectKey, ObjectInstance> {
        todo!()
    }
}

impl ObjectBase for bool {
    fn reflect_type(&self) -> Rc<dyn Type> {
        TYPE.with(Clone::clone)
    }
}

// thread_local! {
//     static OPERATIONS: HashMap<ObjectKey, ObjectInstance> = hashmap! [
//         ObjectKey::Symbol(OP_NOT) => ObjectRef::new(Func::new_func1(not)),
//         ObjectKey::Symbol(OP_EQ) => ObjectRef::new(Func::new_func2(eq)),
//         ObjectKey::Symbol(OP_NE) => ObjectRef::new(Func::new_func2(ne)),
//         ObjectKey::Symbol(OP_AND) => ObjectRef::new(Func::new_func2(and)),
//         ObjectKey::Symbol(OP_OR) => ObjectRef::new(Func::new_func2(or)),
//         ObjectKey::Symbol(FN_TO_STRING) => ObjectRef::new(Func::from_raw_func1(to_string)),
//     ];

//     static TYPE_DATA: TypeData = TypeData::new(TY_INT, Vec::new());
// }

// fn not(arg: &bool) -> bool {
//     !arg
// }

// fn eq(lhs: &bool, rhs: &bool) -> bool {
//     lhs == rhs
// }

// fn ne(lhs: &bool, rhs: &bool) -> bool {
//     lhs != rhs
// }

// fn and(lhs: &bool, rhs: &bool) -> bool {
//     *lhs && *rhs
// }

// fn or(lhs: &bool, rhs: &bool) -> bool {
//     *lhs || *rhs
// }

// #[cfg(test)]
// mod test {
//     use super::*;

//     #[test]
//     fn test_eq_with_two_bools() -> Result<(), RuntimeError> {
//         let lhs = ObjectInstance::new(true);
//         let rhs = ObjectInstance::new(false);
//         let result = lhs.get(&ObjectKey::Symbol(OP_EQ))?.call(vec![lhs, rhs].as_slice())?;

//         assert_eq!(false, *result.value::<bool>().unwrap());

//         Ok(())
//     }
// }
