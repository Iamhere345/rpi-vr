#![allow(dead_code)]
#![allow(unused_variables)]

use std::net::UdpSocket;
use enigo::*;

use mouse::*;

pub mod mouse;

/*
TODO
conf.toml file for story stuff like the raspberry pi's ip addresss and port


Packet Architecture

values are encoded are strings (i.e) pitch is sent as "86.4"
"Pitch_Roll_Yaw"
e.g
"86.4_2.3_22.9"

these values need to be parsed properly to be read as f32s

*/

const RPI_IP: &str = "0.0.0.0";
const RPI_PORT: &str = "8080";

fn main() {

    let mut enigo = Enigo::new();
    let (screen_width, screen_height) = enigo.main_display_size();

    let (mut last_pitch, mut last_yaw): (f32, f32) = (0.0, 0.0);

    println!("width: {} height: {}", screen_width, screen_height);

    let socket = UdpSocket::bind(format!("{}:{}", RPI_IP, RPI_PORT)).expect("Unable to bind socket to listening port");

    let mut buf = [0; 1024];

    'main: loop {
    
        let result_res = socket.recv(&mut buf);

        if result_res.is_err() {
            println!("[ERROR] Failed to recv packet. Error: {}", result_res.unwrap_err());
            continue;
        }

        // get data part of packet (pitch roll and yaw)
        let data_raw = String::from_utf8_lossy(&buf).to_string();

        //println!("{}", data_raw);

        // these shouldn't need to be mut but apparently rust hates me
        let (mut pitch, mut roll, mut yaw): (f32, f32, f32) = (0.0, 0.0, 0.0);

        let data_split = data_raw.splitn(3, '_');

        // there cannot be less than 3 values
        if data_split.clone().collect::<Vec<_>>().len() < 3 {
            println!("[ERROR] Invalid packet data (less than 3 values received)");
            continue;
        }

        for (i, _axis) in data_split.enumerate() {

            let mut axis = _axis.clone();

            // last value contains \0's that make up the rest of the buffer
            // these need to be removed so the value can be parsed
            if i == 2 {
                let axis_trim = axis.split_once('\0');
                axis = match axis_trim {
                    Some(a) => a.0,
                    None => {
                        println!("[WARNING] Buffer full.");
                        axis
                    }
                };
            }

            let axis_parse_result = axis.parse::<f32>();

            let axis_float = match axis_parse_result {
                Ok(f) => f,
                Err(e) => {
                    println!("{:?}", axis.as_bytes());
                    println!("[ERROR] Invalid packet (unable to decode data). Error: {}", e);
                    continue 'main;
                }
            };

            match i {
                0 => pitch = axis_float,
                1 => roll = axis_float,
                2 => yaw = axis_float,
                _ => {/* this shouldn't happen because splitn will never return more than 3 values */}
            }

        }

        println!("pitch: {} roll: {} yaw: {}", pitch, roll, yaw);

        let mouse_change = to_mouse_movement(pitch, yaw, last_pitch, last_yaw, screen_width, screen_height);

        last_pitch = pitch;
        last_yaw = yaw;

        println!("x: {} y: {}", mouse_change.x, mouse_change.y);

        move_mouse_to(&mut enigo, mouse_change);

    }


}
