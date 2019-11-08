use std::net::{IpAddr, Ipv4Addr, Ipv6Addr};

pub trait Ipv4AddrExt {
    fn is_shared(&self) -> bool;
    fn is_ietf_protocol_assignment(&self) -> bool;
    fn is_reserved(&self) -> bool;
    fn is_benchmarking(&self) -> bool;
}

impl Ipv4AddrExt for Ipv4Addr {
    fn is_shared(&self) -> bool {
        self.octets()[0] == 100 && (self.octets()[1] & 0b1100_0000 == 0b0100_0000)
    }

    fn is_ietf_protocol_assignment(&self) -> bool {
        self.octets()[0] == 192 && self.octets()[1] == 0 && self.octets()[2] == 0
    }

    fn is_reserved(&self) -> bool {
        self.octets()[0] & 240 == 240 && !self.is_broadcast()
    }

    fn is_benchmarking(&self) -> bool {
        self.octets()[0] == 198 && (self.octets()[1] & 0xfe) == 18
    }
}

impl IpExt for Ipv4Addr {
    fn is_global(&self) -> bool {
        if u32::from(*self) == 0xc000_0009 || u32::from(*self) == 0xc000_000a {
            return true;
        }
        !self.is_private()
            && !self.is_loopback()
            && !self.is_link_local()
            && !self.is_broadcast()
            && !self.is_documentation()
            && !Ipv4AddrExt::is_shared(self)
            && !Ipv4AddrExt::is_ietf_protocol_assignment(self)
            && !Ipv4AddrExt::is_reserved(self)
            && !Ipv4AddrExt::is_benchmarking(self)
            && self.octets()[0] != 0
    }
}

pub enum Ipv6MulticastScope {
    InterfaceLocal,
    LinkLocal,
    RealmLocal,
    AdminLocal,
    SiteLocal,
    OrganizationLocal,
    Global,
}

pub trait Ipv6AddrExt {
    fn multicast_scope(&self) -> Option<Ipv6MulticastScope>;
    fn is_unicast_link_local(&self) -> bool;
    fn is_unique_local(&self) -> bool;
    fn is_unicast_global(&self) -> bool;
    fn is_documentation(&self) -> bool;
}

impl Ipv6AddrExt for Ipv6Addr {
    fn multicast_scope(&self) -> Option<Ipv6MulticastScope> {
        if self.is_multicast() {
            match self.segments()[0] & 0x000f {
                1 => Some(Ipv6MulticastScope::InterfaceLocal),
                2 => Some(Ipv6MulticastScope::LinkLocal),
                3 => Some(Ipv6MulticastScope::RealmLocal),
                4 => Some(Ipv6MulticastScope::AdminLocal),
                5 => Some(Ipv6MulticastScope::SiteLocal),
                8 => Some(Ipv6MulticastScope::OrganizationLocal),
                14 => Some(Ipv6MulticastScope::Global),
                _ => None,
            }
        } else {
            None
        }
    }

    fn is_unicast_link_local(&self) -> bool {
        (self.segments()[0] & 0xffc0) == 0xfe80
    }

    fn is_unique_local(&self) -> bool {
        (self.segments()[0] & 0xfe00) == 0xfc00
    }

    fn is_unicast_global(&self) -> bool {
        !self.is_multicast()
            && !self.is_loopback()
            && !Ipv6AddrExt::is_unicast_link_local(self)
            && !Ipv6AddrExt::is_unique_local(self)
            && !self.is_unspecified()
            && !Ipv6AddrExt::is_documentation(self)
    }

    fn is_documentation(&self) -> bool {
        (self.segments()[0] == 0x2001) && (self.segments()[1] == 0xdb8)
    }
}

impl IpExt for Ipv6Addr {
    fn is_global(&self) -> bool {
        match Ipv6AddrExt::multicast_scope(self) {
            Some(Ipv6MulticastScope::Global) => true,
            None => Ipv6AddrExt::is_unicast_global(self),
            _ => false,
        }
    }
}

pub trait IpExt {
    fn is_global(&self) -> bool;
}

impl IpExt for IpAddr {
    fn is_global(&self) -> bool {
        match self {
            IpAddr::V4(ip) => IpExt::is_global(ip),
            IpAddr::V6(ip) => IpExt::is_global(ip),
        }
    }
}
