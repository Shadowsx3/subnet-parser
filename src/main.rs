use std::net::Ipv4Addr;
use crate::generator::SubnetGenerator;

pub mod subnet;
pub mod generator;

fn main() {
    println!("Hello, world!");
    let base_address = Ipv4Addr::new(10, 1, 1, 0);
    let base_mask = Ipv4Addr::new(255, 255, 255, 0);
    let required_hosts = vec![58, 49, 29, 14, 3, 260];
    let list = SubnetGenerator::generate_subnets(base_address, base_mask, required_hosts);
    for subnet in list {
        println!("{}", subnet);
    }
}
