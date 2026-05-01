#[macro_export]
macro_rules! gen_obdh_types {
    ($chell_union: ty, $name: ident, $on_tm: ty, $subsys_cmd: ty) => {
        $crate::paste::paste! {
            type [<$name ComChannels>] = $crate::obdh::InternalComChannels<$subsys_cmd, {$chell_union::SIZE}>;
            type [<$name CanReceiver>] = $crate::obdh::SouthCanReceiver<'static, $subsys_cmd, {$chell_union::SIZE}, $on_tm>;
            type [<$name CanSender>] = $crate::obdh::SouthCanSender<'static, $subsys_cmd, {$chell_union::SIZE}>;
            type [<$name TMSender>] = $crate::obdh::TMSender<'static, {$chell_union::SIZE}>;
            type [<$name TCReceiver>] = $crate::obdh::TCReceiver<'static, $subsys_cmd>;
        }
    };

    ($chell_union: ty, $name: ident, cmd => $subsys_cmd: ty) => {
        $crate::gen_obdh_types!($chell_union, $name, $crate::obdh::EmptyFunc, $subsys_cmd)
    };

    ($chell_union: ty, $name: ident, on_tm => $on_tm: ty) => {
        $crate::gen_obdh_types!($chell_union, $name, $on_tm, $crate::types::NoCommand)
    };

    ($chell_union: ty, $name: ident) => {
        $crate::gen_obdh_types!($chell_union, $name, $crate::obdh::EmptyFunc, $crate::types::NoCommand)
    };
}
