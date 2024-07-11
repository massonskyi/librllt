pub mod fabric;
pub mod timeit;
pub mod functor;

macro_rules! ifndef {
    ($name:ident, $body:block) => {
        #[cfg(not(feature = stringify!($name)))]
        {
            $body
        }
    };
}

macro_rules! endif {
    () => {};
}

// macro_rules! define {
//     ($name:ident: $T:ty, $value:expr) => {
//         ifndef!($name, {
//             pub const $name: $T = $value;
//         });
//     };
//     ($name:ident, $value:expr) => {
//         ifndef!($name, {
//             pub static $name: impl Fn() -> _ = move || $value;
//         });
//     };
//     ($name:ident($($args:tt)*) $body:block) => {
//         ifndef!($name, {
//             pub fn $name($($args)*) $body
//         });
//     };
//     ($name:ident, $struct_name:ident { $($field:ident: $field_type:ty = $value:expr),* }) => {
//         ifndef!($name, {
//             pub struct $struct_name {
//                 $(pub $field: $field_type),*
//             }
//             pub static $name: $struct_name = $struct_name {
//                 $($field: $value),*
//             };
//         });
//     };
// }
macro_rules! define {
    // Константы
    ($name:ident: $T:ty, $value:expr) => {
        pub const $name: $T = $value;
    };

    // Функции без параметров
    ($name:ident, $body:block) => {
        use std::any::Any;
        pub fn $name() -> Box<dyn Any> {
            Box::new($body)
        }
    };

    // Функции с параметрами
    ($name:ident, $($arg:ident : $arg_ty:ty),* => $body:block) => {
        use std::any::Any;
        pub fn $name($($arg : $arg_ty),*) -> Box<dyn Any> {
            Box::new($body)
        }
    };

    // // Лямбды
    // ($name:ident, | $($arg:ident : $arg_ty:ty),* | $body:block) => {
    //     pub static $name: &'static (dyn Fn($($arg_ty),*) -> Box<dyn Any> + Sync + Send) = &(|$($arg : $arg_ty),*| Box::new($body));
    // };


    // Структуры
    ($name:ident, $struct_name:ident { $($field:ident : $field_ty:ty = $value:expr),* }) => {
        pub struct $struct_name {
            $(pub $field: $field_ty),*
        }

        pub static $name: $struct_name = $struct_name {
            $($field: $value),*
        };
    };
}

macro_rules! printf {
    ($($args:tt)*) => {{
        print!("{}", format!($($args)*));
        io::stdout().flush().unwrap();
    }};
}

#[cfg(debug_assertions)]
macro_rules! debug {
    ($($args:tt)*) => {
        println!("[DEBUG]: {}", format!($($args)*));
    };
}

#[cfg(not(debug_assertions))]
macro_rules! debug {
    ($($args:tt)*) => {};
}
macro_rules! logger {
    ($($args:tt)*) => {{
        let mut file = File::create(Path::new("log.txt")).unwrap();
        writeln!(file, "{}", format!($($args)*)).unwrap();
    }};
}

macro_rules! timer {
    ($name:ident, $code:block) => {{
        let start = Instant::now();
        let result = $code;
        let duration = start.elapsed();
        println!("[TIMER {}]: {} ms", stringify!($name), duration.as_millis());
        result
    }};
}
macro_rules! try_except {
    ($code:block, $except:expr) => {
        match $code {
            Ok(value) => value,
            Err(error) => {
                println!("[ERROR]: {}", error);
                $except
            }
        }
    };
}
macro_rules! assert_eq_msg {
    ($left:expr, $right:expr, $msg:expr) => {
        if $left != $right {
            panic!("[ASSERTION FAILED]: {} (left: {}, right: {})", $msg, $left, $right);
        }
    };
}
macro_rules! unwrap_or_else_msg {
    ($result:expr, $default:expr, $msg:expr) => {
        match $result {
            Ok(value) => value,
            Err(_) => {
                println!("[ERROR]: {}", $msg);
                $default
            }
        }
    };
}

macro_rules! vec {
    ($($x:expr),*) => {
        {
            let mut temp_vec = Vec::new();
            $(
                temp_vec.push($x);
            )*
            temp_vec
        }
    };
}

macro_rules! concat {
    ($($x:tt)*) => {
        {
            let mut result = String::new();
            $(
                result.push_str(&format!($x).to_string());
            )*
            result
        }
    };
}

macro_rules! repeat {
    ($n:expr, $code:block) => {
        for _ in 0..$n {
            $code
        }
    };
}

macro_rules! range {
    ($start:expr, $end:expr) => {
        (0..$end - $start).map(|i| i + $start)
    };
}

macro_rules! range_inclusive {
    ($start:expr, $end:expr) => {
        (0..=$end - $start).map(|i| i + $start)
    };
}

macro_rules! range_inclusive_step {
    ($start:expr, $end:expr, $step:expr) => {
        (0..=$end - $start).step_by($step).map(|i| i + $start)
    };
}

macro_rules! range_step {
    ($start:expr, $end:expr, $step:expr) => {
        (0..$end - $start).step_by($step).map(|i| i + $start)
    };
}

macro_rules! range_step_inclusive {
    ($start:expr, $end:expr, $step:expr) => {
        (0..=$end - $start).step_by($step).map(|i| i + $start)
    };
}

macro_rules! benchmark {
    ($name:ident, $code:block) => {{
        let start = Instant::now();
        $code;
        let duration = start.elapsed();
        println!("[BENCHMARK {}]: {} ms", stringify!($name), duration.as_millis());
    }};
}
macro_rules! unreachable_msg {
    ($msg:expr) => {
        panic!("[UNREACHABLE CODE]: {}", $msg);
    };
}
macro_rules! stringify_type {
    ($t:ty) => {
        stringify!($t)
    };
}
macro_rules! map {
    ($($key:expr => $value:expr),*) => {
        {
            let mut temp_map = std::collections::HashMap::new();
            $(
                temp_map.insert($key, $value);
            )*
            temp_map
        }
    };
}
macro_rules! todo {
    ($msg:expr) => {
        compile_error!(concat!("TODO: ", $msg));
    };
}
macro_rules! measure {
    ($code:block) => {{
        let start = Instant::now();
        let result = $code;
        let duration = start.elapsed();
        (result, duration)
    }};
}
macro_rules! assert_approx_eq {
    ($left:expr, $right:expr, $tolerance:expr) => {
        if ($left - $right).abs() > $tolerance {
            panic!("[ASSERTION FAILED]: {} is not approximately equal to {} (tolerance: {})", $left, $right, $tolerance);
        }
    };
}
macro_rules! enum_iter {
    ($name:ident) => {
        impl $name {
            pub fn iter() -> impl Iterator<Item = $name> {
                let mut index = 0;
                std::iter::from_fn(move || {
                    let value = $name::from_u32(index);
                    index += 1;
                    value
                })
            }
        }
    };
}
macro_rules! tuple_iter {
    ($name:ident) => {
        impl<$($T: Copy),*> $name<$($T),*> {
            pub fn iter(&self) -> impl Iterator<Item = &$T> {
                let mut index = 0;
                std::iter::from_fn(move || {
                    let value = &self.0;
                    index += 1;
                    value
                })
            }
        }
    };
}
macro_rules! assert_matches {
    ($expr:expr, $pattern:pat) => {
        match $expr {
            $pattern => {},
            ref value => {
                panic!("assertion failed: `(left matches right)`\n  left: `{:?}`\n right: `{}`", value, stringify!($pattern));
            }
        }
    };
}
macro_rules! assert_panics {
    ($code:block) => {
        let result = std::panic::catch_unwind(std::panic::AssertUnwindSafe(|| $code));
        assert!(result.is_err());
    };
}
macro_rules! assert_eq_float {
    ($left:expr, $right:expr, $tolerance:expr) => {
        let left = $left as f64;
        let right = $right as f64;
        let tolerance = $tolerance as f64;
        if (left - right).abs() > tolerance {
            panic!("assertion failed: `(left == right)`\n  left: `{:?}`\n right: `{:?}`\n tolerance: `{:?}`", left, right, tolerance);
        }
    };
}
macro_rules! assert_contains {
    ($container:expr, $element:expr) => {
        if !$container.contains(&$element) {
            panic!("assertion failed: `(container contains element)`\n  container: `{:?}`\n element: `{:?}`", $container, $element);
        }
    };
}

macro_rules! assert_empty {
    ($container:expr) => {
        if !$container.is_empty() {
            panic!("assertion failed: `(container is empty)`\n  container: `{:?}`", $container);
        }
    };
}
