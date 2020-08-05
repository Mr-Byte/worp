use crate::interpreter::{
    error::RuntimeError,
    object::{self, methods::to_string, types::func::Func, ObjectBase},
    symbol::common::{methods::FN_TO_STRING, operators::OP_ADD, types::TY_LIST},
};
use maplit::hashmap;
use object::{reflection::TypeData, ObjectKey, ObjectRef};
use std::{collections::HashMap, iter, rc::Rc};

thread_local! {
    static OPERATIONS: HashMap<ObjectKey, ObjectRef> = hashmap! [
        ObjectKey::Symbol(OP_ADD) => ObjectRef::new(Func::from_raw_func2(concat)),
        ObjectKey::Symbol(FN_TO_STRING) => ObjectRef::new(Func::from_raw_func1(to_string)),
        "length".into() => ObjectRef::new(Func::new_func1(length)),
        "is_empty".into() => ObjectRef::new(Func::new_func1(is_empty)),
    ];

    static TYPE_DATA: TypeData = TypeData::new(TY_LIST, Vec::new());
}

impl ObjectBase for Rc<[ObjectRef]> {
    fn get(&self, key: &ObjectKey) -> Result<ObjectRef, RuntimeError> {
        if let ObjectKey::Index(index) = key {
            let index = if *index >= 0 { *index } else { (self.len() as i64) + *index };

            if (index as usize) >= self.len() || index < 0 {
                Err(RuntimeError::IndexOutOfBounds(self.len(), index))
            } else {
                Ok(self[index as usize].clone())
            }
        } else {
            OPERATIONS.with(|ops_table| ops_table.get(key).cloned().ok_or_else(|| RuntimeError::MissingField(key.clone())))
        }
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
        let items = self.iter().map(|obj| obj.format_value()).collect::<Vec<_>>().join(", ");

        format!("[ {} ]", items)
    }
}

fn concat(lhs: ObjectRef, rhs: ObjectRef) -> Result<ObjectRef, RuntimeError> {
    let lhs = lhs
        .value::<Rc<[ObjectRef]>>()
        .ok_or_else(|| RuntimeError::InvalidType(TY_LIST, lhs.instance_type_data().type_tag().clone()))?;
    let output: Rc<[ObjectRef]> = if let Some(list) = rhs.value::<Rc<[ObjectRef]>>() {
        lhs.iter().chain(list.iter()).cloned().collect::<Vec<_>>().into()
    } else {
        lhs.iter().chain(iter::once(&rhs)).cloned().collect::<Vec<_>>().into()
    };

    Ok(ObjectRef::new(output))
}

fn length(value: &Rc<[ObjectRef]>) -> i64 {
    value.len() as i64
}

fn is_empty(value: &Rc<[ObjectRef]>) -> bool {
    value.is_empty()
}
