
fn main() {
    simple_logger::SimpleLogger::new().env().init().unwrap();

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
            log::info!("got packet (len={})", packet.data.len());
        }
    }
}
