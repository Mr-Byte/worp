#[macro_export]
#[doc(hidden)]
macro_rules! match_type {
    (@value=$value:expr => _ => $body:expr ) => {{
        $body
    }};

    (@value=$value:expr => $command:ident : $typ:ty => $body:expr, $($rest:tt)*) => {
        if let Some($command) = $value.value::<$typ>() {
            $body
        } else {
            match_type!(@value=$value => $($rest)*)
        }
    };

    ($value:expr, $command:ident : $typ:ty => $body:expr, $($rest:tt)* ) => {{
        if let Some($command) = $value.value::<$typ>() {
            $body
        } else {
            match_type!(@value=$value => $($rest)*)
        }
    }};
}

#[macro_export]
#[doc(hidden)]
macro_rules! type_member {
    // Operators
    (op_add => $arg1:ident, $arg2:ident) => {
        (
            $crate::runtime::core::ValueKey::Symbol($crate::runtime::core::symbol::common::operators::OP_ADD),
            $crate::runtime::core::Value::new($crate::runtime::lib::Func::new_func2(op_add)),
        )
    };

    // Regular functions
    ($name:ident => ) => {
        (
            Into::<$crate::runtime::core::ValueKey>::into(stringify!($name)),
            $crate::runtime::core::Value::new($crate::runtime::lib::Func::new_func0($name)),
        )
    };
    ($name:ident => $arg1:ident) => {
        (
            Into::<$crate::runtime::core::ValueKey>::into(stringify!($name)),
            $crate::runtime::core::Value::new($crate::runtime::lib::Func::new_func1($name)),
        )
    };
    ($name:ident => $arg1:ident, $arg2:ident) => {
        (
            Into::<$crate::runtime::core::ValueKey>::into(stringify!($name)),
            $crate::runtime::core::Value::new($crate::runtime::lib::Func::new_func2($name)),
        )
    };
}

#[macro_export]
#[doc(hidden)]
macro_rules! decl_type {
    (
        type $ty_ident:ident = $ty_name:expr;
        $(fn construct(&self, args: &[$crate::runtime::core::Value]) -> $crate::runtime::core::Value $constructor_body:block)?
        $(fn $name:ident ($($arg:ident : $typ:ty),*) $(-> $ret:ty)? $rest:block)*
    ) => {
        #[derive(Debug)]
        struct $ty_ident {
            name: $crate::runtime::core::Symbol,
            members: std::collections::HashMap<$crate::runtime::core::ValueKey, $crate::runtime::core::Value>
        }

        impl Default for $ty_ident {
            fn default() -> Self {
                Self {
                    name: Symbol::new($ty_name),
                    members: {
                        vec! [
                            $(type_member!($name => $($arg),*),)*
                        ].into_iter().collect::<_>()
                    }
                }
            }
        }

        impl $crate::runtime::core::Type for $ty_ident {
            $(
                fn construct(&self, args: &[$crate::runtime::core::Value]) -> $crate::runtime::core::Value
                $constructor_body
            )?

            fn name(&self) -> &$crate::runtime::core::Symbol {
                &self.name
            }

            fn impl_names(&self) -> &[&$crate::runtime::core::Symbol] {
                &[]
            }

            fn members(&self) -> &HashMap<$crate::runtime::core::ValueKey, $crate::runtime::core::Value> {
                &self.members
            }
        }

        $(
            fn $name ($($arg : $typ),*) $(-> $ret)?  $rest
        )*
    };
}
