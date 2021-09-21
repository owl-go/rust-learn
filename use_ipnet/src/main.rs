use ipnet::{IpNet, Ipv4Net, Ipv6Net};
use std::net::{Ipv4Addr, Ipv6Addr};
use std::str::FromStr;
fn main() {
    // Create an Ipv4Net and Ipv6Net from their constructors.

    let net4 = Ipv4Net::new(Ipv4Addr::new(10, 1, 1, 0), 24).unwrap();
    let net6 = Ipv6Net::new(Ipv6Addr::new(0xfd, 0, 0, 0, 0, 0, 0, 0), 24).unwrap();

    // They can also be created from string representations.

    let net4 = Ipv4Net::from_str("10.1.1.0/24").unwrap();
    let net6 = Ipv6Net::from_str("fd00::/24").unwrap();

    // Or alternatively as follows.
    let net4: Ipv4Net = "10.1.1.0/24".parse().unwrap();
    let net6: Ipv6Net = "fd00::/24".parse().unwrap();

    // IpNet can represent either an IPv4 or IPv6 network address.

    let net = IpNet::from(net4);
    // It can also be created from string representations.

    let net = IpNet::from_str("10.1.1.0/24").unwrap();
    let net: IpNet = "10.1.1.0/24".parse().unwrap();

    // There are a number of methods that can be used. Read the
    // documentation for the full details.

    println!("{} hostmask = {}", net, net.hostmask());
    println!("{} netmask = {}", net4, net4.netmask());
}
