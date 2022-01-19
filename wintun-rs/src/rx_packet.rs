
use crate::{Session, BYTE, DWORD};

pub struct RxPacket<'a> {
    pub session: &'a Session<'a>,
    pub data: &'a mut [u8],
}

impl<'a> RxPacket<'a> {
    pub fn new(session: &'a Session, packet: *mut u8, len: DWORD) -> Self {
        let slice = unsafe { std::slice::from_raw_parts_mut(packet, len.try_into().unwrap()) };

        RxPacket {
            session,
            data: slice
        }
    }
}

impl Drop for RxPacket<'_> {
    fn drop(&mut self) {
        unsafe {
            let buffer = &self.data[0] as *const BYTE;
            (self.session.lib.release_receive_packet)(self.session.handle, buffer);
        };
    }
}
