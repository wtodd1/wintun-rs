use std::sync::Arc;
use std::sync::Once;
use crate::Wintun;

static LIBRARY_ONCE: Once = Once::new();
static mut LIBRARY: Option<Arc<Wintun>> = None;

pub fn wintun_get() -> Arc<Wintun> {
    let lib = unsafe {
        LIBRARY_ONCE.call_once(|| {
            LIBRARY = Some(Arc::new(Wintun::new()));
        });

        LIBRARY.as_ref().unwrap().clone()
    };

    lib
}
