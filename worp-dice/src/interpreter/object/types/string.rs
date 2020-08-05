use crate::interpreter::{
    error::RuntimeError,
    object::{self, methods::to_string, types::func::Func, ObjectBase},
    symbol::common::{methods::FN_TO_STRING, operators::OP_ADD, types::TY_STRING},
};
use maplit::hashmap;
use object::{reflection::TypeData, ObjectKey, ObjectRef};
use std::{collections::HashMap, rc::Rc};

thread_local! {
    static OPERATIONS: HashMap<ObjectKey, ObjectRef> = hashmap! [
        ObjectKey::Symbol(OP_ADD) => ObjectRef::new(Func::new_func2(concat)),
        ObjectKey::Symbol(FN_TO_STRING) => ObjectRef::new(Func::from_raw_func1(to_string)),
        "length".into() => ObjectRef::new(Func::new_func1(length)),
        "is_empty".into() => ObjectRef::new(Func::new_func1(is_empty)),
    ];

    static TYPE_DATA: TypeData = TypeData::new(TY_STRING, Vec::new());
}

impl ObjectBase for Rc<str> {
    fn get(&self, key: &ObjectKey) -> Result<ObjectRef, RuntimeError> {
        OPERATIONS.with(|ops_table| ops_table.get(key).cloned().ok_or_else(|| RuntimeError::MissingField(key.clone())))
    }

    fn properties(&self) -> Vec<(ObjectKey, TypeData)> {
        OPERATIONS.with(|ops| {
            ops.clone()
                .into_iter()
                .map(|(key, value)| (key, value.instance_type_data()))
                .collect::<Vec<_>>()
        })
    }

    fn type_data() -> TypeData {
        TYPE_DATA.with(Clone::clone)
    }

    fn instance_type_data(&self) -> TypeData {
        Self::type_data()
    }

    fn format_value(&self) -> String {
        ToString::to_string(self)
    }
}

fn concat(lhs: &Rc<str>, rhs: &Rc<str>) -> Rc<str> {
    format!("{}{}", lhs, rhs).into()
}

fn length(value: &Rc<str>) -> i64 {
    value.len() as i64
}

fn is_empty(value: &Rc<str>) -> bool {
    value.is_empty()
}
