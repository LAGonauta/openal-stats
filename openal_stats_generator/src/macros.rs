#[macro_export]
macro_rules! DECL_THUNK {
    {
        $($sym:ident($($param:ident: $param_ty:ty),*) -> $ret_ty:ty,)*
    } => {
        $(
            #[unsafe(no_mangle)]
            pub unsafe extern "C" fn $sym($($param: $param_ty),*) -> $ret_ty {
                _ = crate::stats_processor::STATS_SEND.send(crate::stats_processor::Stats::$sym);

                unsafe {
                    crate::al_api::api.$sym($($param),*)
                }
            }
        )*
    }
}
