#![allow(dead_code)]
//use active_win_pos_rs::get_active_window;
use godot::prelude::*;

#[derive(Debug)]
pub struct Shape {
    pub pos: Vector2i,
    pub size: Vector2i,
}

impl Shape {
    pub fn empty() -> Self {
        Self {
            pos: Vector2i { x: 0, y: 0 },
            size: Vector2i { x: 0, y: 0 },
        }
    }

    pub fn top(&self) -> i32 {
        self.pos.y
    }
    pub fn bottom(&self) -> i32 {
        self.pos.y + self.size.y
    }
    pub fn left(&self) -> i32 {
        self.pos.x
    }
    pub fn right(&self) -> i32 {
        self.pos.x + self.size.x
    }
}

//pub fn get_window_shape() -> Option<Shape> {
//    match get_active_window() {
//        Ok(active_window) => Some(Shape {
//            pos: Vector2i::new(
//                active_window.position.x as i32,
//                active_window.position.y as i32,
//            ),
//            size: Vector2i::new(
//                active_window.position.width as i32,
//                active_window.position.height as i32,
//            ),
//        }),
//        Err(()) => None,
//    }
//}
