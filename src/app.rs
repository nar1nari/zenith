use leptos::wasm_bindgen::JsCast;
use leptos::{html, prelude::*};
use leptos_use::{
    use_mouse_with_options, use_window_size, UseMouseCoordType, UseMouseEventExtractor,
    UseMouseOptions,
};
use web_sys::MouseEvent;

use crate::render::render_sky;

#[derive(Clone)]
struct Extractor;

impl UseMouseEventExtractor for Extractor {
    fn extract_mouse_coords(&self, event: &MouseEvent) -> Option<(f64, f64)> {
        Some((event.offset_x() as f64, event.offset_y() as f64))
    }
}

fn find_city_by_name(name: &str) -> Option<&'static cities::City> {
    cities::all()
        .iter()
        .find(|city| city.city.eq_ignore_ascii_case(name))
}

#[component]
pub fn CitySelector(city: RwSignal<String>) -> impl IntoView {
    view! {
        <div class="controls">
            <label for="city_input">
                "Select city"
            </label>
            <input id="city_input" list="cities" bind:value=city/>

            <datalist id="cities">
                {cities::all().into_iter().map(|val| view! { <option value={val.city}/> }).collect::<Vec<_>>()}
            </datalist>
        </div>
    }
}

#[component]
pub fn SkyCanvas(observer_lat: Memo<f64>, observer_lon: Memo<f64>) -> impl IntoView {
    let canvas_ref: NodeRef<html::Canvas> = NodeRef::new();
    let mouse = use_mouse_with_options(
        UseMouseOptions::default()
            .target(canvas_ref)
            .coord_type(UseMouseCoordType::Custom(Extractor)),
    );
    let window_size = use_window_size();

    Effect::new(move |_| {
        let Some(canvas) = canvas_ref.get() else {
            return;
        };
        let (lat, lon) = (observer_lat.get(), observer_lon.get());
        let (x, y) = (mouse.x.get(), mouse.y.get());
        let _ = window_size.width.get();
        let _ = window_size.height.get();

        let dpr = web_sys::window().unwrap().device_pixel_ratio();
        let (width, height) = (canvas.client_width() as f64, canvas.client_height() as f64);

        canvas.set_width((width * dpr) as u32);
        canvas.set_height((height * dpr) as u32);

        let ctx = canvas
            .get_context("2d")
            .unwrap()
            .unwrap()
            .dyn_into::<web_sys::CanvasRenderingContext2d>()
            .unwrap();

        ctx.scale(dpr, dpr).unwrap();
        ctx.clear_rect(0.0, 0.0, width, height);

        render_sky(&ctx, lat, lon, width, height, x, y);
    });

    view! {
        <canvas node_ref=canvas_ref class="sky-canvas"/>
    }
}

#[component]
pub fn App() -> impl IntoView {
    let city = RwSignal::new(String::from("New York City"));
    let observer_lat =
        Memo::new(move |_| find_city_by_name(&city.get()).map_or(0.0, |c| c.latitude));
    let observer_lon =
        Memo::new(move |_| find_city_by_name(&city.get()).map_or(0.0, |c| c.longitude));

    view! {
        <main class="layout">
            <section class="sidebar">
                <CitySelector city/>
                <p>
                    "Latitude: " {observer_lat} <br/>
                    "Longtitude: " {observer_lon}
                </p>
            </section>

            <section class="canvas-container">
                <SkyCanvas observer_lat observer_lon/>
            </section>
        </main>
    }
}
