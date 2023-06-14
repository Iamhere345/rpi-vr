use std::net::UdpSocket;

/*

TODO
conf.toml file for story stuff like the raspberry pi's ip addresss and port

*/

/*

Packet Architecture
Len: 4
Data:
1 - constant known by both the client and the server to ensure either device is talking to the correct device
2 - pitch
3 - yaw
4 - roll

*/

const RPI_IP: &str = "127.0.0.1";
const RPI_PORT: &str = "8080";

const PACKET_LEN: usize = 4;

fn main() {
    
    let socket = UdpSocket::bind(format!("{}:{}", RPI_IP, RPI_PORT)).expect("Unable to bind socket to listening port");

    let mut buf = [0; 4];

    loop {
    
        let result_res = socket.recv(&mut buf);

        if result_res.is_err() {
            println!("failed to recv packet");
            continue;
        }

        let result = result_res.unwrap();

        if result != PACKET_LEN {
            println!("unexpected packen len");
        }

    }


}
