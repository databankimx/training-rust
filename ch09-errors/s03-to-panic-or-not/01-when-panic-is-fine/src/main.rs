use std::net::IpAddr;

fn main() {
    // The address is written into the program, so we know it is valid. parse still returns a Result,
    // so expect documents why the call can't fail here.
    let home: IpAddr = "127.0.0.1".parse().expect("127.0.0.1 is a valid IP address");
    println!("home address: {home}");

    // Try: change "127.0.0.1" to "not an address". What does expect print?
}
