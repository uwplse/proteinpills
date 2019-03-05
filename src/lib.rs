extern crate syscall;
#[macro_use]
extern crate lazy_static;

use std::os::raw::c_char;
use std::sync::Mutex;
use std::thread;

struct State {
    is_traced: bool
}

lazy_static! {
    static ref STATE: Mutex<State> = Mutex::new(State {is_traced: detect_tracing()});
}

const MAGIC_NUMBER: usize = 42;
const DETECT_TRACE: usize = 5000;

const ANNOTATE_TIMEOUT: usize = 5001;
const ANNOTATE_MESSAGE: usize = 5002;
const ANNOTATE_STATE : usize = 5003;

const INT_FIELD: usize = 5010;
const STR_FIELD: usize = 5011;


fn detect_tracing() -> bool {
    unsafe {
        let ret = syscall::syscall0(DETECT_TRACE);
        ret == MAGIC_NUMBER
    }
}

fn is_tracing() -> bool {
    STATE.lock().unwrap().is_traced
}

#[no_mangle]
pub extern "C" fn annotate_timeout(ty: *mut c_char) {
    if is_tracing() {
        unsafe {
            syscall::syscall1(ANNOTATE_TIMEOUT, ty as usize);
        }
    }
}

#[no_mangle]
pub extern "C" fn annotate_message(ty: *mut c_char) {
    if is_tracing() {
        unsafe {
            syscall::syscall1(ANNOTATE_MESSAGE, ty as usize);
        }
    }
}

#[no_mangle]
pub extern "C" fn annotate_state() {
    if is_tracing() {
        unsafe {
            syscall::syscall0(ANNOTATE_STATE);
        }
    }
}


#[no_mangle]
pub extern "C" fn int_field(path: *mut c_char, value: i32) {
    if is_tracing() {
        unsafe {
            syscall::syscall2(INT_FIELD, path as usize, value as usize);
        }
    }
}

#[no_mangle]
pub extern "C" fn str_field(path: *mut c_char, value: *mut c_char) {
    if is_tracing() {
        unsafe {
            syscall::syscall2(STR_FIELD, path as usize, value as usize);
        }
    }
}

#[no_mangle]
pub extern "C" fn register_state_function(f: Option<extern "C" fn()>) {
    if is_tracing() {
        if let Some(state_fn) = f {
            thread::spawn(move || {
                loop {
                    annotate_state();
                    state_fn();
                }
            });
        }
    }
}
