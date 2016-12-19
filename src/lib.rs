//! An `Either` enum over N different types.
//!
//! Thanks to @bluss for their `Either` crate.

#![cfg_attr(not(feature = "use_std"), no_std)]

#[cfg(not(feature = "use_std"))]
extern crate core as std;

use std::fmt;
use std::iter;
use std::ops::Deref;
use std::ops::DerefMut;
#[cfg(feature = "use_std")]
use std::io::{self, Write, Read, BufRead};
#[cfg(feature = "use_std")]
use std::error::Error;

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

            pub fn as_mut(&mut self) -> $enum_name_head<&mut $n_titlecase_head, $( &mut $n_titlecase_tail ),*> {
                match *self {
                    $enum_name_head::$n_titlecase_head(ref mut value) =>
                        $enum_name_head::$n_titlecase_head(value),
                    $(
                        $enum_name_head::$n_titlecase_tail(ref mut value) =>
                            $enum_name_head::$n_titlecase_tail(value),
                    )*
                }
            }
        }

        impl<$n_titlecase_head, $( $n_titlecase_tail ),*> Iterator for
            $enum_name_head<$n_titlecase_head, $( $n_titlecase_tail ),*>
            where $n_titlecase_head: Iterator,
            $( $n_titlecase_tail: Iterator<Item=$n_titlecase_head::Item> ),* {

            type Item = $n_titlecase_head::Item;

            fn next(&mut self) -> Option<Self::Item> {
                match *self {
                    $enum_name_head::$n_titlecase_head(ref mut value) => value.next(),
                    $(
                        $enum_name_head::$n_titlecase_tail(ref mut value) => value.next(),
                    )*
                }
            }

            fn size_hint(&self) -> (usize, Option<usize>) {
                match *self {
                    $enum_name_head::$n_titlecase_head(ref value) => value.size_hint(),
                    $(
                        $enum_name_head::$n_titlecase_tail(ref value) => value.size_hint(),
                    )*
                }
            }

            fn fold<Acc, G>(self, init: Acc, f: G) -> Acc
                where G: FnMut(Acc, Self::Item) -> Acc,
            {
                match self {
                    $enum_name_head::$n_titlecase_head(value) => value.fold(init, f),
                    $(
                        $enum_name_head::$n_titlecase_tail(value) => value.fold(init, f),
                    )*
                }
            }

            fn count(self) -> usize {
                match self {
                    $enum_name_head::$n_titlecase_head(value) => value.count(),
                    $(
                        $enum_name_head::$n_titlecase_tail(value) => value.count(),
                    )*
                }
            }

            fn last(self) -> Option<Self::Item> {
                match self {
                    $enum_name_head::$n_titlecase_head(value) => value.last(),
                    $(
                        $enum_name_head::$n_titlecase_tail(value) => value.last(),
                    )*
                }
            }

            fn nth(&mut self, n: usize) -> Option<Self::Item> {
                match *self {
                    $enum_name_head::$n_titlecase_head(ref mut value) => value.nth(n),
                    $(
                        $enum_name_head::$n_titlecase_tail(ref mut value) => value.nth(n),
                    )*
                }
            }

            fn collect<B>(self) -> B
                where B: iter::FromIterator<Self::Item>
            {
                match self {
                    $enum_name_head::$n_titlecase_head(value) => value.collect(),
                    $(
                        $enum_name_head::$n_titlecase_tail(value) => value.collect(),
                    )*
                }
            }

            fn all<F>(&mut self, f: F) -> bool
                where F: FnMut(Self::Item) -> bool
            {
                match *self {
                    $enum_name_head::$n_titlecase_head(ref mut value) => value.all(f),
                    $(
                        $enum_name_head::$n_titlecase_tail(ref mut value) => value.all(f),
                    )*
                }
            }
        }

        impl<$n_titlecase_head, $( $n_titlecase_tail ),*> DoubleEndedIterator for
            $enum_name_head<$n_titlecase_head, $( $n_titlecase_tail ),*>
            where $n_titlecase_head: DoubleEndedIterator,
            $( $n_titlecase_tail: DoubleEndedIterator<Item=$n_titlecase_head::Item> ),*
        {
            fn next_back(&mut self) -> Option<Self::Item> {
                match *self {
                    $enum_name_head::$n_titlecase_head(ref mut value) => value.next_back(),
                    $(
                        $enum_name_head::$n_titlecase_tail(ref mut value) => value.next_back(),
                    )*
                }
            }
        }

        impl<$n_titlecase_head, $( $n_titlecase_tail ),*> ExactSizeIterator for
            $enum_name_head<$n_titlecase_head, $( $n_titlecase_tail ),*>
            where $n_titlecase_head: ExactSizeIterator,
            $( $n_titlecase_tail: ExactSizeIterator<Item=$n_titlecase_head::Item> ),*
        {
        }

        #[cfg(feature = "use_std")]
        impl<$n_titlecase_head, $( $n_titlecase_tail ),*> Read for
            $enum_name_head<$n_titlecase_head, $( $n_titlecase_tail ),*>
            where $n_titlecase_head: Read, $( $n_titlecase_tail: Read ),*
        {
            fn read(&mut self, buf: &mut [u8]) -> io::Result<usize> {
                match *self {
                    $enum_name_head::$n_titlecase_head(ref mut value) => value.read(buf),
                    $( $enum_name_head::$n_titlecase_tail(ref mut value) => value.read(buf) ),*
                }
            }

            fn read_to_end(&mut self, buf: &mut Vec<u8>) -> io::Result<usize> {
                match *self {
                    $enum_name_head::$n_titlecase_head(ref mut value) => value.read_to_end(buf),
                    $( $enum_name_head::$n_titlecase_tail(ref mut value) => value.read_to_end(buf) ),*
                }
            }
        }

        #[cfg(feature = "use_std")]
        impl<$n_titlecase_head, $( $n_titlecase_tail ),*> BufRead for
            $enum_name_head<$n_titlecase_head, $( $n_titlecase_tail ),*>
            where $n_titlecase_head: BufRead, $( $n_titlecase_tail: BufRead ),*
        {
            fn fill_buf(&mut self) -> io::Result<&[u8]> {
                match *self {
                    $enum_name_head::$n_titlecase_head(ref mut value) => value.fill_buf(),
                    $( $enum_name_head::$n_titlecase_tail(ref mut value) => value.fill_buf() ),*
                }
            }

            fn consume(&mut self, amt: usize) {
                match *self {
                    $enum_name_head::$n_titlecase_head(ref mut value) => value.consume(amt),
                    $( $enum_name_head::$n_titlecase_tail(ref mut value) => value.consume(amt) ),*
                }
            }
        }

        #[cfg(feature = "use_std")]
        impl<$n_titlecase_head, $( $n_titlecase_tail ),*> Write for
            $enum_name_head<$n_titlecase_head, $( $n_titlecase_tail ),*>
            where $n_titlecase_head: Write, $( $n_titlecase_tail: Write ),*
        {
            fn write(&mut self, buf: &[u8]) -> io::Result<usize> {
                match *self {
                    $enum_name_head::$n_titlecase_head(ref mut value) => value.write(buf),
                    $( $enum_name_head::$n_titlecase_tail(ref mut value) => value.write(buf) ),*
                }
            }

            fn flush(&mut self) -> io::Result<()> {
                match *self {
                    $enum_name_head::$n_titlecase_head(ref mut value) => value.flush(),
                    $( $enum_name_head::$n_titlecase_tail(ref mut value) => value.flush() ),*
                }
            }
        }

        impl<Target, $n_titlecase_head, $( $n_titlecase_tail ),*> AsRef<Target> for
            $enum_name_head<$n_titlecase_head, $( $n_titlecase_tail ),*>
            where $n_titlecase_head: AsRef<Target>, $( $n_titlecase_tail: AsRef<Target> ),*
        {
            fn as_ref(&self) -> &Target {
                match *self {
                    $enum_name_head::$n_titlecase_head(ref value) => value.as_ref(),
                    $( $enum_name_head::$n_titlecase_tail(ref value) => value.as_ref() ),*
                }
            }
        }

        impl<Target, $n_titlecase_head, $( $n_titlecase_tail ),*> AsMut<Target> for
            $enum_name_head<$n_titlecase_head, $( $n_titlecase_tail ),*>
            where $n_titlecase_head: AsMut<Target>, $( $n_titlecase_tail: AsMut<Target> ),*
        {
            fn as_mut(&mut self) -> &mut Target {
                match *self {
                    $enum_name_head::$n_titlecase_head(ref mut value) => value.as_mut(),
                    $( $enum_name_head::$n_titlecase_tail(ref mut value) => value.as_mut() ),*
                }
            }
        }

        impl<$n_titlecase_head, $( $n_titlecase_tail ),*> Deref for
            $enum_name_head<$n_titlecase_head, $( $n_titlecase_tail ),*>
            where $n_titlecase_head: Deref, $( $n_titlecase_tail: Deref<Target=$n_titlecase_head::Target> ),*
        {
            type Target = $n_titlecase_head::Target;

            fn deref(&self) -> &Self::Target {
                match *self {
                    $enum_name_head::$n_titlecase_head(ref value) => value.deref(),
                    $( $enum_name_head::$n_titlecase_tail(ref value) => value.deref() ),*
                }
            }
        }

        impl<$n_titlecase_head, $( $n_titlecase_tail ),*> DerefMut for
            $enum_name_head<$n_titlecase_head, $( $n_titlecase_tail ),*>
            where $n_titlecase_head: DerefMut, $( $n_titlecase_tail: DerefMut<Target=$n_titlecase_head::Target> ),*
        {
            fn deref_mut(&mut self) -> &mut Self::Target {
                match *self {
                    $enum_name_head::$n_titlecase_head(ref mut value) => value.deref_mut(),
                    $( $enum_name_head::$n_titlecase_tail(ref mut value) => value.deref_mut() ),*
                }
            }
        }

        #[cfg(feature = "use_std")]
        impl<$n_titlecase_head, $( $n_titlecase_tail ),*> Error for
            $enum_name_head<$n_titlecase_head, $( $n_titlecase_tail ),*>
            where $n_titlecase_head: Error, $( $n_titlecase_tail: Error ),*
        {
            fn description(&self) -> &str {
                match *self {
                    $enum_name_head::$n_titlecase_head(ref value) => value.description(),
                    $( $enum_name_head::$n_titlecase_tail(ref value) => value.description() ),*
                }
            }

            fn cause(&self) -> Option<&Error> {
                match *self {
                    $enum_name_head::$n_titlecase_head(ref value) => value.cause(),
                    $( $enum_name_head::$n_titlecase_tail(ref value) => value.cause() ),*
                }
            }
        }

        impl<$n_titlecase_head, $( $n_titlecase_tail ),*> fmt::Display for
            $enum_name_head<$n_titlecase_head, $( $n_titlecase_tail ),*>
            where $n_titlecase_head: fmt::Display, $( $n_titlecase_tail: fmt::Display ),*
        {
            fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
                match *self {
                    $enum_name_head::$n_titlecase_head(ref value) => {
                        value.fmt(f)
                    },
                    $( $enum_name_head::$n_titlecase_tail(ref value) => {
                        value.fmt(f)
                    } ),*
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
    (Either2, Two, two),
    (Single, One, one),
);

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn it_works() {
        let either7 = match 1 {
            0 => Either7::One("a"),
            1 => Either7::Two(5u32),
            2 => Either7::Three("string".to_string()),
            3 => Either7::Four([1,2,3]),
            4 => Either7::Five([1,2,3,4,5,6]),
            6 => Either7::Six(()),
            _ => Either7::Seven(([()])),
        };

        assert_eq!(either7.as_ref().two(), Some(&5u32));
        assert_eq!(either7.six(), None);
    }
}
