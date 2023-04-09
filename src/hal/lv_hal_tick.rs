include!("../workaround.rs");

#[cfg(not(feature = "lv_tick_custom"))]
static mut sys_time: u32 = 0;
#[cfg(not(feature = "lv_tick_custom"))]
static mut tick_irq_flag: u8 = 0;

#[cfg(not(feature = "lv_tick_custom"))]
#[no_mangle]
pub unsafe extern "C" fn lv_tick_inc(tick_period: u32) {
    tick_irq_flag = 0;
    sys_time += tick_period;
}

#[no_mangle]
#[cfg(not(feature = "lv_tick_custom"))]
pub fn lv_tick_get() -> u32 {
    println!("lv_tick_get() called");
    unsafe {
        let mut result: u32;
        loop {
            unsafe {
                tick_irq_flag = 1;
                result = sys_time;
            };
            if tick_irq_flag != 0 {
                break;
            }
        }
        result
    }
}

#[no_mangle]
#[cfg(feature = "lv_tick_custom")]
pub fn lv_tick_get() -> u32 {
    LV_TICK_CUSTOM_SYS_TIME_EXPR!()
}

#[no_mangle]
extern "C" fn lv_tick_elaps(prev_tick: u32) -> u32 {
    let act_time = lv_tick_get();

    if act_time >= prev_tick {
        act_time - prev_tick
    } else {
        (core::u32::MAX - prev_tick + 1) + act_time
    }
}
