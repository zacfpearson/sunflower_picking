use wasm_bindgen::prelude::*;
use wasm_bindgen::JsCast;
use web_sys::HtmlElement;
use bevy::prelude::{EventWriter, Query, ResMut, Res, Transform, Vec3, WindowDescriptor, Windows, With,};
use bevy::window::{WindowResized};

use crate::resizable::Resizable;

#[wasm_bindgen]
extern "C" {
    fn resize_canvas(width: f32, height: f32);

    fn get_orientation() -> bool;

    fn set_orientation(state: bool);

    #[wasm_bindgen(js_namespace = console)]
    fn log(s: &str);
}

#[wasm_bindgen]
pub fn get_resize_width() -> f32
{
    let window = web_sys::window().expect("no global `window` exists");
    let document = window.document().expect("should have a document window");
    let binding = document.get_element_by_id("bevy-container").expect("should have app element");
    let canvas = binding
        .dyn_ref::<HtmlElement>()
        .expect("should be an HtmlElement");
    
    //TODO: The resize goes crazy if the width isn't a factor of 16. 
    //      This is because of how the grid is setup. If the calculated height of all the elments is greater
    //      than that of canvas, the sizes blow up. This is not a general way to do this and should be fixed!
    let mut even_width = canvas.offset_width();
    while even_width.rem_euclid(16) != 0
    {
        even_width = even_width - 1;
    }

    let width: f32 = even_width as f32;

    return width;
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

            //update_actual_size_from_backend doesn't work on mobile orientation change.
            //The new window size gets very small. So, using set_resolution in these instances. 
            if get_orientation()
            {
                window.set_resolution(width, height);
                set_orientation(false);
            }
            else
            {
                window.update_actual_size_from_backend(width as u32, height as u32);
            }
    
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
    *width = get_resize_width();
    *height = (*width * 0.5).floor();
    resize_canvas(*width, *height); //SEE TODO above, this should not be needed
}