use crate::{fmt::Format, BufReadExt, ReadFrom, ReadFromError};
use std::fmt::{self, Display};

macro_rules! impl_read_into_for_tuple {
    ($e:ident) => {
        use std::convert::Infallible as $e;
        impl ReadFrom for () {
            type ParseError = $e;
            #[inline]
            fn try_read_from<F: Format, S: BufReadExt>(_stream: &mut S, _format: F) -> Result<(), ReadFromError<Self>> {
                Ok(())
            }
        }
    };
    ($e:ident $($t:ident)+) => {
        #[derive(Debug)]
        pub enum $e<$($t, )+ > {
            $($t($t), )+
        }
        impl<$($t: std::error::Error, )+ > Display for $e<$($t, )+ > {
            #[inline]
            fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
                match self { $( Self::$t(err) => Display::fmt(err, f), )+ }
            }
        }
        impl<$($t: std::error::Error, )+ > std::error::Error for $e<$($t, )+ > {}
        impl<$($t: ReadFrom, )+> ReadFrom for ( $($t, )+ ) {
            type ParseError = $e<$(<$t as ReadFrom>::ParseError, )+>;
            #[inline]
            fn try_read_from<F: Format, S: BufReadExt>(stream: &mut S, format: F) -> Result<($($t, )+), ReadFromError<Self>> {
                Ok(( $(<$t as ReadFrom>::try_read_from(stream, format).map_err(|err| match err {
                    ReadFromError::<$t>::IOError(e) => ReadFromError::<Self>::IOError(e),
                    ReadFromError::<$t>::EOF => ReadFromError::<Self>::EOF,
                    ReadFromError::<$t>::EOL => ReadFromError::<Self>::EOL,
                    ReadFromError::<$t>::FromStrError(e, s, n) => ReadFromError::<Self>::FromStrError($e::$t(e), s, n),
                })?, )+ ))
            }
        }
    };
}

impl_read_into_for_tuple!(Tuple0Error);
impl_read_into_for_tuple!(Tuple1Error T1);
impl_read_into_for_tuple!(Tuple2Error T1 T2);
impl_read_into_for_tuple!(Tuple3Error T1 T2 T3);
impl_read_into_for_tuple!(Tuple4Error T1 T2 T3 T4);
impl_read_into_for_tuple!(Tuple5Error T1 T2 T3 T4 T5);
impl_read_into_for_tuple!(Tuple6Error T1 T2 T3 T4 T5 T6);
impl_read_into_for_tuple!(Tuple7Error T1 T2 T3 T4 T5 T6 T7);
impl_read_into_for_tuple!(Tuple8Error T1 T2 T3 T4 T5 T6 T7 T8);
impl_read_into_for_tuple!(Tuple9Error T1 T2 T3 T4 T5 T6 T7 T8 T9);
impl_read_into_for_tuple!(Tuple10Error T1 T2 T3 T4 T5 T6 T7 T8 T9 T10);
impl_read_into_for_tuple!(Tuple11Error T1 T2 T3 T4 T5 T6 T7 T8 T9 T10 T11);
impl_read_into_for_tuple!(Tuple12Error T1 T2 T3 T4 T5 T6 T7 T8 T9 T10 T11 T12);
