extern crate wasm_bindgen;
extern crate uuid;

use wasm_bindgen::prelude::*;
use uuid::Uuid;

pub fn fibo(n: u128) -> u128 {
    if n <= 1 {
        1
    } else {
        fibo(n - 1) + fibo(n - 2)
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn test_fibo() {
        assert_eq!(fibo(32), 3524578);
    }
}

pub fn uuidv4() -> String {
    Uuid::new_v4().to_string()
}

// Called by our JS entry point to run the example
#[wasm_bindgen(start)]
pub fn run() -> Result<(), JsValue> {
    // Use `web_sys`'s global `window` function to get a handle on the global
    // window object.
    let window = web_sys::window().expect("no global `window` exists");
    let document = window.document().expect("should have a document on window");
    let content = document.get_element_by_id("wasm-from-rust-practice").expect("body shoud have the content");

    // Manufacture the element we're gonna append
    let val = document.create_element("div")?;
    val.set_inner_html("DOM: Hello from Rust!");
    content.append_child(&val)?;

    let val = document.create_element("div")?;
    let uuid = format!("UUID: {}", uuidv4());
    val.set_inner_html(&uuid);
    content.append_child(&val)?;

    let val = document.create_element("div")?;
    val.set_inner_html(&format!("FIBO(32): {}", fibo(32)));
    content.append_child(&val)?;

    let div = document.create_element("div")?;
    let link = document.create_element("a")?;
    div.set_attribute("style", "text-align:right;")?;
    link.set_attribute("href", "/")?;
    link.set_inner_html("back...");
    link.set_attribute("class", "and_more")?;
    div.append_child(&link)?;
    content.append_child(&div)?;

    Ok(())
}
