use std::os::raw::{c_void, c_uint};
use std::ptr;
use std::ptr::null;

const LL_NODE_META_SIZE: usize = std::mem::size_of::<*mut lv_ll_node_t>() * 2;

fn ll_prev_p_offset(ll_p: &lv_ll_t) -> usize {
    ll_p.n_size as usize
}

fn ll_next_p_offset(ll_p: &lv_ll_t) -> usize {
    ll_p.n_size as usize + std::mem::size_of::<*mut lv_ll_node_t>()
}

impl lv_ll_t {
    pub fn init(&mut self, node_size: u32) {
        self.head = std::ptr::null_mut();
        self.tail = std::ptr::null_mut();

        #[cfg(feature = "LV_ARCH_64")]
        {
            /* Round the size up to 8 */
            let node_size = (node_size + 7) & !0x7;
            self.n_size = node_size;
        }
        #[cfg(not(feature = "LV_ARCH_64"))]
        {
            /* Round the size up to 4 */
            let node_size = (node_size + 3) & !0x3;
            self.n_size = node_size;
        }
    }

    pub fn insert_head(&mut self) -> *mut c_void {
        unsafe {
            let mut n_new: *mut lv_ll_node_t = ptr::null_mut();
            n_new = lv_mem_alloc((self.n_size as usize + LL_NODE_META_SIZE) as usize) as *mut lv_ll_node_t;

            if !n_new.is_null() {
                self.set_prev(n_new, ptr::null_mut()); /* No prev. before the new head */
                self.set_next(n_new, self.head); /* After new comes the old head */

                if !self.head.is_null() { /* If there is an old head, set the new node as the previous of the old head */
                    self.set_prev(self.head, n_new);
                }

                self.head = n_new; /* Set the new head in the list */
                if self.tail.is_null() { /* If there is no tail (i.e., the list was empty), set the tail to the new head as well */
                    self.tail = n_new;
                }
            }

            return n_new as *mut c_void;
        }
    }

    unsafe fn set_prev(&self, act: *mut lv_ll_node_t, prev: *mut lv_ll_node_t) {
        if act.is_null() {
            return; // Can't set the prev node of `NULL`
        }

        let mut act8: *mut u8 = act as *mut u8;

        act8 = act8.add(ll_prev_p_offset(self));

        let act_node_p: *mut *mut lv_ll_node_t = act8 as *mut *mut lv_ll_node_t;
        let prev_node_p: *const *mut lv_ll_node_t = &prev as *const *mut lv_ll_node_t;

        *act_node_p = *prev_node_p;
    }

    unsafe fn set_next(&self, act: *mut lv_ll_node_t, next: *mut lv_ll_node_t) {
        if act.is_null() {
            return;
        }
        let mut act8: *mut u8 = act as *mut u8;

        act8 = act8.add(ll_next_p_offset(self));

        let act_node_p: *mut *mut lv_ll_node_t = act8 as *mut *mut lv_ll_node_t;
        let next_node_p: *const *mut lv_ll_node_t = &next as *const *mut lv_ll_node_t;

        *act_node_p = *next_node_p;
    }

    pub fn get_head(&self) -> *mut c_void {
        self.head as *mut c_void
    }

    pub fn get_tail(&self) -> *mut c_void {
        self.tail as *mut c_void
    }

    pub unsafe fn get_next(&self, n_act: *mut c_void) -> *mut c_void {
        let mut n_act_d = (n_act as *const lv_ll_node_t).wrapping_add(ll_next_p_offset(self));
        *(n_act_d as *const *mut std::ffi::c_void)

    }

    pub unsafe fn get_prev(&self, n_act: *mut c_void) -> *mut c_void {
        let mut n_act_d = (n_act as *const lv_ll_node_t).wrapping_add(ll_prev_p_offset(self));
        *(n_act_d as *const *mut std::ffi::c_void)
    }

    pub unsafe fn get_len(&self) -> u32 {
        let mut len = 0;
        let mut node = self.get_head();

        while !node.is_null() {
            len += 1;
            node = self.get_next(node);
        }

        len
    }

    pub unsafe fn insert_prev(&mut self, n_act: *mut c_void) -> *mut c_void {
        let mut n_new: *mut c_void = ptr::null_mut();

        if n_act.is_null() {
            return ptr::null_mut();
        }

        if n_act == self.get_head() {
            n_new = self.insert_head();
            if n_new == ptr::null_mut() {
                return ptr::null_mut();
            }
        } else {
            n_new = lv_mem_alloc((self.n_size as usize + LL_NODE_META_SIZE) as usize) as *mut c_void;
            if n_new == ptr::null_mut() {
                return ptr::null_mut();
            }

            let n_prev = self.get_prev(n_act);
            self.set_next(n_prev as *mut lv_ll_node_t, n_new as *mut lv_ll_node_t);
            self.set_prev(n_new as *mut lv_ll_node_t, n_prev as *mut lv_ll_node_t);
            self.set_next(n_new as *mut lv_ll_node_t, n_act as *mut lv_ll_node_t);
            self.set_prev(n_act as *mut lv_ll_node_t, n_new as *mut lv_ll_node_t);
        }

        n_new
    }

    pub unsafe fn insert_tail(&mut self) -> *mut c_void {
        let mut n_new: *mut c_void = lv_mem_alloc((self.n_size as usize + LL_NODE_META_SIZE) as usize) as *mut c_void;

        if n_new != ptr::null_mut() {
            self.set_next(n_new as *mut lv_ll_node_t, ptr::null_mut());
            self.set_prev(n_new as *mut lv_ll_node_t, self.tail);
            if self.tail.is_null() {
                self.set_next(self.tail as *mut lv_ll_node_t, n_new as *mut lv_ll_node_t);
            }

            self.tail = n_new as *mut lv_ll_node_t;
            if self.head.is_null() {
                self.head = n_new as *mut lv_ll_node_t;
            }
        }

        n_new
    }

    pub unsafe fn remove(&mut self, node_p: *mut c_void) {
        if node_p.is_null() {
            return;
        }

        if node_p == self.get_head() {
            self.head = self.get_next(node_p) as *mut lv_ll_node_t;
            if self.head.is_null() {
                self.tail = ptr::null_mut();
            } else {
                self.set_prev(self.head as *mut lv_ll_node_t, ptr::null_mut());
            }
        } else if node_p == self.get_tail() {
            self.tail = self.get_prev(node_p) as *mut lv_ll_node_t;
            if self.tail.is_null() {
                self.head = ptr::null_mut();
            } else {
                self.set_next(self.tail as *mut lv_ll_node_t, ptr::null_mut());
            }
        } else {
            let n_prev = self.get_prev(node_p);
            let n_next = self.get_next(node_p);
            self.set_next(n_prev as *mut lv_ll_node_t, n_next as *mut lv_ll_node_t);
            self.set_prev(n_next as *mut lv_ll_node_t, n_prev as *mut lv_ll_node_t);
        }
    }

    pub unsafe fn clear(&mut self) {
        let mut node = self.get_head();
        while !node.is_null() {
            let next = self.get_next(node);
            self.remove(node);
            lv_mem_free(node);
            node = next;
        }
        self.head = ptr::null_mut();
        self.tail = ptr::null_mut();
    }

    pub unsafe fn is_empty(&self) -> bool {
        self.head.is_null() && self.tail.is_null()
    }

    pub unsafe fn move_before(&mut self, n_act: *mut c_void, n_after: *mut c_void) {
        if n_act == n_after {
            return;
        }

        let mut n_before: *mut c_void = ptr::null_mut();
        if n_after.is_null() {
            n_before = self.get_tail();
        } else {
            n_before = self.get_prev(n_after);
        }

        if n_act == n_before {
            return;
        }

        self.remove(n_act);

        self.set_next(n_before as *mut lv_ll_node_t, n_act as *mut lv_ll_node_t);
        self.set_prev(n_act as *mut lv_ll_node_t, n_before as *mut lv_ll_node_t);
        self.set_prev(n_after as *mut lv_ll_node_t, n_act as *mut lv_ll_node_t);
        self.set_next(n_act as *mut lv_ll_node_t, n_after as *mut lv_ll_node_t);

        if n_before.is_null() {
            self.head = n_act as *mut lv_ll_node_t;
        }

        if n_before.is_null() {
            self.head = n_act as *mut lv_ll_node_t;
        }
    }

    pub unsafe fn change_list(&mut self, new_ll_p: *mut lv_ll_t, node: *mut c_void, head: bool) {
        self.remove(node);
        if head {
            (*new_ll_p).set_prev(node as *mut lv_ll_node_t, ptr::null_mut());
            (*new_ll_p).set_next(node as *mut lv_ll_node_t, (*new_ll_p).head);

            if !(*new_ll_p).get_head().is_null() {
                (*new_ll_p).set_prev((*new_ll_p).head as *mut lv_ll_node_t, node as *mut lv_ll_node_t);
            }
            (*new_ll_p).head = node as *mut lv_ll_node_t;
            if (*new_ll_p).tail.is_null() {
                (*new_ll_p).tail = node as *mut lv_ll_node_t;
            }
        } else {
            (*new_ll_p).set_prev(node as *mut lv_ll_node_t, (*new_ll_p).tail);
            (*new_ll_p).set_next(node as *mut lv_ll_node_t, ptr::null_mut());

            if !(*new_ll_p).get_tail().is_null() {
                (*new_ll_p).set_next((*new_ll_p).tail, node as *mut lv_ll_node_t);
            }
            (*new_ll_p).tail = node as *mut lv_ll_node_t;
            if (*new_ll_p).head.is_null() {
                (*new_ll_p).head = node as *mut lv_ll_node_t;
            }
        }
    }
}

#[no_mangle]
pub unsafe extern "C" fn _lv_ll_init(ll_p: *mut lv_ll_t, node_size: c_uint) {
    (*ll_p).init(node_size as u32);
}


#[no_mangle]
pub unsafe extern "C" fn _lv_ll_ins_head(ll_p: *mut lv_ll_t) -> *mut c_void {
    (*ll_p).insert_head()
}

#[no_mangle]
pub unsafe extern "C" fn _lv_ll_get_head(ll_p: *const lv_ll_t) -> *mut c_void {
    if ll_p.is_null() {
        return ptr::null_mut();
    }
    (*ll_p).get_head()
}

#[no_mangle]
pub unsafe extern "C" fn _lv_ll_get_tail(ll_p: *const lv_ll_t) -> *mut c_void {
    if ll_p.is_null() {
        return ptr::null_mut();
    }
    (*ll_p).get_tail()
}

#[no_mangle]
pub unsafe extern "C" fn _lv_ll_get_next(ll_p: *const lv_ll_t, act: *mut c_void) -> *mut c_void {
    (*ll_p).get_next(act)
}

#[no_mangle]
pub unsafe extern "C" fn _lv_ll_get_prev(ll_p: *const lv_ll_t, act: *mut c_void) -> *mut c_void {
    (*ll_p).get_prev(act)
}

#[no_mangle]
pub unsafe extern "C" fn _lv_ll_get_len(ll_p: *const lv_ll_t) -> u32 {
    (*ll_p).get_len() as c_uint
}

#[no_mangle]
pub unsafe extern "C" fn _lv_ll_ins_prev(ll_p: *mut lv_ll_t, act: *mut c_void) -> *mut c_void {
    (*ll_p).insert_prev(act)
}

#[no_mangle]
pub unsafe extern "C" fn _lv_ll_ins_tail(ll_p: *mut lv_ll_t) -> *mut c_void {
    (*ll_p).insert_tail()
}

#[no_mangle]
pub unsafe extern "C" fn _lv_ll_remove(ll_p: *mut lv_ll_t, node_p: *mut c_void) {
    (*ll_p).remove(node_p)
}

#[no_mangle]
pub unsafe extern "C" fn _lv_ll_clear(ll_p: *mut lv_ll_t) {
    (*ll_p).clear()
}

#[no_mangle]
pub unsafe extern "C" fn _lv_ll_is_empty(ll_p: *const lv_ll_t) -> bool {
    (*ll_p).is_empty()
}

#[no_mangle]
pub unsafe extern "C" fn _lv_ll_move_before(ll_p: *mut lv_ll_t, n_act: *mut c_void, n_after: *mut c_void) {
    (*ll_p).move_before(n_act, n_after)
}

#[no_mangle]
pub unsafe extern "C" fn _lv_ll_chg_list(ll_p: *mut lv_ll_t, new_ll_p: *mut lv_ll_t, node: *mut c_void, head: bool) {
    (*ll_p).change_list(new_ll_p, node, head)
}
