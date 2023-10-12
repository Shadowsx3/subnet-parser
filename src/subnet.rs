use std::fmt::Display;
use std::net::Ipv4Addr;

#[derive(Debug, Clone)]
pub struct Subnet {
    pub address: Ipv4Addr,
    pub mask: Ipv4Addr,
    pub available_hosts: u32,
    pub broadcast: Ipv4Addr,
    pub host_used: u32,
}

impl Subnet {
    pub(crate) fn new(address: Ipv4Addr, mask: Ipv4Addr, host_used: u32) -> Subnet {
        let available_hosts = !u32::from(mask) - 1;
        let broadcast = Ipv4Addr::from(u32::from(address) + available_hosts + 1);
        Subnet {
            address,
            mask,
            available_hosts,
            broadcast,
            host_used,
        }
    }
}

impl Display for Subnet {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(
            f,
            "Subnet: {{\n\tIp: {}/{}\n\tBroadcast: {}\n\tHosts: {} used of {}\n}}",
            self.address,
            self.mask
                .octets()
                .iter()
                .map(|n| n.count_ones())
                .sum::<u32>(),
            self.broadcast,
            self.host_used,
            self.available_hosts
        )
    }
}