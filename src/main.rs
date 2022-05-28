use std::fs;

fn main() {
    fs::write("/etc/iwd/main.conf", "[General]
EnableNetworkConfiguration=true
AddressRandomization=network

[Network]
EnableIPv6=true").unwrap();
}
