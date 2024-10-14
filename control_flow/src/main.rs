// defining the type
// the best practice is: 1st declear a struct then use that struct in the enum -
struct Ipv4Addr {
    location: String,
}

struct Ipv6Addr {
    area: (u8, u8),
}

enum IpAddr {
    V4(Ipv4Addr),
    V6(Ipv6Addr),
}
enum Option<T> {
    None,    // dont need to pass
    Some(T), // need to pass something - anything is acceptable
}

#[derive(Debug)]
enum kind {
    v3,
    v4,
}
// expected values in v3 and v4
enum diff_kind {
    v3(String),
    v4(u8, u8, u8, u8), // tuple
}
// defining struct , also using enum value here
struct addr {
    address: String,
    kind: kind,
}

// impl addr -> struct
impl addr {
    fn new(addr: &str) -> Self {
        Self {
            address: addr.to_string(),
            kind: kind::v4,
        }
    }
}
fn main() {
    // preffered:
    // defining address and kind, using struct value
    let random_ip = addr {
        address: String::from("200202"),
        kind: kind::v4,
    };
    // also you can do the same stuff like this: using impl
    // calling impl
    let random_ip_two = addr::new("39399");
    let home_ip = diff_kind::v3(String::from("393939")); // enum is String - v3
    let office_ip = diff_kind::v4(1, 1, 1, 1); //  enum is tuple - v4

    // calling fn route with the name
    route(random_ip);

    // calling route_ip fn:
    // we can call the address like this also, but prefer not to use this
    route_ip("9333", kind::v3);

    let some_number = Some(5);
    let some_char = Some('e');
}

// creating fn which takes struct as ref also doing debug here- :?
fn route(address: addr) {
    println!(
        "the route is {} and the kind is :{:?}",
        address.address, address.kind
    )
}

fn route_ip(ip: &str, kind: kind) {
    println!("the route is :{ip} and the kind is :{kind:?}")
}

// options - pending (will be learning latter :) still a lot to learn about options...
