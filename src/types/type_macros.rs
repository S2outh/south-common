macro_rules! key_value_bitflags {
    (
        pub struct $name: ident: $type:ty {
            $(const $flagname: ident = $flagvalue: expr;)*
        }
    ) => { paste::paste! {
        bitflags! {
            pub struct $name: $type {
                $(const $flagname = $flagvalue;)*
            }
        }

        #[cfg_attr(feature = "serde", derive(serde::Serialize))]
        pub struct [<$name KeyValue>] {
            $(pub [<$flagname:lower>]: bool,)*
        }

        impl From<$name> for [<$name KeyValue>] {
            fn from(value: $name) -> Self {
                Self {
                    $([<$flagname:lower>]: value.contains($name::$flagname),)*
                }
            }
        }
    }};
}
