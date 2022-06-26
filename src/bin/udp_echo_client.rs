use lunatic::{net, Mailbox};

#[lunatic::main]
fn main(_: Mailbox<()>) {
    let socket = net::UdpSocket::bind("127.0.0.1:0").unwrap();
    println!("UdpSocket bound on addr: {}", socket.local_addr().unwrap());

    socket.connect("127.0.0.1:8888").expect("connect function failed");
    socket.send(&[72, 101, 108, 108, 111, 10]).expect("couldn't send message");

    println!("sent Hello");
}
