use std::ops::{Sub, Add};
use ndarray::Array2;
use crate::matrixutil::{power_of, scalar_mult, create_weight};

// enum storing each cost function
pub enum Cost {
    MSE,
}

impl Cost {

    // MSE(xᵢ,yᵢ) = 1/n Σ(i=0;n) (yᵢ-ŷᵢ)^2
    pub fn calculate(&self, predicted: &Vec<Vec<Vec<Array2<f32>>>>, expected: &Vec<Array2<f32>>) -> f32{
        match self {
            Cost::MSE => { 
                let mut outp: f32 = 0f32;
                for i in 0..predicted.len() {
                    let error = expected[i].clone().sub(predicted[i][1].last().unwrap());
                    let squared_error = power_of(error, 2);
                    outp += squared_error.sum();
                }
                outp / predicted.len() as f32
            },
        }
    }

    // I'm confused because it seems like it's supposed to be a scalar but it's not idk bro leave me alone
    // ∂MSE/∂ŷ = -2/n Σ(i=0;n) (yᵢ-ŷᵢ)
    pub fn derivate(&self, predicted: &Vec<Vec<Vec<Array2<f32>>>>, expected: &Vec<Array2<f32>>) -> Array2<f32> {
        match self {
            Cost::MSE => {
                let m = expected[0].shape();
                let mut outp = create_weight(vec![m[0],m[1]]);
                for i in 0..predicted.len() {
                    let error = expected[i].clone().sub(predicted[i][1].last().unwrap());
                    let derivative_squared_error = scalar_mult(error, -2f32/predicted.len() as f32);
                    outp = outp.clone().add(derivative_squared_error);
                }
                outp
            } 
        }
    }
}
