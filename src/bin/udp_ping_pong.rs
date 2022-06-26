use std::time::Duration;
use lunatic::{net, Mailbox, Process, sleep};

#[lunatic::main]
fn main(_: Mailbox<()>) {

    Process::spawn("".to_string(), wait_ping);
    
    loop {
        Process::spawn("".to_string(), send_ping);
        sleep(Duration::from_millis(1000));
    }
}

fn send_ping(_: String, _: Mailbox<()>) {
    let socket = net::UdpSocket::bind("127.0.0.1:0").unwrap();
    let local_addr = socket.local_addr().unwrap();
    println!("send_ping<{}> UdpSocket bound", &local_addr);    
    socket.connect("127.0.0.1:8888").expect("connect function failed");

    loop {
        socket.send("PING".as_bytes()).expect("couldn't send message");
        println!("send_ping<{}> PING Sent", &local_addr);
        
        let mut buf = [0; 4];
        match socket.recv(&mut buf) {
            Ok(received) => println!("send_ping<{}> PONG {received} bytes {:?}", &local_addr, &buf[..received]),
            Err(e) => println!("send_ping<{}> PONG recv function failed: {e:?}", &local_addr),
        }
        
        sleep(Duration::from_millis(1000));
    }
}

fn wait_ping(_: String, _: Mailbox<()>) {

    let socket = net::UdpSocket::bind("127.0.0.1:8888").unwrap();
    let local_addr = socket.local_addr().unwrap();
    println!("wait_ping<{}>> UdpSocket bound", &local_addr);
    
    loop {
        let mut buf = [0; 4];
        let (len, addr) = socket.recv_from(&mut buf).unwrap();
        println!("wait_ping<{}> UdpSocket received {:?} bytes from {:?}", &local_addr, &len, &addr);

        let len_out = socket.send_to("PONG".as_bytes(), addr).unwrap();
        println!("wait_ping<{}> UdpSocket replied PONG {:?} bytes to {:?}", &local_addr, &len_out, &addr);        
    };
}
