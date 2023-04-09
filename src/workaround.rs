include!(concat!(env!("OUT_DIR"), "/bindings.rs"));

#[cfg(feature = "lv_tick_custom")]
macro_rules! LV_TICK_CUSTOM_SYS_TIME_EXPR {
        () => {
            unsafe {
                SDL_GetTicks() as u32
            }
        };
    }
