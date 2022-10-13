#[macro_export]
macro_rules! set_things {
    ($($($w:expr)?,$( $y:expr )?);*) => {
        {
            $(
                $($w)? = $($y)?;
            )*
        }
    };
    ( $( $x:stmt), * ) => {
        {
            $(
                $x;
            )*
        }
    };
}
#[macro_export]
macro_rules! create_fn {
    () => (fn xfunc() { });
}

#[macro_export]
macro_rules! print_vars {
    (x => $e:expr) => (println!("mode X: {}", $e));
    (y => $e:expr) => (println!("mode Y: {}", $e));
}
#[macro_export]
macro_rules! bit_more_complicated {
    (
        $(
            $x:expr; [ $( $y:expr ),* ]
        );*
    ) => {
        &[ $($( $x + $y ),*),* ]
    }
}

#[macro_export]
macro_rules! call_trait {
    ( $( $x:expr)? ) => {
        {
            $($x)?.a_number()
        }
    };
}