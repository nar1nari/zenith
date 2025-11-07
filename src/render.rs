use crate::{
    astronomy::{calculate_alt_az, calculate_local_sidereal_time},
    bodies::{get_bodies, Body},
};
use chrono::Utc;
use siderust::bodies::Earth;
use std::f64;
use std::f64::consts::PI;
use web_sys::CanvasRenderingContext2d;

#[derive(Debug, Clone, Copy, PartialEq)]
struct Vec2 {
    x: f64,
    y: f64,
}

impl Vec2 {
    fn new(x: f64, y: f64) -> Self {
        Self { x, y }
    }

    fn distance(&self, other: Vec2) -> f64 {
        ((self.x - other.x).powi(2) + (self.y - other.y).powi(2)).sqrt()
    }
}

fn project_to_screen(azimuth: f64, altitude: f64, screen_center: Vec2, sky_radius: f64) -> Vec2 {
    let radius = (90.0 - altitude.to_degrees()) / 90.0 * sky_radius;
    Vec2::new(
        screen_center.x + (radius * azimuth.sin()),
        screen_center.y + (radius * azimuth.cos()),
    )
}

pub fn render_sky(
    ctx: &CanvasRenderingContext2d,
    observer_lat: f64,
    observer_lon: f64,
    canvas_width: f64,
    canvas_height: f64,
    mouse_x: f64,
    mouse_y: f64,
) {
    let screen_center = Vec2::new(canvas_width / 2.0, canvas_height / 2.0);
    let min_dim = canvas_width.min(canvas_height);
    let margin = min_dim * 0.05;
    let sky_radius = min_dim / 2.0 - margin;
    let planet_size = min_dim * 0.01;
    let mouse_pos = Vec2::new(mouse_x, mouse_y);

    ctx.begin_path();
    ctx.arc(screen_center.x, screen_center.y, sky_radius, 0.0, PI * 2.0)
        .unwrap();
    ctx.set_fill_style_str("#181C25");
    ctx.fill();

    let tri_height = sky_radius * 0.08;
    let tri_width = sky_radius * 0.08;
    let top_center = Vec2::new(screen_center.x, screen_center.y - sky_radius - tri_height);
    draw_north(ctx, top_center, tri_width, tri_height);

    let jd = siderust::astro::JulianDate::from_utc(Utc::now());
    let lst = calculate_local_sidereal_time(jd.value(), observer_lon);
    let earth_pos = Earth::vsop87e(jd);
    let bodies = get_bodies();

    for body in bodies.iter() {
        let (azimuth, altitude) = calculate_alt_az(jd, body, &earth_pos, observer_lat, lst);

        if altitude > 0.0 {
            let pos = project_to_screen(azimuth, altitude, screen_center, sky_radius);
            render_body(ctx, body, pos, planet_size, mouse_pos);
        }
    }
}

fn render_body(
    ctx: &CanvasRenderingContext2d,
    body: &Body,
    pos: Vec2,
    base_size: f64,
    mouse: Vec2,
) {
    let dist = pos.distance(mouse);
    let highlight = dist < base_size * 1.5;
    let radius = if highlight {
        base_size * 1.5
    } else {
        base_size
    };

    ctx.begin_path();
    ctx.arc(pos.x, pos.y, radius, 0.0, PI * 2.0).unwrap();
    ctx.set_fill_style_str(body.color);
    ctx.fill();

    if highlight {
        ctx.set_font(&format!("{}px monospace", base_size * 3.0));
        ctx.set_fill_style_str("#DFE3EB");
        ctx.set_text_align("left");
        let _ = ctx.fill_text(body.name, pos.x + radius, pos.y - radius);
    }
}

fn draw_north(ctx: &CanvasRenderingContext2d, top_center: Vec2, tri_width: f64, tri_height: f64) {
    ctx.begin_path();
    ctx.move_to(top_center.x, top_center.y);
    ctx.line_to(top_center.x - tri_width / 2.0, top_center.y + tri_height);
    ctx.line_to(top_center.x + tri_width / 2.0, top_center.y + tri_height);
    ctx.close_path();

    ctx.set_fill_style_str("#E67E80");
    ctx.fill();

    ctx.set_fill_style_str("#181C25");
    ctx.set_font(&format!("{}px monospace", tri_height * 0.75));
    ctx.set_text_align("center");
    let _ = ctx.fill_text("N", top_center.x, top_center.y + tri_height * 0.95);
}
