#![allow(dead_code)]

use std::net::Ipv4Addr;

use crate::configs::KEAv4PoolVariant;

pub fn v4_pool_to_start_end_available_ips(pool: KEAv4PoolVariant) -> (Ipv4Addr, Ipv4Addr) {
    match pool {
        KEAv4PoolVariant::Range(start, end) => (start, end),
        KEAv4PoolVariant::Cidr(start, prefix) => {
            let ip: u32 = start.into();
            let host_bits = 32 - prefix;

            let netmask = if prefix == 0 {
                0
            } else {
                u32::MAX << host_bits
            };

            let network = ip & netmask;
            let broadcast = network | !netmask;

            match prefix {
                32 => (network.into(), network.into()),
                31 => {
                    let start_ip = network.into();
                    let end_ip = broadcast.into();
                    (start_ip, end_ip)
                }
                _ => {
                    let start_ip = (network + 1).into();
                    let end_ip = (broadcast - 1).into();
                    (start_ip, end_ip)
                }
            }
        }
    }
}

#[cfg(test)]
mod tests {
    use rstest::rstest;
    use std::net::Ipv4Addr;

    use crate::{configs::KEAv4PoolVariant, utils::v4_pool_to_start_end_available_ips};

    #[rstest]
    #[case(
        KEAv4PoolVariant::Cidr(Ipv4Addr::new(0, 0, 0, 0), 0),
        Ipv4Addr::new(0, 0, 0, 1),
        Ipv4Addr::new(255, 255, 255, 254)
    )]
    #[case(
        KEAv4PoolVariant::Cidr(Ipv4Addr::new(1, 0, 0, 0), 8),
        Ipv4Addr::new(1, 0, 0, 1),
        Ipv4Addr::new(1, 255, 255, 254)
    )]
    #[case(
        KEAv4PoolVariant::Cidr(Ipv4Addr::new(1, 2, 0, 0), 16),
        Ipv4Addr::new(1, 2, 0, 1),
        Ipv4Addr::new(1, 2, 255, 254)
    )]
    #[case(
        KEAv4PoolVariant::Cidr(Ipv4Addr::new(1, 2, 3, 0), 24),
        Ipv4Addr::new(1, 2, 3, 1),
        Ipv4Addr::new(1, 2, 3, 254)
    )]
    #[case(
        KEAv4PoolVariant::Cidr(Ipv4Addr::new(1, 2, 3, 10), 28),
        Ipv4Addr::new(1, 2, 3, 1),
        Ipv4Addr::new(1, 2, 3, 14)
    )]
    #[case(
        KEAv4PoolVariant::Cidr(Ipv4Addr::new(1, 2, 3, 30), 32),
        Ipv4Addr::new(1, 2, 3, 30),
        Ipv4Addr::new(1, 2, 3, 30)
    )]
    fn cidr_pool_convert_test(
        #[case] pool: KEAv4PoolVariant,
        #[case] start_ip: Ipv4Addr,
        #[case] end_ip: Ipv4Addr,
    ) {
        assert_eq!(v4_pool_to_start_end_available_ips(pool), (start_ip, end_ip));
    }
}
