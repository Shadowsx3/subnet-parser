use std::env;
use std::net::{Ipv4Addr};
use crate::generator::SubnetGenerator;

pub mod subnet;
pub mod generator;
pub mod ip_dir;

fn main() {
    let args: Vec<String> = env::args().collect();

    if args.len() < 4 {
        eprintln!("Usage: {} <base_address> <base_mask> <required_hosts...>", args[0]);
        std::process::exit(1);
    }

    // Parse command line arguments
    let base_address_str = &args[1];
    let base_mask_str = &args[2];
    let required_hosts: Vec<u32> = args[3..].iter().filter_map(|s| s.parse().ok()).collect();

    let base_address = match base_address_str.parse::<Ipv4Addr>() {
        Ok(addr) => addr,
        Err(err) => {
            eprintln!("Invalid base address: {}", err);
            std::process::exit(1);
        }
    };

    let base_mask = match base_mask_str.parse::<Ipv4Addr>() {
        Ok(mask) => mask,
        Err(err) => {
            eprintln!("Invalid base mask: {}", err);
            std::process::exit(1);
        }
    };

    if required_hosts.is_empty() {
        eprintln!("No valid required hosts provided.");
        std::process::exit(1);
    }

    let list = SubnetGenerator::generate_subnets(base_address, base_mask, required_hosts);
    for subnet in list {
        println!("{}", subnet);
    }
}
