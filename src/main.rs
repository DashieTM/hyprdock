use std::{
    io::Read,
    os::unix::net::{UnixListener, UnixStream},
};

fn main() {
    socket_connect();
}

fn socket_connect() { 
    let socket_path = "/var/run/acpid.socket";

    let mut unix_stream = UnixStream::connect(socket_path).expect("Could not create stream");
    let mut buffer = String::new();
    unix_stream
        .read_to_string(&mut buffer)
        .expect("something went wrong");
    println!("{}", buffer);
}
