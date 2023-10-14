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

    pub fn get_hosts(&self) -> Vec<Ipv4Addr> {
        let mut hosts = Vec::new();
        for i in 1..self.available_hosts {
            hosts.push(Ipv4Addr::from(u32::from(self.address) + i));
        }
        hosts
    }

    pub fn generate_dhcp_config(&self) -> String {
        // Construct DHCP configuration commands
        let subnet_ip = self.address;
        let subnet_mask = self.mask;

        format!(
            "ip dhcp pool Subnet_{}/{}\n\
            network {}/{}\n\
            default-router {}\n\
            dns-server {}\n",
            subnet_ip, subnet_mask,
            subnet_ip, subnet_mask,
            subnet_ip, subnet_ip
        )
    }
}

impl Display for Subnet {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(
            f,
            "Subnet: {{\n\tIp: {}/{}\n\tMask: {}\n\tBroadcast: {}\n\tHosts: {} used of {}\n}}",
            self.address,
            self.mask
                .octets()
                .iter()
                .map(|n| n.count_ones())
                .sum::<u32>(),
            self.mask,
            self.broadcast,
            self.host_used,
            self.available_hosts
        )
    }
}