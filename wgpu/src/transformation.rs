use glam::{Mat4, Vec3, Vec4};
use std::ops::Mul;

/// A 2D transformation matrix.
#[derive(Debug, Clone, Copy, PartialEq)]
pub struct Transformation(Mat4);

impl Transformation {
    /// Get the identity transformation.
    pub fn identity() -> Transformation {
        Transformation(Mat4::identity())
    }

    /// Creates an orthographic projection.
    #[rustfmt::skip]
    pub fn orthographic(width: u16, height: u16) -> Transformation {
        Transformation(Mat4::from_cols(
            Vec4::new(2.0 / f32::from(width), 0.0, 0.0, 0.0),
            Vec4::new(0.0, 2.0 / f32::from(height), 0.0, 0.0),
            Vec4::new(0.0, 0.0, -1.0, 0.0),
            Vec4::new(-1.0, -1.0, 0.0, 1.0)
        ))
    }

    /// Creates a translate transformation.
    pub fn translate(x: f32, y: f32) -> Transformation {
        Transformation(Mat4::from_translation(Vec3::new(x, y, 0.0)))
    }

    /// Creates a scale transformation.
    pub fn scale(x: f32, y: f32) -> Transformation {
        Transformation(Mat4::from_scale(Vec3::new(x, y, 1.0)))
    }
}

impl Mul for Transformation {
    type Output = Self;

    fn mul(self, rhs: Self) -> Self {
        Transformation(self.0 * rhs.0)
    }
}

impl AsRef<[f32; 16]> for Transformation {
    fn as_ref(&self) -> &[f32; 16] {
        self.0.as_ref()
    }
}

impl From<Transformation> for [f32; 16] {
    fn from(t: Transformation) -> [f32; 16] {
        t.as_ref().clone()
    }
}
