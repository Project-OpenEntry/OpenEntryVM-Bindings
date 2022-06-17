macro_rules! gen_bindings {
    ($(fn_bind![$ty: ty, $root: ident::$func: ident];)+) => {
        paste::paste! {
            $(
                pub(crate) static mut [<$root __ $func>]: Option<$ty> = None;
            )+
        }
    };
}

macro_rules! load_bindings {
    (@step $idx: expr, $idx_out: ident, $runtime: ident,) => {
        $idx_out = $idx;
    };

    (@step $idx: expr, $idx_out: ident, $runtime: ident, $ty: ty, $root: ident, $func: ident, $($_ty: ty, $_root: ident, $_func: ident,)*) => {
        paste::paste! {
            unsafe {
                [<$root __ $func>] = Some(std::mem::transmute::<*const (), $ty>($runtime.ffi.0[$idx] as *const ()));
            }
        }

        load_bindings!(@step $idx + 1usize, $idx_out, $runtime, $($_ty, $_root, $_func,)*);
    };

    ($idx_out: ident, $runtime: ident, $(fn_bind![$ty: ty, $root: ident::$func: ident];)+) => {
        load_bindings!(@step 0usize, $idx_out, $runtime, $($ty, $root, $func,)+)
    };
}

macro_rules! async_binding {
    ($idx_in: ident, $runtime: ident, &Arc, $root: ident::$func: ident, $ret: ty, [$($id: ident: $ty: ty),*]) => {
        paste::paste! {
            [<$root __ $func>] = Some(std::mem::transmute::<*const (), for<'a> fn(&'a std::sync::Arc<$root>$(, $ty)*) -> FfiFuture<$ret>>($runtime.ffi.0[$idx_in] as *const ()));

            $idx_in += 1usize;
        }
    };

    ($idx_in: ident, $runtime: ident, $root: ident::$func: ident, $ret: ty, [$($id: ident: $ty: ty),*]) => {
        paste::paste! {
            [<$root __ $func>] = Some(std::mem::transmute::<*const (), for<'a> fn(&'a $root$(, $ty)*) -> FfiFuture<$ret>>($runtime.ffi.0[$idx_in] as *const ()));

            $idx_in += 1usize;
        }
    };

    ($idx_in: ident, $runtime: ident, noself, $root: ident::$func: ident, $ret: ty, [$($id: ident: $ty: ty),*]) => {
        paste::paste! {
            [<$root __ $func>] = Some(std::mem::transmute::<*const (), fn($($ty),*) -> FfiFuture<$ret>>($runtime.ffi.0[$idx_in] as *const ()));

            $idx_in += 1usize;
        }
    };
}

pub(crate) use gen_bindings;
pub(crate) use async_binding;
pub(crate) use load_bindings;