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
    (op_not => $arg1:ident) => {
        (
            $crate::runtime::core::ValueKey::Symbol($crate::runtime::core::symbol::common::operators::OP_NOT),
            $crate::runtime::core::Value::new($crate::runtime::lib::Func::new_func1(op_not)),
        )
    };

    (op_neg => $arg1:ident) => {
        (
            $crate::runtime::core::ValueKey::Symbol($crate::runtime::core::symbol::common::operators::OP_NEG),
            $crate::runtime::core::Value::new($crate::runtime::lib::Func::new_func1(op_neg)),
        )
    };

    (op_mul => $arg1:ident, $arg2:ident) => {
        (
            $crate::runtime::core::ValueKey::Symbol($crate::runtime::core::symbol::common::operators::OP_MUL),
            $crate::runtime::core::Value::new($crate::runtime::lib::Func::new_func2(op_mul)),
        )
    };

    (op_div => $arg1:ident, $arg2:ident) => {
        (
            $crate::runtime::core::ValueKey::Symbol($crate::runtime::core::symbol::common::operators::OP_DIV),
            $crate::runtime::core::Value::new($crate::runtime::lib::Func::new_func2(op_div)),
        )
    };

    (op_rem => $arg1:ident, $arg2:ident) => {
        (
            $crate::runtime::core::ValueKey::Symbol($crate::runtime::core::symbol::common::operators::OP_REM),
            $crate::runtime::core::Value::new($crate::runtime::lib::Func::new_func2(op_rem)),
        )
    };

    (op_add => $arg1:ident, $arg2:ident) => {
        (
            $crate::runtime::core::ValueKey::Symbol($crate::runtime::core::symbol::common::operators::OP_ADD),
            $crate::runtime::core::Value::new($crate::runtime::lib::Func::new_func2(op_add)),
        )
    };

    (op_sub => $arg1:ident, $arg2:ident) => {
        (
            $crate::runtime::core::ValueKey::Symbol($crate::runtime::core::symbol::common::operators::OP_SUB),
            $crate::runtime::core::Value::new($crate::runtime::lib::Func::new_func2(op_sub)),
        )
    };

    (op_eq => $arg1:ident, $arg2:ident) => {
        (
            $crate::runtime::core::ValueKey::Symbol($crate::runtime::core::symbol::common::operators::OP_EQ),
            $crate::runtime::core::Value::new($crate::runtime::lib::Func::new_func2(op_eq)),
        )
    };

    (op_ne => $arg1:ident, $arg2:ident) => {
        (
            $crate::runtime::core::ValueKey::Symbol($crate::runtime::core::symbol::common::operators::OP_NE),
            $crate::runtime::core::Value::new($crate::runtime::lib::Func::new_func2(op_ne)),
        )
    };

    (op_gt => $arg1:ident, $arg2:ident) => {
        (
            $crate::runtime::core::ValueKey::Symbol($crate::runtime::core::symbol::common::operators::OP_GT),
            $crate::runtime::core::Value::new($crate::runtime::lib::Func::new_func2(op_gt)),
        )
    };

    (op_lt => $arg1:ident, $arg2:ident) => {
        (
            $crate::runtime::core::ValueKey::Symbol($crate::runtime::core::symbol::common::operators::OP_LT),
            $crate::runtime::core::Value::new($crate::runtime::lib::Func::new_func2(op_lt)),
        )
    };

    (op_gte => $arg1:ident, $arg2:ident) => {
        (
            $crate::runtime::core::ValueKey::Symbol($crate::runtime::core::symbol::common::operators::OP_GTE),
            $crate::runtime::core::Value::new($crate::runtime::lib::Func::new_func2(op_gte)),
        )
    };

    (op_lte => $arg1:ident, $arg2:ident) => {
        (
            $crate::runtime::core::ValueKey::Symbol($crate::runtime::core::symbol::common::operators::OP_LTE),
            $crate::runtime::core::Value::new($crate::runtime::lib::Func::new_func2(op_lte)),
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
        impl $ty_ident:ident for $ty:ident as $ty_name:expr;
        $(constructor(&self, $constructor_args:ident: $constructor_args_ty:ty) $constructor_body:block)?
        $(fn $name:ident ($($arg:ident : $typ:ty),*) $(-> $ret:ty)? $rest:block)*
    ) => {
        #[derive(Debug)]
        pub(crate) struct $ty_ident {
            name: $crate::runtime::core::Symbol,
            members: std::collections::HashMap<$crate::runtime::core::ValueKey, $crate::runtime::core::Value>
        }

        paste::paste! {
            thread_local! {
                #[allow(non_upper_case_globals)]
                static [<$ty_ident _TYPE>]: std::rc::Rc<$ty_ident> = Default::default();
            }
        }

        impl $ty_ident {
            pub const NAME: $crate::runtime::core::Symbol = $crate::runtime::core::Symbol::new_static($ty_name);

            pub fn instance() -> std::rc::Rc<Self> {
                paste::paste! {
                    [<$ty_ident _TYPE>].with(Clone::clone)
                }
            }
        }

        impl Default for $ty_ident {
            fn default() -> Self {
                Self {
                    name: Self::NAME,
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
                fn construct(&self, $constructor_args: $constructor_args_ty) -> Result<$crate::runtime::core::Value, $crate::runtime::error::RuntimeError>
                $constructor_body
            )?

            fn name(&self) -> &$crate::runtime::core::Symbol {
                &self.name
            }

            fn impl_names(&self) -> &[&$crate::runtime::core::Symbol] {
                &[]
            }

            fn members(&self) -> &std::collections::HashMap<$crate::runtime::core::ValueKey, $crate::runtime::core::Value> {
                &self.members
            }
        }

        impl $crate::runtime::core::TypeInstanceBase for $ty {
            fn as_any(&self) -> &dyn std::any::Any {
                self
            }

            fn instance_type(&self) -> Rc<dyn $crate::runtime::core::Type> {
                $ty_ident::instance()
            }

            fn reflect_type() -> Rc<dyn $crate::runtime::core::Type> {
                $ty_ident::instance()
            }
        }

        $(
            fn $name ($($arg : $typ),*) $(-> $ret)?  $rest
        )*
    };
}
