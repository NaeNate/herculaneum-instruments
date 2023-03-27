use wasm_bindgen::prelude::wasm_bindgen;

#[wasm_bindgen]
pub fn add(x: f64, y: f64) -> f64 {
    x + y
}

#[wasm_bindgen]
pub fn sub(x: f64, y: f64) -> f64 {
    x - y
}

#[wasm_bindgen]
pub fn mul(x: f64, y: f64) -> f64 {
    x * y
}

#[wasm_bindgen]
pub fn div(x: f64, y: f64) -> f64 {
    x / y
}
