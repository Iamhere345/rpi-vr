use std::f32::consts::PI;
use enigo::*;

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

pub fn to_mouse_movement(mut pitch: f32, yaw: f32, last_pitch: f32, last_yaw: f32, width: i32, height: i32) -> Vec2<f32> {

    //pitch *= -1.0;

    let delta_pitch = pitch - last_pitch;
    let delta_yaw = yaw - last_yaw;

    let change_x = delta_pitch * PI * width as f32 / 180.0;
    let change_y = delta_yaw * PI * height as f32 / 180.0;

    Vec2::new(change_x, change_y)

}

pub fn move_mouse_to(enigo: &mut Enigo, pos: Vec2<f32>) {

    enigo.mouse_move_relative(pos.x.floor() as i32, pos.y.floor() as i32);

}