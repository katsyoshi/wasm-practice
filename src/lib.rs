extern crate wasm_bindgen;
extern crate uuid;

use wasm_bindgen::prelude::*;
use uuid::Uuid;

#[wasm_bindgen]
pub fn fibo(n: i32) -> i32 {
    if n <= 1 {
        1
    } else {
        fibo(n - 1) + fibo(n - 2)
    }
}

#[wasm_bindgen]
pub fn uuidv4() -> String {
    Uuid::new_v4().to_string()
}
