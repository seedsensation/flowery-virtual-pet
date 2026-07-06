#![allow(dead_code)]
use active_win_pos_rs::get_active_window;
use godot::prelude::*;

#[derive(Debug)]
pub struct Shape {
    pos: Vector2,
    size: Vector2,
}

impl Shape {
    pub fn empty() -> Self {
        Self {
            pos: Vector2 { x: 0f32, y: 0f32 },
            size: Vector2 { x: 0f32, y: 0f32 },
        }
    }

    pub fn top(&self) -> f32 {
        self.pos.y
    }
    pub fn bottom(&self) -> f32 {
        self.pos.y + self.size.y
    }
    pub fn left(&self) -> f32 {
        self.pos.x
    }
    pub fn right(&self) -> f32 {
        self.pos.x + self.size.x
    }
}

pub fn get_window_shape() -> Option<Shape> {
    match get_active_window() {
        Ok(active_window) => Some(Shape {
            pos: Vector2::new(
                active_window.position.x as f32,
                active_window.position.y as f32,
            ),
            size: Vector2::new(
                active_window.position.width as f32,
                active_window.position.height as f32,
            ),
        }),
        Err(()) => None,
    }
}
