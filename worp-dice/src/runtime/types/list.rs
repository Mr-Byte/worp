use crate::runtime::object::{instance::ObjectInstance, reflection::Type, ObjectBase};
use std::{fmt::Display, ops::Deref, rc::Rc};

// TODO: Implement TypeList

#[derive(Debug, Clone)]
pub struct List(Rc<[ObjectInstance]>);

impl ObjectBase for List {
    fn reflect_type(&self) -> Rc<dyn Type> {
        todo!()
    }
}

impl Display for List {
    fn fmt(&self, _f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        todo!()
    }
}

impl Deref for List {
    type Target = [ObjectInstance];

    fn deref(&self) -> &Self::Target {
        &*self.0
    }
}

impl From<Vec<ObjectInstance>> for List {
    fn from(value: Vec<ObjectInstance>) -> Self {
        Self(value.into())
    }
}

// thread_local! {
//     static OPERATIONS: HashMap<ObjectKey, ObjectInstance> = hashmap! [
//         ObjectKey::Symbol(OP_ADD) => ObjectRef::new(Func::from_raw_func2(concat)),
//         ObjectKey::Symbol(FN_TO_STRING) => ObjectRef::new(Func::from_raw_func1(to_string)),
//         "length".into() => ObjectRef::new(Func::new_func1(length)),
//         "is_empty".into() => ObjectRef::new(Func::new_func1(is_empty)),
//     ];

//     static TYPE_DATA: TypeData = TypeData::new(TY_LIST, Vec::new());
// }

// fn concat(lhs: ObjectInstance, rhs: ObjectInstance) -> Result<ObjectInstance, RuntimeError> {
//     let lhs = lhs
//         .value::<Rc<[ObjectInstance]>>()
//         .ok_or_else(|| RuntimeError::InvalidType(TY_LIST, lhs.instance_type_data().type_tag().clone()))?;
//     let output: Rc<[ObjectInstance]> = if let Some(list) = rhs.value::<Rc<[ObjectInstance]>>() {
//         lhs.iter().chain(list.iter()).cloned().collect::<Vec<_>>().into()
//     } else {
//         lhs.iter().chain(iter::once(&rhs)).cloned().collect::<Vec<_>>().into()
//     };

//     Ok(ObjectInstance::new(output))
// }

// fn length(value: &Rc<[ObjectInstance]>) -> i64 {
//     value.len() as i64
// }

// fn is_empty(value: &Rc<[ObjectInstance]>) -> bool {
//     value.is_empty()
// }
