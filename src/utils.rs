#![allow(dead_code)]

use std::net::{IpAddr, Ipv4Addr};

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

pub fn is_address_in_pool(address: IpAddr, pool: &KEAv4PoolVariant) -> bool {
    match address {
        IpAddr::V4(addr_v4) => {
            let (start_ip, end_ip) = v4_pool_to_start_end_available_ips(*pool);

            addr_v4 >= start_ip && addr_v4 <= end_ip
        }
        IpAddr::V6(_) => false,
    }
}

#[cfg(test)]
mod tests {
    use rstest::rstest;
    use std::{
        net::{IpAddr, Ipv4Addr},
        str::FromStr,
    };

    use crate::configs::KEAv4PoolVariant;

    use super::{is_address_in_pool, v4_pool_to_start_end_available_ips};

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

    #[rstest]
    #[case(
        KEAv4PoolVariant::Range(Ipv4Addr::new(1, 2, 3, 10), Ipv4Addr::new(1, 2, 3, 30)),
        IpAddr::from_str("1.2.3.25").unwrap(),
        true
    )]
    #[case(
        KEAv4PoolVariant::Cidr(Ipv4Addr::new(192, 168, 2, 1), 24),
        IpAddr::from_str("192.168.2.100").unwrap(),
        true
    )]
    #[case(
        KEAv4PoolVariant::Range(Ipv4Addr::new(192, 168, 2, 1), Ipv4Addr::new(192, 168, 10, 100)),
        IpAddr::from_str("192.168.8.25").unwrap(),
        true
    )]
    #[case(
        KEAv4PoolVariant::Range(Ipv4Addr::new(192, 168, 2, 1), Ipv4Addr::new(192, 168, 10, 100)),
        IpAddr::from_str("192.168.15.100").unwrap(),
        false
    )]
    #[case(
        KEAv4PoolVariant::Cidr(Ipv4Addr::new(192, 168, 2, 1), 24),
        IpAddr::from_str("192.168.4.22").unwrap(),
        false
    )]
    fn is_address_in_pool_test(
        #[case] pool: KEAv4PoolVariant,
        #[case] ip: IpAddr,
        #[case] equal: bool,
    ) {
        assert_eq!(is_address_in_pool(ip, &pool), equal);
    }
}
