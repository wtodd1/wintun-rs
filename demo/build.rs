
use std::env;
use std::path::Path;

fn main() {
    let dst_dll = Path::new(&env::var("CARGO_MANIFEST_DIR").unwrap()).join("target/debug/wintun.dll");
    let wintun_dll = Path::new(&env::var("CARGO_MANIFEST_DIR").unwrap()).join("../bin/amd64/wintun.dll");

    std::fs::copy(wintun_dll, dst_dll).ok();
}
