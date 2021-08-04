#[macro_export]
macro_rules! newtype {
    (@__impl $ty:path => $name:ident) => {
        impl std::ops::Deref for $name {
            type Target = $ty;

            fn deref(&self) -> &Self::Target {
                &self.0
            }
        }

        impl $name {
            #[allow(dead_code)]
            pub fn into_inner(self) -> $ty {
                self.0
            }
        }
    };
    (@__prefix $($tokens:tt)+) => {
        #[cfg_attr(feature = "serde", derive(serde::Deserialize, serde::Serialize))]
        #[derive(Debug, Clone, PartialEq, Eq, PartialOrd, Ord, std::hash::Hash)]
        #[repr(transparent)]
        $($tokens)*;
    };
    ($ty:path => pub $name:ident) => {
        $crate::newtype!(@__prefix pub struct $name($ty));
        $crate::newtype!(@__impl $ty => $name);
    };
    ($ty:path => pub ($($vis:tt)+) $name:ident) => {
        $crate::newtype!(@__prefix pub($($vis)+) struct $name($ty));
        $crate::newtype!(@__impl $ty => $name);
    };
    ($ty:path => $name:ident) => {
        $crate::newtype!(@__prefix struct $name($ty));
        $crate::newtype!(@__impl $ty => $name);
    }
}

#[macro_export]
macro_rules! newtype_copy {
    (@__impl $ty:path => $name:ident) => {
        impl std::ops::Deref for $name {
            type Target = $ty;

            fn deref(&self) -> &Self::Target {
                &self.0
            }
        }

        impl $name {
            #[allow(dead_code)]
            pub fn into_inner(self) -> $ty {
                self.0
            }
        }
    };
    (@__prefix $($tokens:tt)+) => {
        #[cfg_attr(feature = "serde", derive(serde::Deserialize, serde::Serialize))]
        #[derive(Debug, Copy, Clone, PartialEq, Eq, PartialOrd, Ord, std::hash::Hash)]
        #[repr(transparent)]
        $($tokens)*;
    };
    ($ty:path => pub $name:ident) => {
        $crate::newtype_copy!(@__prefix pub struct $name($ty));
        $crate::newtype_copy!(@__impl $ty => $name);
    };
    ($ty:path => pub ($($vis:tt)+) $name:ident) => {
        $crate::newtype_copy!(@__prefix pub($($vis)+) struct $name($ty));
        $crate::newtype_copy!(@__impl $ty => $name);
    };
    ($ty:path => $name:ident) => {
        $crate::newtype_copy!(@__prefix struct $name($ty));
        $crate::newtype_copy!(@__impl $ty => $name);
    }
}

#[cfg(feature = "uuid")]
#[macro_export]
macro_rules! uuid {
    (pub $name:ident) => {
        $crate::newtype_copy!(::uuid::Uuid => pub $name);
    };
    (pub ($($vis:tt)+) $name:ident) => {
        $crate::newtype_copy!(::uuid::Uuid => pub ($($vis)*) $name);
    };
    ($name:ident) => {
        $crate::newtype_copy!(::uuid::Uuid => $name);
    };
}

#[macro_export]
macro_rules! u8 {
    (pub $name:ident) => {
        $crate::newtype_copy!(u8 => pub $name);
    };
    (pub ($($vis:tt)+) $name:ident) => {
        $crate::newtype_copy!(u8 => pub ($($vis)*) $name);
    };
    ($name:ident) => {
        $crate::newtype_copy!(u8 => $name);
    };
}

#[macro_export]
macro_rules! u16 {
    (pub $name:ident) => {
        $crate::newtype_copy!(u16 => pub $name);
    };
    (pub ($($vis:tt)+) $name:ident) => {
        $crate::newtype_copy!(u16 => pub ($($vis)*) $name);
    };
    ($name:ident) => {
        $crate::newtype_copy!(u16 => $name);
    };
}

#[macro_export]
macro_rules! u32 {
    (pub $name:ident) => {
        $crate::newtype_copy!(u32 => pub $name);
    };
    (pub ($($vis:tt)+) $name:ident) => {
        $crate::newtype_copy!(u32 => pub ($($vis)*) $name);
    };
    ($name:ident) => {
        $crate::newtype_copy!(u32 => $name);
    };
}

#[macro_export]
macro_rules! u64 {
    (pub $name:ident) => {
        $crate::newtype_copy!(u64 => pub $name);
    };
    (pub ($($vis:tt)+) $name:ident) => {
        $crate::newtype_copy!(u64 => pub ($($vis)*) $name);
    };
    ($name:ident) => {
        $crate::newtype_copy!(u64 => $name);
    };
}

#[macro_export]
macro_rules! u128 {
    (pub $name:ident) => {
        $crate::newtype_copy!(u128 => pub $name);
    };
    (pub ($($vis:tt)+) $name:ident) => {
        $crate::newtype_copy!(u128 => pub ($($vis)*) $name);
    };
    ($name:ident) => {
        $crate::newtype_copy!(u128 => $name);
    };
}

#[macro_export]
macro_rules! usize {
    (pub $name:ident) => {
        $crate::newtype_copy!(usize => pub $name);
    };
    (pub ($($vis:tt)+) $name:ident) => {
        $crate::newtype_copy!(usize => pub ($($vis)*) $name);
    };
    ($name:ident) => {
        $crate::newtype_copy!(usize => $name);
    };
}

#[macro_export]
macro_rules! i8 {
    (pub $name:ident) => {
        $crate::newtype_copy!(i8 => pub $name);
    };
    (pub ($($vis:tt)+) $name:ident) => {
        $crate::newtype_copy!(i8 => pub ($($vis)*) $name);
    };
    ($name:ident) => {
        $crate::newtype_copy!(i8 => $name);
    };
}

#[macro_export]
macro_rules! i16 {
    (pub $name:ident) => {
        $crate::newtype_copy!(i16 => pub $name);
    };
    (pub ($($vis:tt)+) $name:ident) => {
        $crate::newtype_copy!(i16 => pub ($($vis)*) $name);
    };
    ($name:ident) => {
        $crate::newtype_copy!(i16 => $name);
    };
}

#[macro_export]
macro_rules! i32 {
    (pub $name:ident) => {
        $crate::newtype_copy!(i32 => pub $name);
    };
    (pub ($($vis:tt)+) $name:ident) => {
        $crate::newtype_copy!(i32 => pub ($($vis)*) $name);
    };
    ($name:ident) => {
        $crate::newtype_copy!(i32 => $name);
    };
}

#[macro_export]
macro_rules! i64 {
    (pub $name:ident) => {
        $crate::newtype_copy!(i64 => pub $name);
    };
    (pub ($($vis:tt)+) $name:ident) => {
        $crate::newtype_copy!(i64 => pub ($($vis)*) $name);
    };
    ($name:ident) => {
        $crate::newtype_copy!(i64 => $name);
    };
}

#[macro_export]
macro_rules! i128 {
    (pub $name:ident) => {
        $crate::newtype_copy!(i128 => pub $name);
    };
    (pub ($($vis:tt)+) $name:ident) => {
        $crate::newtype_copy!(i128 => pub ($($vis)*) $name);
    };
    ($name:ident) => {
        $crate::newtype_copy!(i128 => $name);
    };
}

#[macro_export]
macro_rules! isize {
    (pub $name:ident) => {
        $crate::newtype_copy!(isize => pub $name);
    };
    (pub ($($vis:tt)+) $name:ident) => {
        $crate::newtype_copy!(isize => pub ($($vis)*) $name);
    };
    ($name:ident) => {
        $crate::newtype_copy!(isize => $name);
    };
}

#[macro_export]
macro_rules! bytevec {
    (pub $name:ident) => {
        $crate::newtype!(Vec<u8> => pub $name);
    };
    (pub($($vis:tt)+) $name:ident) => {
        $crate::newtype!(Vec<u8> => pub ($($vis)*) $name);
    };
    ($name:ident) => {
        $crate::newtype!(Vec<u8> => $name);
    };

}

#[macro_export]
macro_rules! string {
    (@__impl $name:ident) => {
        impl std::ops::Deref for $name {
            type Target = str;

            fn deref(&self) -> &Self::Target {
                &self.0
            }
        }

        impl $name {
            #[allow(dead_code)]
            pub fn into_inner(self) -> String {
                self.0
            }
        }
    };
    (pub $name:ident) => {
        #[cfg_attr(feature = "serde", derive(serde::Deserialize, serde::Serialize))]
        #[derive(Debug, Clone, PartialEq, Eq, PartialOrd, Ord, std::hash::Hash)]
        #[repr(transparent)]
        pub struct $name(String);
        $crate::string!(@__impl $name);
    };
    (pub ($($vis:tt)+) $name:ident) => {
        #[cfg_attr(feature = "serde", derive(serde::Deserialize, serde::Serialize))]
        #[derive(Debug, Clone, PartialEq, Eq, PartialOrd, Ord, std::hash::Hash)]
        #[repr(transparent)]
        pub($($vis)+) struct $name(String);
        $crate::string!(@__impl $name);
    };
    ($name:ident) => {
        #[cfg_attr(feature = "serde", derive(serde::Deserialize, serde::Serialize))]
        #[derive(Debug, Clone, PartialEq, Eq, PartialOrd, Ord, std::hash::Hash)]
        #[repr(transparent)]
        struct $name(String);
        $crate::string!(@__impl $name);
    };
}

#[cfg(test)]
mod test {
    use crate as nova;

    #[test]
    fn create_string() {
        nova::string!(pub(crate) QuietScreaming);

        QuietScreaming("wow".into());
    }

    #[test]
    fn newtype_u32() {
        nova::newtype_copy!(u32 => MyU32);

        MyU32(32);
    }

    #[test]
    fn newtypes() {
        nova::u8!(pub(crate) A);
        nova::u16!(pub B);
        nova::u32!(pub C);
        nova::u64!(pub D);
        nova::u128!(pub E);
        nova::usize!(pub F);
        nova::i8!(pub A2);
        nova::i16!(pub B2);
        nova::i32!(pub C2);
        nova::i64!(pub D2);
        nova::i128!(pub E2);
        nova::isize!(pub F2);
        nova::string!(pub G);
        nova::bytevec!(pub(in super) H);
    }

    #[test]
    #[cfg(feature = "uuid")]
    fn create_uuid() {
        nova::uuid!(pub QuietScreaming);

        QuietScreaming(uuid::Uuid::new_v4());
    }
}
