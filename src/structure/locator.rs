pub struct Locator {
    transport: Transport,
    port: u32,
    address: [u8; 16],
}

#[repr(u32)]
pub enum Transport {
    Invalid,
    Reserved,
    Udpv4,
    Udpv6,
    AddressInvalid,
    PortInvalid,
}
