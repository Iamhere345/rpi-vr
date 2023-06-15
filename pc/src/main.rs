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
3 - roll
4 - yaw

*/

const RPI_IP: &str = "127.0.0.1";
const RPI_PORT: &str = "8080";

const PACKET_LEN: usize = 4;

struct Vec2<T> {
    x: T,
    y: T
}

fn to_mouse_movement(pitch: f32, roll: f32, yaw: f32) -> (f32, f32) {

    // TODO
    (0.0, 0.0)

}

// TODO
fn move_mouse(move_by: (f32, f32)) -> Option<_> {
    None
}

fn main() {
    
    let socket = UdpSocket::bind(format!("{}:{}", RPI_IP, RPI_PORT)).expect("Unable to bind socket to listening port");

    let mut buf: [f32; 4] = [0.0; 4];

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

        // get data part of packet (pitch roll and yaw)
        let buf = &mut buf[1..3];

        let (x, y) = to_mouse_movement(buf[0], buf[1], buf[2]);
        move_mouse((x, y));

    }


}
