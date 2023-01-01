
use wasm_bindgen::prelude::*;
use wasm_bindgen::JsCast;
use web_sys::{HtmlElement};

#[wasm_bindgen]
extern "C" {
    pub fn show_canvas();

    pub fn touch_supported() -> bool;

    pub fn get_up_pressed() -> bool;

    pub fn get_down_pressed() -> bool;

    pub fn get_right_pressed() -> bool;

    pub fn get_left_pressed() -> bool;
}

#[wasm_bindgen]
pub fn check_up() -> bool {
    return get_up_pressed()
}

#[wasm_bindgen]
pub fn check_down() -> bool {
    return get_down_pressed()
}

#[wasm_bindgen]
pub fn check_right() -> bool {
    return get_right_pressed()
}

#[wasm_bindgen]
pub fn check_left() -> bool {
    return get_left_pressed()
}

#[wasm_bindgen]
pub fn toggle_loading()
{
    let window = web_sys::window().expect("no global `window` exists");
    let document = window.document().expect("should have a document window");

    let loading_element_temp = document
        .get_element_by_id("loading")
        .expect("should have loading element");
    
    let loading_element = loading_element_temp
        .dyn_ref::<HtmlElement>()
        .expect("should be HtmlElement");

    loading_element.style().set_property("visibility","hidden").expect("can't set visibility to hidden property");
    
    if touch_supported()
    {
        let controls_element_temp = document
        .get_element_by_id("show_controls")
        .expect("should have show_contorls element");
    
        let controls_element = controls_element_temp
            .dyn_ref::<HtmlElement>()
            .expect("should be HtmlElement");

        controls_element.style().set_property("visibility","visible").expect("can't change visibility to hidden");
    } else {
        let canvas_element_temp = document
        .get_element_by_id("bevy")
        .expect("should have bevy element");

        let canvas_element = canvas_element_temp
            .dyn_ref::<HtmlElement>()
            .expect("should be HtmlElement");

        canvas_element.focus().expect("could not focus");
    }
}

