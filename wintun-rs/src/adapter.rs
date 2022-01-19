
use crate::{Wintun, AdapterHandle};
use uuid::Uuid;

use std::sync::Arc;

use crate::wintun_get;
use crate::util::encode_str;

pub struct Adapter {
    pub lib: Arc<Wintun>,
    pub handle: AdapterHandle,
}

impl Adapter {
    pub fn new(name: &str, tunnel_type: &str, uuid: Uuid) -> Self {

        let lib = wintun_get();
        let name = encode_str(name);
        let tunnel_type = encode_str(tunnel_type);

        let handle = unsafe { (lib.create_adapter)(name.as_ptr(), tunnel_type.as_ptr(), &uuid.to_guid()) };

        Self {
            lib: lib,
            handle: handle,
        }
    }

    pub fn open(name: &str) -> Option<Self> {
        let lib = wintun_get();
        let name = encode_str(name);

        let handle = unsafe { (lib.open_adapter)(name.as_ptr()) };
        if handle.is_null() {
            return None;
        }

        Some(Self {
            lib: lib,
            handle: handle,
        })
    }
}

impl Drop for Adapter {
    fn drop(&mut self) {
        println!("dropping adapter");
        unsafe {
            (self.lib.close_adapter)(self.handle);
        }
    }
}
