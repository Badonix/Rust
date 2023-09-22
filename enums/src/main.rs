#[derive(Debug)]
enum IpAddrKind {
    V4,
    V6,
}

#[derive(Debug)]
struct IpAddress {
    kind: IpAddrKind,
    address: String,
}
fn main() {
    let home = IpAddress {
        kind: IpAddrKind::V4,
        address: String::from("127.0.0.1"),
    };
    let loopback = IpAddress {
        kind: IpAddrKind::V6,
        address: String::from("::1"),
    };
    println!("{:?}", home);
    println!("{:?}", loopback);
}

fn route(ip_kind: IpAddrKind) {
    println!("{:?}", ip_kind);
}
