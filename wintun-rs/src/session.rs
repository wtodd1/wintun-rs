
use crate::AdapterHandle;
use crate::adapter::Adapter;
use crate::SessionHandle;
use crate::DWORD;
use crate::Wintun;
use crate::HANDLE;
use std::sync::Arc;
use crate::RxPacket;
use winapi::um::synchapi::WaitForSingleObject;
use winapi::um::winbase::INFINITE;

pub struct Session<'a> {
    pub lib: Arc<Wintun>,
    pub handle: SessionHandle,
    _adapter: &'a AdapterHandle,
    event: HANDLE,
}

impl<'a> Session<'a> {
    pub fn new(adapter: &'a Adapter, capacity: usize) -> Self {

        let lib = crate::wintun_get();

        assert!(capacity >= crate::MIN_RING_CAPACITY);
        assert!(capacity <= crate::MAX_RING_CAPACITY);
        let capacity: DWORD = capacity.try_into().unwrap();

        let handle = unsafe {
            let handle = (lib.start_session)(adapter.handle, capacity);
            assert!(handle != 0 as SessionHandle);
            handle
        };

        let event = unsafe {
            (lib.get_read_wait_event)(handle)
        };

        Self {
            lib: lib,
            handle: handle,
            _adapter: &adapter.handle,
            event: event
        }
    }

    pub fn recv(&'a self) -> Option<RxPacket<'a>> {
        loop {

            let mut packet_size: DWORD = 0;
            let packet = {
                
                let packet = unsafe {(self.lib.receive_packet)(self.handle, &mut packet_size as *mut _) };
        
                if packet == 0 as *mut crate::BYTE {
                    let error = unsafe { crate::GetLastError() };
                    match error {
                        crate::error::ERROR_NO_MORE_ITEMS => {
                            unsafe { WaitForSingleObject(self.event, INFINITE) };
                            continue;
                        },
                        _ => { break None; }
                    }
                }
                else {
                    packet
                }
            };

            break Some(RxPacket::new(self, packet, packet_size));
        }
    }

    pub fn send(&'a self, packet: &[u8]) {
        let buffer = unsafe { (self.lib.allocate_send_packet)(self.handle, packet.len().try_into().unwrap()) };
        if buffer == 0 as *mut crate::BYTE {
            return;
        }

        let slice = unsafe { std::slice::from_raw_parts_mut(buffer, packet.len())};
        slice.copy_from_slice(packet);

        unsafe { (self.lib.send_packet)(self.handle, buffer) };
    }
}

impl Drop for Session<'_> {
    fn drop(&mut self) {
        unsafe { (self.lib.end_session)(self.handle) }
    }
}
