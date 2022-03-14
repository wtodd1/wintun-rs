

use log::{info, warn, error};

use winapi::shared::ifdef::NET_LUID;
use winapi::shared::minwindef::{BOOL, FARPROC, HMODULE};
use winapi::um::libloaderapi::{FreeLibrary, GetProcAddress, LoadLibraryW};

pub use winapi::um::errhandlingapi::GetLastError;

pub mod error {
    pub use winapi::shared::winerror::*;
}

#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct _WINTUN_ADAPTER {
    _unused: [u8; 0],
}
pub type AdapterHandle = *mut _WINTUN_ADAPTER;

#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct _TUN_SESSION {
    _unused: [u8; 0],
}
pub type SessionHandle = *mut _TUN_SESSION;

pub type BYTE = winapi::shared::minwindef::BYTE;
pub type GUID = winapi::shared::guiddef::GUID;
pub type DWORD = winapi::shared::minwindef::DWORD;
pub type DWORD64 = winapi::shared::basetsd::DWORD64;
pub type LPCWSTR = winapi::shared::ntdef::LPCWSTR;
pub type HANDLE = winapi::shared::ntdef::HANDLE;

#[repr(C)]
pub enum LoggerLevel {
    Info,
    Warn,
    Error,
}

pub type LoggerCallback = unsafe extern "C" fn(LoggerLevel, DWORD64, LPCWSTR);

pub const MIN_RING_CAPACITY: usize = 0x20000;
pub const MAX_RING_CAPACITY: usize = 0x4000000;
pub const MAX_IP_PACKET_SIZE: DWORD = 0xFFFF;

type WintunCreateAdapter =
    unsafe extern "C" fn(*const u16, *const u16, *const GUID) -> AdapterHandle;
type WintunOpenAdapter = unsafe extern "C" fn(*const u16) -> AdapterHandle;
type WintunCloseAdapter = unsafe extern "C" fn(AdapterHandle);
type WintunDeleteDriver = unsafe extern "C" fn() -> BOOL;
type WintunGetAdapterLUID = unsafe extern "C" fn(AdapterHandle, *mut NET_LUID);
type WintunGetRunningDriverVersion = unsafe extern "C" fn() -> DWORD;
type WintunSetLogger = unsafe extern "C" fn(LoggerCallback);
type WintunStartSession = unsafe extern "C" fn(AdapterHandle, DWORD) -> SessionHandle;
type WintunEndSession = unsafe extern "C" fn(SessionHandle);
type WintunGetReadWaitEvent = unsafe extern "C" fn(SessionHandle) -> HANDLE;
type WintunReceivePacket = unsafe extern "C" fn(SessionHandle, *mut DWORD) -> *mut BYTE;
type WintunReleaseReceivePacket = unsafe extern "C" fn(SessionHandle, *const BYTE);
type WintunAllocateSendPacket = unsafe extern "C" fn(SessionHandle, DWORD) -> *mut BYTE;
type WintunSendPacket = unsafe extern "C" fn(SessionHandle, *const BYTE);

pub struct Wintun {
    lib: HMODULE,
    pub create_adapter: WintunCreateAdapter,
    pub open_adapter: WintunOpenAdapter,
    pub close_adapter: WintunCloseAdapter,
    pub delete_driver: WintunDeleteDriver,
    pub get_adapter_luid: WintunGetAdapterLUID,
    pub get_running_driver_version: WintunGetRunningDriverVersion,
    pub set_logger: WintunSetLogger,
    pub start_session: WintunStartSession,
    pub end_session: WintunEndSession,
    pub get_read_wait_event: WintunGetReadWaitEvent,
    pub receive_packet: WintunReceivePacket,
    pub release_receive_packet: WintunReleaseReceivePacket,
    pub allocate_send_packet: WintunAllocateSendPacket,
    pub send_packet: WintunSendPacket,
}

unsafe extern "C" fn wintun_logger_cb(level: LoggerLevel, _time: DWORD64, message: LPCWSTR) {
    let message = crate::util::decode_str(message).into_string().unwrap();

    match level {
        LoggerLevel::Info => info!("{}", message),
        LoggerLevel::Warn => warn!("{}", message),
        LoggerLevel::Error => error!("{}", message),
    }
}

impl Wintun {
    pub fn new() -> Wintun {
        use std::ffi::OsStr;
        use std::iter::once;
        use std::os::windows::ffi::OsStrExt;

        let lib_path: Vec<u16> = OsStr::new("wintun.dll")
            .encode_wide()
            .chain(once(0))
            .collect();
        let lib = unsafe { LoadLibraryW(lib_path.as_ptr()) };
        assert!(lib != 0 as HMODULE);

        macro_rules! load_func {
            ($lib:ident, $x:ident) => { unsafe {
                let name = std::ffi::CString::new(stringify!($x)).unwrap();
                let proc = GetProcAddress($lib, name.as_ptr());
                assert!(proc != 0 as FARPROC);
                std::mem::transmute::<FARPROC, $x>(proc)
            }};
        }

        let wintun = Wintun {
            lib: lib,
            create_adapter: load_func!(lib, WintunCreateAdapter),
            open_adapter: load_func!(lib, WintunOpenAdapter),
            close_adapter: load_func!(lib, WintunCloseAdapter),
            delete_driver: load_func!(lib, WintunDeleteDriver),
            get_adapter_luid: load_func!(lib, WintunGetAdapterLUID),
            get_running_driver_version: load_func!(lib, WintunGetRunningDriverVersion),
            set_logger: load_func!(lib, WintunSetLogger),
            start_session: load_func!(lib, WintunStartSession),
            end_session: load_func!(lib, WintunEndSession),
            get_read_wait_event: load_func!(lib, WintunGetReadWaitEvent),
            receive_packet: load_func!(lib, WintunReceivePacket),
            release_receive_packet: load_func!(lib, WintunReleaseReceivePacket),
            allocate_send_packet: load_func!(lib, WintunAllocateSendPacket),
            send_packet: load_func!(lib, WintunSendPacket),
        };

        unsafe {
            (wintun.set_logger)(wintun_logger_cb);
        }
        
        wintun
    }
}

impl Drop for Wintun {
    fn drop(&mut self) {
        unsafe { (self.delete_driver)(); };
        unsafe { FreeLibrary(self.lib); };
        self.lib = 0 as HMODULE;
    }
}

unsafe impl Send for Wintun {}
unsafe impl Sync for Wintun {}

