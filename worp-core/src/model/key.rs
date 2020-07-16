use std::{fmt::Debug, hash::Hash};

macro_rules! key {
    ($key: ident, $source: ident => $target: ident) => {
        #[derive(Clone, Debug, Hash, Eq, PartialEq, Ord, PartialOrd, Deserialize, Serialize)]
        pub struct $key(String);

        impl<T: AsRef<str>> From<T> for $key {
            fn from(value: T) -> Self {
                $key(String::from(value.as_ref()))
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
    type Key: for<'de> Clone + Debug + Hash + Eq + PartialEq + Ord + PartialOrd;

    fn key(&self) -> Self::Key;
}
