extern crate wasm_bindgen;

use wasm_bindgen::prelude::{wasm_bindgen, JsValue};

#[wasm_bindgen]
extern "C" {
    fn alert(s: &str);
}

#[wasm_bindgen]
pub fn greet() {
    alert("Hello, wasm-game-of-life!");
}

mod typing;

#[wasm_bindgen]
pub struct PlacementRuntime {
    problem: crate::typing::Problem,
}

#[wasm_bindgen]
impl PlacementRuntime {
    #[wasm_bindgen(constructor)]
    pub fn new(name: String, blif_string: String, nx: usize, ny: usize) -> Self {
        let blif = typing::BLIFInfo::from_string(name, blif_string);
        let problem = typing::Problem::new(&blif, nx, ny);

        Self { problem }
    }

    #[wasm_bindgen(getter)]
    pub fn nets(&self) -> JsValue {
        JsValue::from_serde(&self.problem.nets).unwrap()
    }

    #[wasm_bindgen(getter)]
    pub fn pins(&self) -> JsValue {
        JsValue::from_serde(&self.problem.pins).unwrap()
    }
}
