use godot::prelude::{Vector2, Vector2i};

pub trait VectorCast {
    fn to_int_vector(self) -> Vector2i;
    fn to_flt_vector(self) -> Vector2;
}

impl VectorCast for Vector2 {
    fn to_int_vector(self) -> Vector2i {
        let (x, y) = self.to_tuple();
        Vector2i {
            x: x as i32,
            y: y as i32,
        }
    }

    fn to_flt_vector(self) -> Vector2 {
        self
    }
}

impl VectorCast for Vector2i {
    fn to_int_vector(self) -> Vector2i {
        self
    }

    fn to_flt_vector(self) -> Vector2 {
        let (x, y) = self.to_tuple();
        Vector2 {
            x: x as f32,
            y: y as f32,
        }
    }
}
