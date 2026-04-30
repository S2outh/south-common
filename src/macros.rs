#[macro_export]
macro_rules! gen_obdh_types {
    ($chell_union: ty, $name: ident, $on_tm: ty) => {
        $crate::paste::paste! {
            type [<$name ComChannels>] = $crate::obdh::InternalComChannels<{$chell_union::SIZE}>;
            type [<$name CanReceiver>] = $crate::obdh::SouthCanReceiver<'static, {$chell_union::SIZE}, $on_tm>;
            type [<$name CanSender>] = $crate::obdh::SouthCanSender<'static, {$chell_union::SIZE}>;
        }
    };

    ($chell_union: ty, $name: ident) => {
        $crate::gen_obdh_types!($chell_union, $name, $crate::obdh::EmptyFunc)
    };
}
