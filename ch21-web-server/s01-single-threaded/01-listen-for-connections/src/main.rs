use std::net::TcpListener;

fn main() {
    // bind listens on this address. The port 7878 is not normally used by HTTP servers, so it's unlikely to clash.
    let listener = TcpListener::bind("127.0.0.1:7878").unwrap();

    // incoming gives one stream for each connection attempt. The stream is closed when it goes out of scope.
    for stream in listener.incoming() {
        let _stream = stream.unwrap();

        println!("Connection established!");
    }

    // Try: open http://127.0.0.1:7878 in a browser. How many 'Connection established!' lines appear for one page load?
}
