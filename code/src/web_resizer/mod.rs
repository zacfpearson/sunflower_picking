use wasm_bindgen::prelude::*;
use wasm_bindgen::JsCast;
use web_sys::HtmlElement;
use bevy::prelude::{EventWriter, Query, ResMut, Res, Transform, Vec3, WindowDescriptor, Windows, With,};
use bevy::window::{WindowResized};

use crate::resizable::Resizable;

#[wasm_bindgen]
pub fn get_resize_width() -> f32
{
    let window = web_sys::window().expect("no global `window` exists");
    let document = window.document().expect("should have a document window");
    let binding = document.get_element_by_id("bevy-container").expect("should have app element");
    let canvas = binding
        .dyn_ref::<HtmlElement>()
        .expect("should be an HtmlElement");

    
    let width: f32 = canvas.offset_width() as f32;

    return width;
}

#[wasm_bindgen]
extern "C" {
    fn resize_canvas(width: f32, height: f32);
}

#[cfg(target_arch = "wasm32")]
pub fn resizer(
    mut windows: ResMut<Windows>,
    mut window_resized_events: EventWriter<WindowResized>,
    wd: Res<WindowDescriptor>,
    mut transforms: Query<&mut Transform, With<Resizable>>,
) {
    let mut width = get_resize_width();
    let mut height = width * 0.5;
    height = height.floor();

    if let Some(window) = windows.get_primary_mut() {
        if window.width() != width || window.height() != height {
            width = if width < wd.resize_constraints.min_width {
                wd.resize_constraints.min_width
            } else {
                width
            };
            width = if width > wd.resize_constraints.max_width {
                wd.resize_constraints.max_width
            } else {
                width
            };

            //code for note keeping aspect ratio.
            height = if height < wd.resize_constraints.min_height {
                wd.resize_constraints.min_height
            } else {
                height
            };
            height = if height > wd.resize_constraints.max_height {
                wd.resize_constraints.max_height
            } else {
                height
            };

            for mut transform in transforms.iter_mut() {
                transform.scale *= Vec3::new(width / window.width(), height / window.height(), 1.0);
                transform.translation *= Vec3::new(width / window.width(), height / window.height(), 1.0);
            }

            window.update_actual_size_from_backend(width as u32, height as u32);
            window_resized_events.send(WindowResized {
                id: window.id(),
                height: height,
                width: width,
            });
            resize_canvas(width, height);
        }
    }
}

#[cfg(target_arch = "wasm32")]
pub fn web_size(width: &mut f32, height: &mut f32)
{
    let window = web_sys::window().expect("no global `window` exists");
    *width = window.inner_width().unwrap().as_f64().unwrap() as f32;
    *height = window.inner_height().unwrap().as_f64().unwrap() as f32;
}