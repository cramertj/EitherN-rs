//! An `Either` enum over N different types.
//!
//! Thanks to @bluss for their `Either` crate.

// TODO: use to property order generic type parameters
macro_rules! reverse_idents {
    () => {};

    ($head:ident) => { $head };

    ($head:ident, $( $tail:ident ),*) => {
        reverse_idents!($( $tail ),*), $head
    };
}

macro_rules! impl_enums {
    () => {};

    (($enum_name_head:ident, $n_titlecase_head:ident, $n_lowercase_head:ident),
    $(($enum_name_tail:ident, $n_titlecase_tail:ident, $n_lowercase_tail:ident),)*) => {
        /// `EitherN` is an enum containing a value of one of `N` possible types
        #[derive(Copy, Clone, PartialEq, Eq, PartialOrd, Ord, Hash, Debug)]
        pub enum $enum_name_head<$n_titlecase_head, $( $n_titlecase_tail ),*> {
            $n_titlecase_head($n_titlecase_head),
            $( $n_titlecase_tail($n_titlecase_tail) ),*
        }

        impl<$n_titlecase_head, $( $n_titlecase_tail ),*>
            $enum_name_head<$n_titlecase_head, $( $n_titlecase_tail ),*> {

            pub fn $n_lowercase_head(self) -> Option<$n_titlecase_head> {
                match self {
                    $enum_name_head::$n_titlecase_head(value) => Some(value),
                    // Necessary to manually match to avoid impossible match
                    // for `_ => None` on `Single` (aka `Either1`)
                    $( $enum_name_head::$n_titlecase_tail(_) => None ),*
                }
            }

            $(
                pub fn $n_lowercase_tail(self) -> Option<$n_titlecase_tail> {
                    match self {
                        $enum_name_head::$n_titlecase_tail(value) => Some(value),
                        _ => None,
                    }
                }
            )*

            pub fn as_ref(&self) -> $enum_name_head<&$n_titlecase_head, $( &$n_titlecase_tail ),*> {
                match *self {
                    $enum_name_head::$n_titlecase_head(ref value) =>
                        $enum_name_head::$n_titlecase_head(value),
                    $(
                        $enum_name_head::$n_titlecase_tail(ref value) =>
                            $enum_name_head::$n_titlecase_tail(value),
                    )*
                }
            }
        }

        impl_enums!($( ($enum_name_tail, $n_titlecase_tail, $n_lowercase_tail), )*);
    }
}

impl_enums!(
    (Either8, Eight, eight),
    (Either7, Seven, seven),
    (Either6, Six, six),
    (Either5, Five, five),
    (Either4, Four, four),
    (Either3, Three, three),
    (Either, Two, two),
    (Single, One, one),
);

#[cfg(test)]
mod tests {
    #[test]
    fn it_works() {
    }
}
