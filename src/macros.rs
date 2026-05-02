#[macro_export]
macro_rules! gen_obdh_types {
    ($name: ident, $def_path: path, $on_tm: ty, $subsys_cmd: ty) => {
        $crate::paste::paste! {
            type [<$name ChellUnion>] = $crate::chell::fd_compat_chell_union!($def_path);
            type [<$name ComChannels>] = $crate::obdh::InternalComChannels<$subsys_cmd, {[<$name ChellUnion>]::SIZE}>;
            type [<$name CanReceiver>] = $crate::obdh::SouthCanReceiver<'static, $subsys_cmd, {[<$name ChellUnion>]::SIZE}, $on_tm>;
            type [<$name CanSender>] = $crate::obdh::SouthCanSender<'static, $subsys_cmd, {[<$name ChellUnion>]::SIZE}>;
            type [<$name TMSender>] = $crate::obdh::TMSender<'static, {[<$name ChellUnion>]::SIZE}>;
            type [<$name TCReceiver>] = $crate::obdh::TCReceiver<'static, $subsys_cmd>;
        }
    };

    ($name: ident, $def_path: path, cmd => $subsys_cmd: ty) => {
        $crate::gen_obdh_types!($name, $def_path, $crate::obdh::EmptyFunc, $subsys_cmd);
    };

    ($name: ident, $def_path: path, on_tm => $on_tm: ty) => {
        $crate::gen_obdh_types!($name, $def_path, $on_tm, $crate::types::NoCommand);
    };

    ($name: ident, $def_path: path) => {
        $crate::gen_obdh_types!($name, $def_path, $crate::obdh::EmptyFunc, $crate::types::NoCommand);
    };
}
