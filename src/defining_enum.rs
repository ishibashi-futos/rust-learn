pub fn defining_enum() {
    let v4 = IpAddrKind::V4;
    let v6 = IpAddrKind::V6;

    let home = IpAddr::home();
    let loopback = IpAddr::loopback();
}

enum IpAddrKind {
    V4,
    V6,
}

fn route(ip_type: IpAddrKind) {
}

struct IpAddr {
    kind: IpAddrKind,
    address: String,
}

impl IpAddr {
    fn home() -> IpAddr {
        IpAddr {
            kind: IpAddrKind::V4,
            address: String::from("127.0.0.1"),
        }
    }

    fn loopback() -> IpAddr {
        IpAddr {
            kind: IpAddrKind::V6,
            address: String::from("::1"),
        }
    }
}
