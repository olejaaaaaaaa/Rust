
pub trait Float {
    fn _into(&self) -> f32;
}

impl Float for i32 {
    fn _into(&self) -> f32 {
        (*self) as f32
    }
}

impl Float for i64 {
    fn _into(&self) -> f32 {
        (*self) as f32
    }
}

impl Float for f64 {
    fn _into(&self) -> f32 {
        (*self) as f32
    }
}

impl Float for f32 {
    fn _into(&self) -> f32 {
        (*self)
    }
}


