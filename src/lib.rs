#[cfg(feature = "uuid")]
#[macro_export]
macro_rules! uuid {
    (@__impl $name:ident) => {
        impl std::ops::Deref for $name {
            type Target = uuid::Uuid;

            fn deref(&self) -> &Self::Target {
                &self.0
            }
        }

        impl $name {
            #[allow(dead_code)]
            pub fn into_inner(self) -> uuid::Uuid {
                self.0
            }
        }
    };
    (pub $name:ident) => {
        #[cfg_attr(feature = "serde", derive(serde::Deserialize, serde::Serialize))]
        #[derive(Debug, Clone, PartialEq, Eq, PartialOrd, Ord, std::hash::Hash)]
        #[repr(transparent)]
        pub struct $name(uuid::Uuid);
        $crate::uuid!(@__impl $name);
    };
    ($name:ident) => {
        #[cfg_attr(feature = "serde", derive(serde::Deserialize, serde::Serialize))]
        #[derive(Debug, Clone, PartialEq, Eq, PartialOrd, Ord, std::hash::Hash)]
        #[repr(transparent)]
        struct $name(uuid::Uuid);
        $crate::uuid!(@__impl $name);
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
        nova::string!(pub QuietScreaming);

        QuietScreaming("wow".into());
    }

    #[test]
    #[cfg(feature = "uuid")]
    fn create_uuid() {
        nova::uuid!(pub QuietScreaming);

        QuietScreaming(uuid::Uuid::new_v4());
    }
}
