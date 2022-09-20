mod calculator;

use wasm_bindgen::prelude::*;

// externは、RustからJavaScriptを呼ぶ宣言。
#[wasm_bindgen]
extern "C" {
    pub fn alert(s: &str);
}

// pubは、JavaScriptからRustを呼ぶときに公開メソッドである宣言。
#[wasm_bindgen]
pub fn addition(left: i32, right: i32) {
    let num = calculator::addition(left, right);
    alert(&format!("addition result {}", num));
}
