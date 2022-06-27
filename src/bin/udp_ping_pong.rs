use std::time::Duration;
use lunatic::{net, Mailbox, Process, sleep};

#[lunatic::main]
fn main(_: Mailbox<()>) {

    Process::spawn("".to_string(), wait_ping);
    
    loop {
        Process::spawn("".to_string(), send_ping);
        sleep(Duration::from_millis(100));
    }
}

fn send_ping(_: String, _: Mailbox<()>) {
    let socket = net::UdpSocket::bind("127.0.0.1:0").unwrap();
    let _local_addr = socket.local_addr().unwrap();

    socket.connect("127.0.0.1:8888").expect("connect function failed");

    loop {
        socket.send("PING".as_bytes()).expect("couldn't send message");
        let mut buf = [0; 4];
        socket.recv(&mut buf).expect("recv error");
        sleep(Duration::from_millis(1000));
    }
}

fn wait_ping(_: String, _: Mailbox<()>) {

    let socket = net::UdpSocket::bind("127.0.0.1:8888").unwrap();
    let _local_addr = socket.local_addr().unwrap();
    
    loop {
        let mut buf = [0; 4];
        let (_len, addr) = socket.recv_from(&mut buf).unwrap();
        let _len_out = socket.send_to("PONG".as_bytes(), addr).unwrap();
    };
}
