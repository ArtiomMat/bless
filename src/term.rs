//! A cross platform interface with the terminal functionality

use crate::error::Error;
use lazy_static::lazy_static;
use libc;
use std::{
    ffi::CStr,
    sync::{
        atomic::{AtomicBool, Ordering},
        Arc, Mutex,
    },
};

/// Returns [width,height] slash [cols,rows].
pub fn get_size() -> Result<[usize; 2], Error> {
    unsafe {
        let mut ws: libc::winsize = std::mem::zeroed();

        let ret = libc::ioctl(
            libc::STDOUT_FILENO,
            libc::TIOCGWINSZ,
            &mut ws as *mut libc::winsize,
        );

        if ret < 0 {
            Err(Error(String::from("ioctl() failed")))
        } else {
            Ok([ws.ws_col as usize, ws.ws_row as usize])
        }
    }
}

pub fn read_u8() -> Result<Option<u8>, Error> {
    let mut c = [0u8];
    let n = read(&mut c)?;
    if n == 1 {
        Ok(Some(c[0]))
    } else {
        Ok(None)
    }
}

/// Reads a UTF-8 character, into rust's char type.
pub fn read_char() -> Result<Option<char>, Error> {
    let mut raw = [0u8; 4];
    raw[0] = match read_u8()? {
        Some(x) => x,
        None => return Ok(None),
    };

    if raw[0] < 128 {
        Ok(Some(raw[0] as char))
    } else {
        Err(Error(String::from("Don't support UTF8 yet.")))
    }
}

/// The result either gives the number of bytes actually read, which may be 0, or an error.
pub fn read(b: &mut [u8]) -> Result<usize, Error> {
    unsafe {
        let n = libc::read(libc::STDIN_FILENO, b.as_ptr() as *mut libc::c_void, b.len());
        if n < 0 {
            Err(Error(String::from("write() failed.")))
        } else {
            Ok(n as usize)
        }
    }
}

pub fn write(s: &str) -> Result<usize, Error> {
    unsafe {
        let n = libc::write(
            libc::STDOUT_FILENO,
            s.as_ptr() as *const libc::c_void,
            s.len(),
        );

        if n < 0 {
            Err(Error(String::from("write() failed.")))
        } else {
            Ok(n as usize)
        }
    }
}

lazy_static! {
    static ref OLD_TC: Mutex<libc::termios> = Mutex::new(unsafe { std::mem::zeroed() });
    static ref NEW_TC: Mutex<libc::termios> = Mutex::new(unsafe { std::mem::zeroed() });
    /// If set_raw_input(true) was called at least once, this is to prevent giving invalid termios structs if yes=false.
    static ref DID_RAW_INPUT: AtomicBool = AtomicBool::new(false);
}

extern "C" fn at_exit() {
    set_raw_input(false);
}

/// Set input to raw non canonical mode, to not have to wait for new line and buffering.
/// With that, it is still blocking, so if you want non-blocking-type behaviour you should use
/// multi threading.
pub fn set_raw_input(yes: bool) {
    // TODO: Make it actually return the error instead of all this stuff
    unsafe {
        let old_tc_ptr = &mut (*OLD_TC.lock().unwrap()) as *mut libc::termios;

        if yes {
            let new_tc_ptr = &mut (*NEW_TC.lock().unwrap()) as *mut libc::termios;

            libc::tcgetattr(libc::STDIN_FILENO, old_tc_ptr);

            *new_tc_ptr.as_mut().unwrap() = *old_tc_ptr.as_mut().unwrap();
            new_tc_ptr.as_mut().unwrap().c_lflag &= !(libc::ICANON | libc::ECHO);
            libc::tcsetattr(libc::STDIN_FILENO, libc::TCSANOW, new_tc_ptr);

            // Also set up the at_exit() function, and update DID_RAW_INPUT
            DID_RAW_INPUT
                .fetch_update(Ordering::Relaxed, Ordering::Relaxed, |v| {
                    if !v {
                        libc::atexit(at_exit);
                    }
                    Some(true)
                })
                .expect("Failed to update DID_RAW_INPUT");
        } else if DID_RAW_INPUT.load(Ordering::Relaxed) {
            libc::tcsetattr(libc::STDIN_FILENO, libc::TCSANOW, old_tc_ptr);
        }
    }
}

// #[deprecated(note="No need to use it, write flushes automatically if necessary. H")]
fn sync() -> Result<(), Error> {
    // unsafe {
    //     let ret = libc::fsync(libc::STDOUT_FILENO);

    //     if ret != 0 {
    //         let err_ptr = libc::__errno_location();
    //         if err_ptr.is_null() {
    //             return Err(Error(String::from("Unknown")));
    //         }

    //         let str_ptr = libc::strerror(*err_ptr);

    //         if str_ptr.is_null() {
    //             return Err(Error(String::from("Unknown")));
    //         }

    //         if let Ok(s) = CStr::from_ptr(str_ptr).to_str() {
    //             Err(Error(format!("{}", s)))
    //         } else {
    //             Err(Error(String::from("Unknown")))
    //         }
    //     } else {
    //         Ok(())
    //     }
    // }
    Ok(())
}
