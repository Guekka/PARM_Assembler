// Mostly from https://stackoverflow.com/a/70470443

pub trait Appliable<Args> {
    type Ret;
    fn make_appliable(&self) -> Box<dyn Fn(Args) -> Self::Ret + '_>;
}

macro_rules! impl_make_appliable {
    // Empty case
    () => {};
    ($first_generic:ident $($other_generics:ident)*) => {
        impl_make_appliable!($($other_generics)*);

        impl<$first_generic, $($other_generics,)* Ret, Func>
            Appliable<($first_generic, $($other_generics,)*)>
            for Func
        where
            Func: Fn($first_generic, $($other_generics,)*) -> Ret,
        {
            type Ret = Ret;
            #[allow(non_snake_case)]
            fn make_appliable(&self) -> Box<dyn Fn(($first_generic, $($other_generics,)*)) -> Self::Ret + '_> {
                        Box::new(move |($first_generic, $($other_generics,)*)| self($first_generic, $($other_generics,)*))
            }
        }
    };
}

impl_make_appliable!(A B C D E F G H I J K L M);

pub fn unescape_string(input: &str) -> String {
    input.replace("\\n", "\n").replace("\\\\", "\\")
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_tuple_args3() {
        let raw_fun = |a: i32, b: i32, c: i32| a + b + c;
        let fun = raw_fun.make_appliable();
        assert_eq!(fun((1, 2, 3)), raw_fun(1, 2, 3));
    }
}
