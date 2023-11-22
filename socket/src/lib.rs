use std::ptr;
use std::sync::mpsc;
use std::sync::mpsc::{Receiver, Sender};

extern crate libc;
use std::ffi::{CStr, CString};

macro_rules! rustln {
    () => {
        println!("<rust>")
    };
    ($($arg:tt)*) => {
        print!("<rust> ");
        println!($($arg)*)
    };
}

#[repr(C)]
pub struct Socket {
    ip: String,

    tx: Sender<String>,
    rx: Receiver<String>,
}

#[no_mangle]
pub extern "C" fn create(ip: *const libc::c_char) -> *const Socket {
    if ip.is_null() {
        rustln!("creation error: `ip` shoud not be a null value");
        return ptr::null();
    }

    let ip = unsafe { CStr::from_ptr(ip).to_bytes() };

    match String::from_utf8(ip.to_vec()) {
        Ok(ip) => {
            rustln!("created: ip = {ip}");

            let (tx, rx) = mpsc::channel();

            Box::into_raw(Box::new(Socket { ip, tx, rx }))
        }
        Err(e) => {
            rustln!("creation error: {e}");
            ptr::null()
        }
    }
}

#[no_mangle]
pub extern "C" fn destroy(socket: *const Socket) {
    if socket.is_null() {
        rustln!("destroying error: `socket` shoud not be a null value");
    } else {
        unsafe {
            let ip = (*socket).ip.clone();

            drop(Box::from_raw(socket as *mut Socket));

            rustln!("destroyed: {ip}");
        }
    }
}

#[no_mangle]
pub extern "C" fn send(socket: *const Socket, data: *const libc::c_char) {
    let data = unsafe { CStr::from_ptr(data).to_bytes() };

    match String::from_utf8(data.to_vec()) {
        Ok(data) => {
            if socket.is_null() {
                rustln!("sending error: `socket` shoud not be a null value");
            } else {
                let socket = unsafe { &*socket };

                match socket.tx.send(String::from("Hello from Rust!")) {
                    Ok(_) => {
                        rustln!("sent: ip = {}, data = {data}", socket.ip);
                    }
                    Err(e) => {
                        rustln!("sending error: {e}");
                    }
                }
            }
        }
        Err(e) => {
            rustln!("sending error: {e}");
        }
    }
}

#[no_mangle]
pub extern "C" fn listen(socket: *const Socket) -> *const libc::c_char {
    if socket.is_null() {
        rustln!("listening error: `socket` shoud not be a null value");
        return ptr::null();
    }

    let socket = unsafe { &*socket };

    match socket.rx.recv() {
        Ok(data) => {
            rustln!("received: ip = {}, data = {data}", socket.ip);

            match CString::new(data) {
                Ok(cstr) => cstr.into_raw() as *const libc::c_char,
                Err(e) => {
                    rustln!("listening error: {e}");
                    ptr::null()
                }
            }
        }
        Err(e) => {
            rustln!("Listening error: {e}");
            ptr::null()
        }
    }
}
