use crate::LPCWSTR;

use std::ffi::OsStr;
use std::ffi::OsString;
use std::iter::once;
use std::os::windows::ffi::OsStrExt;
use std::os::windows::ffi::OsStringExt;

pub fn encode_str(value: &str) -> Vec<u16> {
    OsStr::new(value).encode_wide().chain(once(0)).collect()
}

pub fn wstrlen(value: LPCWSTR) -> usize {
    let mut len: usize = 0;
    let mut pos = value;
    unsafe {
        while *pos != 0 {
            len += 1;
            pos = pos.offset(1);
        }
    }
    len
}

pub fn decode_str(value: LPCWSTR) -> OsString {
    let value = unsafe { std::slice::from_raw_parts(value, wstrlen(value)) };
    OsString::from_wide(value)
}
