use std::net::Ipv4Addr;
use rand::Rng;

pub struct IpDir {
    pub ip: Ipv4Addr,
}

impl IpDir {
    pub fn generate_ip_random_of_class(class: char) -> Ipv4Addr {
        let mut rng = rand::thread_rng();
        let mut ip = Ipv4Addr::new(0, 0, 0, 0);
        match class {
            'A' => {
                ip = Ipv4Addr::new(rng.gen_range(1..127), rng.gen_range(0..255), rng.gen_range(0..255), rng.gen_range(0..255));
            }
            'B' => {
                ip = Ipv4Addr::new(rng.gen_range(128..191), rng.gen_range(0..255), rng.gen_range(0..255), rng.gen_range(0..255));
            }
            'C' => {
                ip = Ipv4Addr::new(rng.gen_range(192..223), rng.gen_range(0..255), rng.gen_range(0..255), rng.gen_range(0..255));
            }
            'D' => {
                ip = Ipv4Addr::new(rng.gen_range(224..239), rng.gen_range(0..255), rng.gen_range(0..255), rng.gen_range(0..255));
            }
            'E' => {
                ip = Ipv4Addr::new(rng.gen_range(240..255), rng.gen_range(0..255), rng.gen_range(0..255), rng.gen_range(0..255));
            }
            _ => {
                println!("Invalid class");
            }
        }
        ip
    }
}



