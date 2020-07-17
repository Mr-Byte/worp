use std::{fmt::Debug, hash::Hash};

macro_rules! key {
    ($source: ident :: $target: ident as $key: ident: $typ: ty) => {
        #[derive(Clone, Debug, Hash, Eq, PartialEq, Ord, PartialOrd, Deserialize, Serialize)]
        pub struct $key($typ);

        impl<T: Into<$typ>> From<T> for $key {
            fn from(value: T) -> Self {
                $key(value.into())
            }
        }

        impl $crate::model::key::Key for $source {
            type Key = $key;

            fn key(&self) -> Self::Key {
                $key(self.$target.clone())
            }
        }
    };
}

pub trait Key {
    type Key: Clone + Debug + Hash + Eq + PartialEq + Ord + PartialOrd;

    fn key(&self) -> Self::Key;
}
