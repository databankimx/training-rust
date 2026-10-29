#[derive(Debug)]
enum IpAddrKind {
    V4,
    V6,
}

// Both variants are the same type, IpAddrKind, so one function can accept either.
fn route(ip_kind: IpAddrKind) {
    println!("routing {ip_kind:?}");
}

fn main() {
    let four = IpAddrKind::V4;
    let six = IpAddrKind::V6;

    route(four);
    route(six);

    // Try: add a third variant, V8, to the enum. Does route still compile, and what does a call with V8 print?
}
