use crate::runtime::object::{reflection::Type, ObjectBase};
use std::{fmt::Display, ops::Deref, rc::Rc};

#[derive(Debug, Clone)]
pub struct RcString(Rc<str>);

impl ObjectBase for RcString {
    fn reflect_type(&self) -> Rc<dyn Type> {
        todo!()
    }
}

impl Display for RcString {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        self.0.fmt(f)
    }
}

impl Deref for RcString {
    type Target = str;

    fn deref(&self) -> &Self::Target {
        &*self.0
    }
}

impl From<String> for RcString {
    fn from(value: String) -> Self {
        RcString(value.into())
    }
}

// thread_local! {
//     static OPERATIONS: HashMap<ObjectKey, ObjectInstance> = hashmap! [
//         ObjectKey::Symbol(OP_ADD) => ObjectRef::new(Func::new_func2(concat)),
//         ObjectKey::Symbol(FN_TO_STRING) => ObjectRef::new(Func::from_raw_func1(to_string)),
//         "length".into() => ObjectRef::new(Func::new_func1(length)),
//         "is_empty".into() => ObjectRef::new(Func::new_func1(is_empty)),
//     ];

//     static TYPE_DATA: TypeData = TypeData::new(TY_STRING, Vec::new());
// }

// fn concat(lhs: &Rc<str>, rhs: &Rc<str>) -> Rc<str> {
//     format!("{}{}", lhs, rhs).into()
// }

// fn length(value: &Rc<str>) -> i64 {
//     value.len() as i64
// }

// fn is_empty(value: &Rc<str>) -> bool {
//     value.is_empty()
// }
