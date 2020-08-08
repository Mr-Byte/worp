#[macro_export]
macro_rules! match_type {
    (@rest=$value:expr => _ => $body:expr ) => {{
        $body
    }};

    (@rest=$value:expr => $command:ident : $typ:ty => $body:expr, $($rest:tt)*) => {
        if let Some($command) = $value.value::<$typ>() {
            $body
        } else {
            match_type!(@rest=$value => $($rest)*)
        }
    };

    ($value:expr, $command:ident : $typ:ty => $body:expr, $($rest:tt)* ) => {{
        if let Some($command) = $value.value::<$typ>() {
            $body
        } else {
            match_type!(@rest=$value => $($rest)*)
        }
    }};
}
