impl lv_area_t {
    pub fn set_width(&mut self, w: lv_coord_t) {
        self.x2 = self.x1 + w - 1;
    }

    pub fn get_width(&self) -> lv_coord_t {
        self.x2 - self.x1 + 1
    }

    pub fn set(&mut self, x1: lv_coord_t, y1: lv_coord_t, x2: lv_coord_t, y2: lv_coord_t) {
        self.x1 = x1;
        self.y1 = y1;
        self.x2 = x2;
        self.y2 = y2;
    }

    pub fn set_height(&mut self, h: lv_coord_t) {
        self.y2 = self.y1 + h - 1;
    }

    pub fn get_height(&self) -> lv_coord_t {
        self.y2 - self.y1 + 1
    }

    pub fn set_pos(&mut self, x: lv_coord_t, y: lv_coord_t) {
        self.x2 = self.x2 + x - self.x1;
        self.y2 = self.y2 + y - self.y1;
        self.x1 = x;
        self.y1 = y;
    }

    pub fn get_size(&self) -> u32 {
        (self.x2 - self.x1 + 1) as u32 * (self.y2 - self.y1 + 1) as u32
    }

    pub fn increase(&mut self, w_extra: lv_coord_t, h_extra: lv_coord_t) {
        self.x1 -= w_extra;
        self.y1 -= h_extra;
        self.x2 += w_extra;
        self.y2 += h_extra;
    }

    pub fn r#move(&mut self, x_ofs: lv_coord_t, y_ofs: lv_coord_t) {
        self.x1 += x_ofs;
        self.y1 += y_ofs;
        self.x2 += x_ofs;
        self.y2 += y_ofs;
    }

    fn intersect(&mut self, a: &lv_area_t, b: &lv_area_t) -> bool {
        self.x1 = if a.x1 > b.x1 { a.x1 } else { b.x1 };
        self.y1 = if a.y1 > b.y1 { a.y1 } else { b.y1 };
        self.x2 = if a.x2 < b.x2 { a.x2 } else { b.x2 };
        self.y2 = if a.y2 < b.y2 { a.y2 } else { b.y2 };
        self.x1 <= self.x2 && self.y1 <= self.y2
    }

    fn join(&mut self, a: &lv_area_t, b: &lv_area_t) {
        self.x1 = if a.x1 < b.x1 { a.x1 } else { b.x1 };
        self.y1 = if a.y1 < b.y1 { a.y1 } else { b.y1 };
        self.x2 = if a.x2 > b.x2 { a.x2 } else { b.x2 };
        self.y2 = if a.y2 > b.y2 { a.y2 } else { b.y2 };
    }

    fn is_point_on(&self, p_p: &lv_point_t, radius: lv_coord_t) -> bool {
        // First check the basic area.
        let is_on_rect = p_p.x >= self.x1 && p_p.x <= self.x2 && p_p.y >= self.y1 && p_p.y <= self.y2;
        if !is_on_rect {
            return false;
        }

        // Now handle potential rounded rectangles.
        if radius <= 0 {
            // No radius, it is within the rectangle.
            return true;
        }
        let w = self.get_width() / 2;
        let h = self.get_height() / 2;
        let max_radius = w.min(h);
        let radius = if radius > max_radius { max_radius } else { radius };

        // Check if it's in one of the corners.
        let mut corner_area = lv_area_t {
            x1: 0,
            y1: 0,
            x2: 0,
            y2: 0,
        };
        // Top left
        corner_area.x1 = self.x1;
        corner_area.x2 = self.x1 + radius;
        corner_area.y1 = self.y1;
        corner_area.y2 = self.y1 + radius;
        if corner_area.is_point_on(p_p, 0) {
            corner_area.x2 += radius;
            corner_area.y2 += radius;
            return lv_point_within_circle(&corner_area, p_p);
        }
        // Bottom left
        corner_area.y1 = self.y2 - radius;
        corner_area.y2 = self.y2;
        if corner_area.is_point_on(p_p, 0) {
            corner_area.x2 += radius;
            corner_area.y1 -= radius;
            return lv_point_within_circle(&corner_area, p_p);
        }
        // Bottom right
        corner_area.x1 = self.x2 - radius;
        corner_area.x2 = self.x2;
        if corner_area.is_point_on(p_p, 0) {
            corner_area.x1 -= radius;
            corner_area.y1 -= radius;
            return lv_point_within_circle(&corner_area, p_p);
        }
        // Top right
        corner_area.y1 = self.y1;
        corner_area.y2 = self.y1 + radius;
        if corner_area.is_point_on(p_p, 0) {
            corner_area.x1 -= radius;
            corner_area.y2 += radius;
            return lv_point_within_circle(&corner_area, p_p);
        }

        // Not within corners.
        true
    }

    fn is_on(&self, other: &lv_area_t) -> bool {
        self.x1 >= other.x1 && self.x2 <= other.x2 && self.y1 >= other.y1 && self.y2 <= other.y2
    }

    fn is_in(&self, other: &lv_area_t, radius: lv_coord_t) -> bool {
        self.x1 >= other.x1 + radius && self.x2 <= other.x2 - radius && self.y1 >= other.y1 + radius && self.y2 <= other.y2 - radius
    }

    fn is_out(&self, other: &lv_area_t) -> bool {
        self.x2 < other.x1 || self.x1 > other.x2 || self.y2 < other.y1 || self.y1 > other.y2
    }

    fn is_equal(&self, other: &lv_area_t) -> bool {
        self.x1 == other.x1 && self.x2 == other.x2 && self.y1 == other.y1 && self.y2 == other.y2
    }

    fn align(&mut self, to_align: &mut lv_area_t, align: lv_align_t, ofs_x: lv_coord_t, ofs_y: lv_coord_t) {
        let mut x = 0;
        let mut y = 0;
        if align == LV_ALIGN_CENTER as u8 {
            x = self.get_width() / 2 - to_align.get_width() / 2;
            y = self.get_height() / 2 - to_align.get_height() / 2;
        }
        else if align == LV_ALIGN_TOP_LEFT as u8 {
            x = 0;
            y = 0;
        }
        else if align == LV_ALIGN_TOP_MID as u8 {
            x = self.get_width() / 2 - to_align.get_width() / 2;
            y = 0;
        }
        else if align == LV_ALIGN_TOP_RIGHT as u8 {
            x = self.get_width() - to_align.get_width();
            y = 0;
        }
        else if align == LV_ALIGN_BOTTOM_LEFT as u8 {
            x = 0;
            y = self.get_height() - to_align.get_height();
        }
        else if align == LV_ALIGN_BOTTOM_MID as u8 {
            x = self.get_width() / 2 - to_align.get_width() / 2;
            y = self.get_height() - to_align.get_height();
        }
        else if align == LV_ALIGN_BOTTOM_RIGHT as u8 {
            x = self.get_width() - to_align.get_width();
            y = self.get_height() - to_align.get_height();
        }
        else if align == LV_ALIGN_LEFT_MID as u8 {
            x = 0;
            y = self.get_height() / 2 - to_align.get_height() / 2;
        }
        else if align == LV_ALIGN_RIGHT_MID as u8 {
            x = self.get_width() - to_align.get_width();
            y = self.get_height() / 2 - to_align.get_height() / 2;
        }
        else if align == LV_ALIGN_BOTTOM_RIGHT as u8 {
            x = self.x2 - to_align.get_width();
            y = self.y2 - to_align.get_height();
        }
        else if align == LV_ALIGN_OUT_TOP_LEFT as u8 {
            x = self.x1 - to_align.get_width();
            y = self.y1 - to_align.get_height();
        }
        else if align == LV_ALIGN_OUT_TOP_MID as u8 {
            x = self.x1 + self.get_width() / 2 - to_align.get_width() / 2;
            y = self.y1 - to_align.get_height();
        }
        else if align == LV_ALIGN_OUT_TOP_RIGHT as u8 {
            x = self.x2;
            y = self.y1 - to_align.get_height();
        }
        else if align == LV_ALIGN_OUT_BOTTOM_LEFT as u8 {
            x = self.x1 - to_align.get_width();
            y = self.y2;
        }
        else if align == LV_ALIGN_OUT_BOTTOM_MID as u8 {
            x = self.x1 + self.get_width() / 2 - to_align.get_width() / 2;
            y = self.y2;
        }
        else if align == LV_ALIGN_OUT_BOTTOM_RIGHT as u8 {
            x = self.x2;
            y = self.y2;
        }
        else if align == LV_ALIGN_OUT_LEFT_TOP as u8 {
            x = self.x1 - to_align.get_width();
            y = self.y1 - to_align.get_height();
        }
        else if align == LV_ALIGN_OUT_LEFT_MID as u8 {
            x = self.x1 - to_align.get_width();
            y = self.y1 + self.get_height() / 2 - to_align.get_height() / 2;
        }
        else if align == LV_ALIGN_OUT_LEFT_BOTTOM as u8 {
            x = self.x1 - to_align.get_width();
            y = self.y2;
        }
        else if align == LV_ALIGN_OUT_RIGHT_TOP as u8 {
            x = self.x2;
            y = self.y1 - to_align.get_height();
        }
        else if align == LV_ALIGN_OUT_RIGHT_MID as u8 {
            x = self.x2;
            y = self.y1 + self.get_height() / 2 - to_align.get_height() / 2;
        }
        else if align == LV_ALIGN_OUT_RIGHT_BOTTOM as u8 {
            x = self.x2;
            y = self.y2;
        }
        else {
            x = 0;
            y = 0;
        }
        x += self.x1;
        y += self.y1;
        let w = to_align.get_width();
        let h = to_align.get_height();
        to_align.set(x, y, x + w - 1, y + h - 1);
    }
}

#[no_mangle]
extern "C" fn lv_area_set_width(area: *mut lv_area_t, w: lv_coord_t) {
    unsafe { (*area).set_width(w); }
}

#[no_mangle]
extern "C" fn lv_area_set(area: *mut lv_area_t, x1: lv_coord_t, y1: lv_coord_t, x2: lv_coord_t, y2: lv_coord_t) {
    unsafe { (*area).set(x1, y1, x2, y2); }
}

#[no_mangle]
#[doc = " Set the height of an area\n @param area_p pointer to an area\n @param h the new height of the area (h == 1 makes y1 == y2)"]
extern "C" fn lv_area_set_height(area_p: *mut lv_area_t, h: lv_coord_t) {
    unsafe { (*area_p).set_height(h); }
}

#[no_mangle]
#[doc = " Set the position of an area (width and height will be kept)\n @param area_p pointer to an area\n @param x the new x coordinate of the area\n @param y the new y coordinate of the area"]
extern "C" fn _lv_area_set_pos(area_p: *mut lv_area_t, x: lv_coord_t, y: lv_coord_t) {
    unsafe { (*area_p).set_pos(x, y); }
}

#[no_mangle]
#[doc = " Return with area of an area (x * y)\n @param area_p pointer to an area\n @return size of area"]
extern "C" fn lv_area_get_size(area_p: *const lv_area_t) -> u32 {
    unsafe { (*area_p).get_size() }
}

#[no_mangle]
extern "C" fn lv_area_increase(area: *mut lv_area_t, w_extra: lv_coord_t, h_extra: lv_coord_t) {
    unsafe { (*area).increase(w_extra, h_extra); }
}

#[no_mangle]
extern "C" fn _lv_area_increase(area: *mut lv_area_t, w_extra: lv_coord_t, h_extra: lv_coord_t) {
    unsafe { (*area).increase(w_extra, h_extra); }
}

#[no_mangle]
extern "C" fn lv_area_move(area: *mut lv_area_t, x_ofs: lv_coord_t, y_ofs: lv_coord_t) {
    unsafe { (*area).r#move(x_ofs, y_ofs); }
}

#[no_mangle]
extern "C" fn _lv_area_intersect(res: *mut lv_area_t, a1: *const lv_area_t, a2: *const lv_area_t) -> bool {
    unsafe { (*res).intersect(&*a1, &*a2) }
}

#[no_mangle]
extern "C" fn _lv_area_join(res: *mut lv_area_t, a1: *const lv_area_t, a2: *const lv_area_t) {
    unsafe { (*res).join(&*a1, &*a2); }
}

#[no_mangle]
extern "C" fn _lv_area_is_in(area: *const lv_area_t, aholder: *const lv_area_t, radius: lv_coord_t) -> bool {
    unsafe { (*area).is_in(&*aholder, radius) }
}

#[no_mangle]
extern "C" fn _lv_area_is_on(area: *const lv_area_t, a2: *const lv_area_t) -> bool {
    unsafe { (*area).is_on(&*a2) }
}

#[no_mangle]
extern "C" fn _lv_area_is_equal(area1: *const lv_area_t, area2: *const lv_area_t) -> bool {
    unsafe { (*area1).is_equal(&*area2) }
}

#[no_mangle]
extern "C" fn _lv_area_is_out(area: *const lv_area_t, aholder: *const lv_area_t) -> bool {
    unsafe { (*area).is_out(&*aholder) }
}

#[no_mangle]
extern "C" fn lv_area_align(base: *mut lv_area_t, to_align: *mut lv_area_t, align: lv_align_t, ofs_x: lv_coord_t, ofs_y: lv_coord_t) {
    unsafe { (*to_align).align(&mut *to_align, align, ofs_x, ofs_y); }
}

#[no_mangle]
extern "C" fn _lv_area_is_point_on(area: *const lv_area_t, point: *const lv_point_t, radius: lv_coord_t) -> bool {
    unsafe { (*area).is_point_on(&*point, radius) }
}

fn lv_point_within_circle(area: &lv_area_t, p: &lv_point_t) -> bool {
    let r = (area.x2 - area.x1) / 2;

    // Circle center.
    let cx = area.x1 + r;
    let cy = area.y1 + r;

    // Simplify the code by moving everything to (0, 0).
    let px = p.x - cx;
    let py = p.y - cy;

    let r_sqrd = (r as u32).pow(2);
    let dist = (px as u32).pow(2) + (py as u32).pow(2);

    if dist <= r_sqrd {
        true
    } else {
        false
    }
}

const _LV_TRANSFORM_TRIGO_SHIFT: u32 = 10;

#[no_mangle]
extern "C" fn lv_point_transform(p: &mut lv_point_t, angle: i32, zoom: i32, pivot: &lv_point_t) {
    if angle == 0 && zoom == 256 {
        return;
    }

    p.x -= pivot.x;
    p.y -= pivot.y;

    if angle == 0 {
        p.x = ((((p.x as i32) * zoom as i32) >> 8) + pivot.x as i32) as lv_coord_t;
        p.y = ((((p.y as i32) * zoom as i32) >> 8) + pivot.y as i32) as lv_coord_t;
        return;
    }

    static mut ANGLE_PREV: i32 = std::i32::MIN;
    static mut SINMA: i32 = 0;
    static mut COSMA: i32 = 0;
    unsafe {
        if ANGLE_PREV != angle {
            let angle_limited = if angle > 3600 { angle - 3600 } else if angle < 0 { angle + 3600 } else { angle };

            let angle_low = angle_limited / 10;
            let angle_high = angle_low + 1;
            let angle_rem = angle_limited - angle_low * 10;

            let s1 = lv_trigo_sin(angle_low as i16) as i32;
            let s2 = lv_trigo_sin(angle_high as i16) as i32;

            let c1 = lv_trigo_sin((angle_low + 90) as i16) as i32;
            let c2 = lv_trigo_sin((angle_high + 90) as i16) as i32;

            SINMA = ((s1 * (10 - angle_rem) + s2 * angle_rem) / 10) as i32;
            COSMA = ((c1 * (10 - angle_rem) + c2 * angle_rem) / 10) as i32;
            SINMA >>= LV_TRIGO_SHIFT - _LV_TRANSFORM_TRIGO_SHIFT;
            COSMA >>= LV_TRIGO_SHIFT - _LV_TRANSFORM_TRIGO_SHIFT;
            ANGLE_PREV = angle;
        }
        let x = p.x as i32;
        let y = p.y as i32;
        if zoom == 256 {
            p.x = (((COSMA * x - SINMA * y) >> _LV_TRANSFORM_TRIGO_SHIFT) + pivot.x as i32) as lv_coord_t;
            p.y = (((SINMA * x + COSMA * y) >> _LV_TRANSFORM_TRIGO_SHIFT) + pivot.y as i32) as lv_coord_t;
        } else {
            p.x = ((((COSMA * x - SINMA * y) * zoom) >> (_LV_TRANSFORM_TRIGO_SHIFT + 8)) + pivot.x as i32) as lv_coord_t;
            p.y = ((((SINMA * x + COSMA * y) * zoom) >> (_LV_TRANSFORM_TRIGO_SHIFT + 8)) + pivot.y as i32) as lv_coord_t;
        }
    }
}
