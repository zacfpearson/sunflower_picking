
use wasm_bindgen::prelude::*;
use wasm_bindgen::JsCast;
use web_sys::{HtmlElement};

#[wasm_bindgen]
pub fn setup_web_controls() {
    let window = web_sys::window().expect("no global `window` exists");
    let document = window.document().expect("should have a document window");

    let up_element_temp = document
        .get_element_by_id("up")
        .expect("should have up element");
    let up_element_temp_2 = document
        .get_element_by_id("up")
        .expect("should have up element");
    let up_element_temp_3 = document
        .get_element_by_id("up")
        .expect("should have up element");
    let up_element_temp_4 = document
        .get_element_by_id("up")
        .expect("should have up element");
    let up_element_temp_5 = document
        .get_element_by_id("up")
        .expect("should have up element");
    let up_element_temp_6 = document
        .get_element_by_id("up")
        .expect("should have up element");


    let up_element = up_element_temp_2
        .dyn_ref::<HtmlElement>()
        .expect("should be HtmlElement");

    let right_element_temp = document
        .get_element_by_id("right")
        .expect("should have right element");
    let right_element_temp_2 = document
        .get_element_by_id("right")
        .expect("should have right element");
    let right_element_temp_3 = document
        .get_element_by_id("right")
        .expect("should have right element");
    let right_element_temp_4 = document
        .get_element_by_id("right")
        .expect("should have right element");
    let right_element_temp_5 = document
        .get_element_by_id("right")
        .expect("should have right element");
    let right_element_temp_6 = document
        .get_element_by_id("right")
        .expect("should have right element");

    let right_element = right_element_temp_2
        .dyn_ref::<HtmlElement>()
        .expect("should be HtmlElement");

    let down_element_temp = document
        .get_element_by_id("down")
        .expect("should have down element");
    let down_element_temp_2 = document
        .get_element_by_id("down")
        .expect("should have down element");
    let down_element_temp_3 = document
        .get_element_by_id("down")
        .expect("should have down element");
    let down_element_temp_4 = document
        .get_element_by_id("down")
        .expect("should have down element");
    let down_element_temp_5 = document
        .get_element_by_id("down")
        .expect("should have down element");
    let down_element_temp_6 = document
        .get_element_by_id("down")
        .expect("should have down element");

    let down_element = down_element_temp_2
        .dyn_ref::<HtmlElement>()
        .expect("should be HtmlElement");

    let left_element_temp = document
        .get_element_by_id("left")
        .expect("should have left element");
    let left_element_temp_2 = document
        .get_element_by_id("left")
        .expect("should have left element");
    let left_element_temp_3 = document
        .get_element_by_id("left")
        .expect("should have left element");
    let left_element_temp_4 = document
        .get_element_by_id("left")
        .expect("should have left element");
    let left_element_temp_5 = document
        .get_element_by_id("left")
        .expect("should have left element");
    let left_element_temp_6 = document
        .get_element_by_id("left")
        .expect("should have left element");

    let left_element = left_element_temp_2
        .dyn_ref::<HtmlElement>()
        .expect("should be HtmlElement");

    let press_up = Closure::<dyn FnMut()>::new(move || {
        up_element_temp
            .dyn_ref::<HtmlElement>()
            .expect("should be HtmlElement").style()
        .set_property("background-color", "#FD7014")
        .expect("should have style help");

    });
    let release_up = Closure::<dyn FnMut()>::new(move || {
        up_element_temp_3
            .dyn_ref::<HtmlElement>()
            .expect("should be HtmlElement").style()
        .set_property("background-color", "transparent")
        .expect("should have style help");
    });

    let out_up = Closure::<dyn FnMut()>::new(move || {
        up_element_temp_4
            .dyn_ref::<HtmlElement>()
            .expect("should be HtmlElement").style()
        .set_property("background-color", "transparent")
        .expect("should have style help");
    });

    let touch_start_up = Closure::<dyn FnMut()>::new(move || {
        up_element_temp_5
            .dyn_ref::<HtmlElement>()
            .expect("should be HtmlElement").style()
        .set_property("background-color", "#FD7014")
        .expect("should have style help");
    });

    let touch_end_up = Closure::<dyn FnMut()>::new(move || {
        up_element_temp_6
            .dyn_ref::<HtmlElement>()
            .expect("should be HtmlElement").style()
        .set_property("background-color", "transparent")
        .expect("should have style help");
    });

    let press_right = Closure::<dyn FnMut()>::new(move || {
        right_element_temp
        .dyn_ref::<HtmlElement>()
        .expect("should be HtmlElement").style()
        .set_property("background-color", "#FD7014")
        .expect("should have style help");
    });
    let release_right = Closure::<dyn FnMut()>::new(move || {
        right_element_temp_3
        .dyn_ref::<HtmlElement>()
        .expect("should be HtmlElement").style()
        .set_property("background-color", "transparent")
        .expect("should have style help");
    });
    let out_right = Closure::<dyn FnMut()>::new(move || {
        right_element_temp_4
        .dyn_ref::<HtmlElement>()
        .expect("should be HtmlElement").style()
        .set_property("background-color", "transparent")
        .expect("should have style help");
    });

    let touch_start_right = Closure::<dyn FnMut()>::new(move || {
        right_element_temp_5
            .dyn_ref::<HtmlElement>()
            .expect("should be HtmlElement").style()
        .set_property("background-color", "#FD7014")
        .expect("should have style help");
    });

    let touch_end_right = Closure::<dyn FnMut()>::new(move || {
        right_element_temp_6
            .dyn_ref::<HtmlElement>()
            .expect("should be HtmlElement").style()
        .set_property("background-color", "transparent")
        .expect("should have style help");
    });
        
    let press_down = Closure::<dyn FnMut()>::new(move || {
        down_element_temp
            .dyn_ref::<HtmlElement>()
            .expect("should be HtmlElement").style()
        .set_property("background-color", "#FD7014")
        .expect("should have style help");
    });
    let release_down = Closure::<dyn FnMut()>::new(move || {
        down_element_temp_3
        .dyn_ref::<HtmlElement>()
        .expect("should be HtmlElement").style()
        .set_property("background-color", "transparent")
        .expect("should have style help");
    });
    let out_down = Closure::<dyn FnMut()>::new(move || {
        down_element_temp_4
        .dyn_ref::<HtmlElement>()
        .expect("should be HtmlElement").style()
        .set_property("background-color", "transparent")
        .expect("should have style help");
    });
    let touch_start_down = Closure::<dyn FnMut()>::new(move || {
        down_element_temp_5
            .dyn_ref::<HtmlElement>()
            .expect("should be HtmlElement").style()
        .set_property("background-color", "#FD7014")
        .expect("should have style help");
    });

    let touch_end_down = Closure::<dyn FnMut()>::new(move || {
        down_element_temp_6
            .dyn_ref::<HtmlElement>()
            .expect("should be HtmlElement").style()
        .set_property("background-color", "transparent")
        .expect("should have style help");
    });
    
    let press_left = Closure::<dyn FnMut()>::new(move || {
        left_element_temp
        .dyn_ref::<HtmlElement>()
        .expect("should be HtmlElement").style()
        .set_property("background-color", "#FD7014")
        .expect("should have style help");
    });
    let release_left = Closure::<dyn FnMut()>::new(move || {
        left_element_temp_3
        .dyn_ref::<HtmlElement>()
        .expect("should be HtmlElement").style()
        .set_property("background-color", "transparent")
        .expect("should have style help");
    });
    let out_left = Closure::<dyn FnMut()>::new(move || {
        left_element_temp_4
        .dyn_ref::<HtmlElement>()
        .expect("should be HtmlElement").style()
        .set_property("background-color", "transparent")
        .expect("should have style help");
    });
    let touch_start_left = Closure::<dyn FnMut()>::new(move || {
        left_element_temp_5
            .dyn_ref::<HtmlElement>()
            .expect("should be HtmlElement").style()
        .set_property("background-color", "#FD7014")
        .expect("should have style help");
    });

    let touch_end_left = Closure::<dyn FnMut()>::new(move || {
        left_element_temp_6
            .dyn_ref::<HtmlElement>()
            .expect("should be HtmlElement").style()
        .set_property("background-color", "transparent")
        .expect("should have style help");
    });

    up_element
        .set_onmousedown(Some(press_up.as_ref().unchecked_ref()));
    up_element
        .set_onmouseup(Some(release_up.as_ref().unchecked_ref()));
    up_element
        .set_onmouseleave(Some(out_up.as_ref().unchecked_ref()));
    up_element.set_ontouchstart(Some(touch_start_up.as_ref().unchecked_ref()));
    up_element.set_ontouchend(Some(touch_end_up.as_ref().unchecked_ref()));

    right_element
        .set_onmousedown(Some(press_right.as_ref().unchecked_ref()));

    right_element
        .set_onmouseup(Some(release_right.as_ref().unchecked_ref()));

    right_element
        .set_onmouseleave(Some(out_right.as_ref().unchecked_ref()));
    right_element.set_ontouchstart(Some(touch_start_right.as_ref().unchecked_ref()));
    right_element.set_ontouchend(Some(touch_end_right.as_ref().unchecked_ref()));

    down_element
        .set_onmousedown(Some(press_down.as_ref().unchecked_ref()));

    down_element
        .set_onmouseup(Some(release_down.as_ref().unchecked_ref()));

    down_element
        .set_onmouseleave(Some(out_down.as_ref().unchecked_ref()));
    down_element.set_ontouchstart(Some(touch_start_down.as_ref().unchecked_ref()));
    down_element.set_ontouchend(Some(touch_end_down.as_ref().unchecked_ref()));

    left_element
        .set_onmousedown(Some(press_left.as_ref().unchecked_ref()));

    left_element
        .set_onmouseup(Some(release_left.as_ref().unchecked_ref()));

    left_element
        .set_onmouseleave(Some(out_left.as_ref().unchecked_ref()));
    left_element.set_ontouchstart(Some(touch_start_left.as_ref().unchecked_ref()));
    left_element.set_ontouchend(Some(touch_end_left.as_ref().unchecked_ref()));

    press_up.forget();
    release_up.forget();
    out_up.forget();
    touch_start_up.forget();
    touch_end_up.forget();

    press_right.forget();
    release_right.forget();
    out_right.forget();
    touch_start_right.forget();
    touch_end_right.forget();
        
    press_down.forget();
    release_down.forget();
    out_down.forget();
    touch_start_down.forget();
    touch_end_down.forget();
    
    press_left.forget();
    release_left.forget();
    out_left.forget();
    touch_start_left.forget();
    touch_end_left.forget();
}

#[wasm_bindgen]
pub fn check_up() -> bool
{
    let window = web_sys::window().expect("no global `window` exists");
    let document = window.document().expect("should have a document window");

    let up_element_temp = document
        .get_element_by_id("up")
        .expect("should have up element");

    let up_element = up_element_temp
        .dyn_ref::<HtmlElement>()
        .expect("should be HtmlElement");
        
    let up_movement = up_element.style().get_property_value("background-color").expect("should have background color property");
    if up_movement != "transparent" {
        return true;
    } 

    return false;
}

#[wasm_bindgen]
pub fn check_right() -> bool
{
    let window = web_sys::window().expect("no global `window` exists");
    let document = window.document().expect("should have a document window");

    let right_element_temp = document
        .get_element_by_id("right")
        .expect("should have right element");
    
    let right_element = right_element_temp
        .dyn_ref::<HtmlElement>()
        .expect("should be HtmlElement");

    let right_movement = right_element.style().get_property_value("background-color").expect("should have background color property");

    if right_movement != "transparent" {
        return true;
    } 

    return false;
}

#[wasm_bindgen]
pub fn check_down() -> bool
{
    let window = web_sys::window().expect("no global `window` exists");
    let document = window.document().expect("should have a document window");

    let down_element_temp = document
        .get_element_by_id("down")
        .expect("should have down element");
    let down_element = down_element_temp
        .dyn_ref::<HtmlElement>()
        .expect("should be HtmlElement");
        
    let down_movement = down_element.style().get_property_value("background-color").expect("should have background color property");

    if down_movement !="transparent" {
        return true;
    } 

    return false;
}

#[wasm_bindgen]
pub fn check_left() -> bool
{
    let window = web_sys::window().expect("no global `window` exists");
    let document = window.document().expect("should have a document window");

    let left_element_temp = document
        .get_element_by_id("left")
        .expect("should have left element");
    let left_element = left_element_temp
        .dyn_ref::<HtmlElement>()
        .expect("should be HtmlElement");
        
    let left_movement = left_element.style().get_property_value("background-color").expect("should have background color property");

    if left_movement != "transparent" {
        return true;
    } 

    return false;
}