
use std::ffi::OsStr;
use std::ffi::OsString;
use std::os::windows::ffi::OsStrExt;
use std::os::windows::ffi::OsStringExt;
use std::iter::once;

use wintun::{Wintun, LoggerLevel, DWORD64, GUID, LPCWSTR};

fn encode_str(value: &str) -> Vec<u16> {
    OsStr::new(value)
        .encode_wide()
        .chain(once(0))
        .collect()
}

fn wstrlen(value: LPCWSTR) -> usize {
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

fn decode_str(value: LPCWSTR) -> OsString {
    let value = unsafe { std::slice::from_raw_parts(value, wstrlen(value)) };
    OsString::from_wide(value)
}

unsafe extern "C" fn logger_cb(level: LoggerLevel, time: DWORD64, message: LPCWSTR) {
    let message = decode_str(message).into_string().unwrap();
    println!("wintun: {}", message);
}

fn main() {
    let wintun = wintun::wintun_get();

    unsafe {
        (wintun.set_logger)(logger_cb);
    }

    use uuid::Uuid;
    let uuid = Uuid::new_v4();

    let adapter = wintun::Adapter::new("Test", "Test", uuid);
    let session = wintun::Session::new(&adapter, wintun::MAX_RING_CAPACITY);

    loop {
        let mut packet: Vec<u8> = Vec::new();
        packet.extend_from_slice(&(0xDEADBEEFu32.to_le_bytes()));
        session.send(&packet);
        
        let packet = session.recv();
        if let Some(packet) = packet {
            println!("got packet (len={})", packet.data.len());
        }
    }
}
