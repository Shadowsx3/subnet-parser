use std::net::Ipv4Addr;
use crate::subnet::Subnet;

pub struct SubnetGenerator;

impl SubnetGenerator {
    pub fn generate_subnets(
        base_address: Ipv4Addr,
        max_mask: Ipv4Addr,
        host_required: Vec<u32>,
    ) -> Vec<Subnet> {
        println!("SubnetGenerator::generate_subnets");
        let mut host_required = host_required;
        host_required.sort();
        host_required.reverse();
        let mut new_base_address = base_address;

        host_required.iter().map(move |n| {
            let base_ip = Ipv4Addr::from(new_base_address);
            let mask_number = 32 - (n + 2).next_power_of_two().trailing_zeros();
            let mask = Ipv4Addr::from(!0 << (32 - mask_number));
            let subnet = Subnet::new(base_ip, mask, n.clone());
            new_base_address = Ipv4Addr::from(u32::from(subnet.broadcast) + 1);
            subnet
        }).collect()
    }
}
