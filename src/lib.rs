use ndarray::prelude::*;
use wasm_bindgen::prelude::*;
mod utils;
use utils::debug::console;

#[wasm_bindgen]
pub fn zeros(shape: &[usize]) {
    let a = Dim(shape);
    let mut b = Array::<i32, _>::zeros(a.f());
    b[[0, 0]] = 7;
    console(b);
}