use std::f32::consts::PI;
use enigo::*;

#[cfg(feature = "direct_input_support")]
use crate::windows_mouse::denormalised_mouse_move_to;

pub struct Vec2<T> {
    pub x: T,
    pub y: T
}

impl<T> Vec2<T> {
    pub fn new(x: T, y: T) -> Vec2<T> {
        Vec2 {
            x: x,
            y: y
        }
    }
}

pub fn test(enigo: &mut Enigo) {
    println!("here");
    enigo.mouse_move_to(0, 0);
}

pub fn to_mouse_movement(pitch: f32, yaw: f32, last_pitch: f32, last_yaw: f32, width: i32, height: i32) -> Vec2<f32> {

    let mut delta_pitch = pitch - last_pitch;
    let mut delta_yaw = yaw - last_yaw;

    if (pitch < 1.0 && last_pitch > 358.0) || (pitch > 358.0 && last_pitch < 1.0) {
        delta_pitch = 0.0;
    }

    if (yaw < 1.0 && last_yaw > 358.0) || (yaw > 358.0 && last_yaw < 1.0) {
        delta_yaw = 0.0;
    }

    //println!("delta pitch: {delta_pitch} delta yaw: {delta_yaw}");

    let change_x = delta_yaw.round() * PI * width as f32 / 180.0;
    let mut change_y = delta_pitch.round() * PI * height as f32 / 180.0;

    if change_y >= 500.0 || change_y <= -500.0 {
        println!("change_y was {change_y} with delta pitch {}", delta_pitch.round());
        change_y = 0.0;
    }

    Vec2::new(change_x, -change_y)

}

#[cfg(not(feature = "direct_input_support"))]
pub fn move_mouse_to(enigo: &mut Enigo, pos: Vec2<f32>) {

    if enigo.mouse_location().1 >= 1079 {
         //println!("{}", pos.y.floor() as i32);
    }

    println!("x {} y {}", pos.x, pos.y);

    enigo.mouse_move_relative(pos.x.floor() as i32, pos.y.floor() as i32);
    
}

#[cfg(feature = "direct_input_support")]
pub fn move_mouse_to(enigo: &mut Enigo, pos: Vec2<f32>) {

    let mouse_loc = enigo.mouse_location();

    // the + 1 stops the mouse from moving by itself for some reason
    let transformed_pos = Vec2::<i32>::new(
        (mouse_loc.0 + 1) + pos.x.floor() as i32, 
        (mouse_loc.1 + 1) + pos.y.floor() as i32
    );

    denormalised_mouse_move_to(transformed_pos.x, transformed_pos.y);

}