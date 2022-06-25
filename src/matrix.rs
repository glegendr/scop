use core::iter::{Product, Sum};
use std::ops::Mul;

#[derive(Debug)]
pub struct Matrix {
    x: [f32; 4],
    y: [f32; 4],
    z: [f32; 4],
    w: [f32; 4],
}

impl Matrix {
    pub fn default() -> Self {
        Matrix {
            x: [1., 0., 0., 0.],
            y: [0., 1., 0., 0.],
            z: [0., 0., 1., 0.],
            w: [0., 0., 0., 1.],
        }
    }

    pub fn to_cols_array_2d(&self) -> [[f32; 4]; 4] {
        [
            self.x,
            self.y,
            self.z,
            self.w
        ]
    }

    pub fn from_translation(translation: [f32; 3]) -> Self {
        let mut ret = Self::default();
        ret.w = [translation[0], translation[1], translation[2], 1.0];
        ret
    }


    pub fn from_rotation_x(rot: f32) -> Self {
        Matrix {
                x: [1.0, 0.0, 0.0, 0.0],
                y: [0.0, rot.cos(), -rot.sin(), 0.0],
                z: [0.0, rot.sin(), rot.cos(), 0.0],
                w: [0.0, 0.0, 0.0, 1.0],
        }
    }

    pub fn from_rotation_y(rot: f32) -> Self {
        Matrix {
            x: [rot.cos(), 0.0, -rot.sin(), 0.0],
            y: [0.0, 1.0, 0.0, 0.0],
            z: [rot.sin(), 0.0, rot.cos(), 0.0],
            w: [0.0, 0.0, 0.0, 1.],
        }
    }

    pub fn from_rotation_z(rot: f32) -> Self {
        Matrix {
            x: [rot.cos(), -rot.sin(), 0.0, 0.0],
            y: [rot.sin(), rot.cos(), 0.0, 0.0],
            z: [0.0, 0.0, 1.0, 0.0],
            w: [0.0, 0.0, 0.0, 1.0],
        }
    }
    pub fn multiply(&self, by: &Self) -> Self {
        Matrix {
            x: [
                self.x[0] * by.x[0] + self.x[1] * by.y[0] + self.x[2] * by.z[0] + self.x[3] * by.w[0],
                self.x[0] * by.x[1] + self.x[1] * by.y[1] + self.x[2] * by.z[1] + self.x[3] * by.w[2],
                self.x[0] * by.x[2] + self.x[1] * by.y[2] + self.x[2] * by.z[2] + self.x[3] * by.w[2],
                self.x[0] * by.x[3] + self.x[1] * by.y[3] + self.x[2] * by.z[3] + self.x[3] * by.w[3],
            ],
            y: [
                self.y[0] * by.x[0] + self.y[1] * by.y[0] + self.y[2] * by.z[0] + self.y[3] * by.w[0],
                self.y[0] * by.x[1] + self.y[1] * by.y[1] + self.y[2] * by.z[1] + self.y[3] * by.w[2],
                self.y[0] * by.x[2] + self.y[1] * by.y[2] + self.y[2] * by.z[2] + self.y[3] * by.w[2],
                self.y[0] * by.x[3] + self.y[1] * by.y[3] + self.y[2] * by.z[3] + self.y[3] * by.w[3],
            ],
            z: [
                self.z[0] * by.x[0] + self.z[1] * by.y[0] + self.z[2] * by.z[0] + self.z[3] * by.w[0],
                self.z[0] * by.x[1] + self.z[1] * by.y[1] + self.z[2] * by.z[1] + self.z[3] * by.w[2],
                self.z[0] * by.x[2] + self.z[1] * by.y[2] + self.z[2] * by.z[2] + self.z[3] * by.w[2],
                self.z[0] * by.x[3] + self.z[1] * by.y[3] + self.z[2] * by.z[3] + self.z[3] * by.w[3],
            ],
            w: [
                self.w[0] * by.x[0] + self.w[1] * by.y[0] + self.w[2] * by.z[0] + self.w[3] * by.w[0],
                self.w[0] * by.x[1] + self.w[1] * by.y[1] + self.w[2] * by.z[1] + self.w[3] * by.w[2],
                self.w[0] * by.x[2] + self.w[1] * by.y[2] + self.w[2] * by.z[2] + self.w[3] * by.w[2],
                self.w[0] * by.x[3] + self.w[1] * by.y[3] + self.w[2] * by.z[3] + self.w[3] * by.w[3],
            ],
        }
    }
}

impl<'a> Product<&'a Self> for Matrix {
    fn product<I>(iter: I) -> Self
    where
        I: Iterator<Item = &'a Self>,
    {
        iter.fold(Self::default(), |a, b| Self::multiply(&a, &b))
    }
}