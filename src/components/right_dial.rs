// Right dial - Harmonic dial with planets and midpoints
use leptos::*;
use wasm_bindgen::JsCast;
use web_sys::HtmlCanvasElement;
use std::f64::consts::PI;
use std::rc::Rc;
use std::cell::RefCell;

use crate::components::dial::Dial;
use crate::components::dial::renderer;
use crate::components::dial::utils;
use crate::models::harmonic::HarmonicType;
use crate::models::planet::{self, Planet};
use crate::utils::math::degrees_to_coords;
use crate::utils::constants::*;

/// RightDial component - the harmonic dial with planets
#[component]
pub fn RightDial(
    current_harmonic: ReadSignal<HarmonicType>,
    shared_rotation: ReadSignal<f64>,
    set_shared_rotation: WriteSignal<f64>,
) -> impl IntoView {
    let planets = planet::get_sibley_chart();
    let midpoints = planet::get_planet_pairs_for_midpoints(&planets);

    // Setup canvas rendering on component mount
    let canvas_id = "right-dial-canvas";
    let dial = Rc::new(RefCell::new(Dial::new(canvas_id)));

    {
        let dial = dial.clone();
        create_effect(move |_| {
            let harmonic = current_harmonic.get();
            let rot = shared_rotation.get();

            if let Some(context) = dial.borrow().get_context() {
                renderer::clear_canvas(&context);
                renderer::draw_circle(&context);

                let num_arms = harmonic.value() * 4;
                context.set_stroke_style(&wasm_bindgen::JsValue::from_str(&SECONDARY_STROKE_COLOR));
                context.set_line_width(1.0);

                for i in 0..num_arms {
                    let angle = (i as f64 * (360.0 / num_arms as f64) + rot) % 360.0;
                    let (x1, y1) = degrees_to_coords(angle, 0.0);
                    let (x2, y2) = degrees_to_coords(angle, RADIUS);

                    context.begin_path();
                    context.move_to(x1, y1);
                    context.line_to(x2, y2);
                    context.stroke();

                    if i % 4 == 0 {
                        let arrow_size = 5.0;
                        let arrow_angle = (angle + 180.0) % 360.0;
                        let (x1, y1) = degrees_to_coords(arrow_angle, RADIUS);
                        let (x2, y2) = degrees_to_coords(arrow_angle + 30.0, RADIUS - arrow_size);
                        let (x3, y3) = degrees_to_coords(arrow_angle - 30.0, RADIUS - arrow_size);

                        context.begin_path();
                        context.move_to(x1, y1);
                        context.line_to(x2, y2);
                        context.line_to(x3, y3);
                        context.close_path();
                        context.set_fill_style(&wasm_bindgen::JsValue::from_str(&BASE_STROKE_COLOR));
                        context.fill();
                    }
                }

                context.set_fill_style(&wasm_bindgen::JsValue::from_str(&BASE_STROKE_COLOR));
                context.set_font(DEGREE_LABEL_FONT_SIZE);
                context.set_text_align("center");
                context.set_text_baseline("middle");

                let harmonic_range = harmonic.harmonic_range();
                let label_step = utils::calculate_label_step(harmonic_range);

                let mut h_deg = 0.0;
                while h_deg < harmonic_range {
                    let tick_angle = (rot + h_deg) % 360.0;
                    context.set_stroke_style(&wasm_bindgen::JsValue::from_str(&SECONDARY_STROKE_COLOR));
                    context.set_line_width(1.0);

                    let (tick_x1, tick_y1) = degrees_to_coords(tick_angle, RADIUS - 2.0);
                    let (tick_x2, tick_y2) = degrees_to_coords(tick_angle, RADIUS - 15.0);

                    context.begin_path();
                    context.move_to(tick_x1, tick_y1);
                    context.line_to(tick_x2, tick_y2);
                    context.stroke();

                    let (lx, ly) = degrees_to_coords(tick_angle, RADIUS - 25.0);
                    context.fill_text(&format!("{}", h_deg as i32), lx, ly).unwrap();
                    h_deg += label_step;
                }

                let arm_angles = (0..num_arms)
                    .map(|i| (i as f64 * (360.0 / num_arms as f64) + rot) % 360.0)
                    .collect::<Vec<_>>();

                let active_midpoints = midpoints.iter()
                    .filter(|(midpoint, _, _)| {
                        arm_angles.iter().any(|&axis| {
                            let midpoint_harm = midpoint % harmonic_range;
                            let axis_harm = axis % harmonic_range;
                            let diff = (midpoint_harm - axis_harm).abs() % harmonic_range;
                            let normalized_diff = if diff > harmonic_range / 2.0 {
                                harmonic_range - diff
                            } else {
                                diff
                            };
                            normalized_diff <= 3.0
                        })
                    })
                    .cloned()
                    .collect::<Vec<_>>();

                for (_, p1_idx, p2_idx) in &active_midpoints {
                    if let (Some(p1), Some(p2)) = (planets.get(*p1_idx), planets.get(*p2_idx)) {
                        let p1_harm_pos_eff = p1.longitude % harmonic_range;
                        let p2_harm_pos_eff = p2.longitude % harmonic_range;
                        let (x1, y1) = degrees_to_coords(p1_harm_pos_eff, RADIUS + 25.0);
                        let (x2, y2) = degrees_to_coords(p2_harm_pos_eff, RADIUS + 25.0);
                        context.begin_path();
                        context.move_to(x1, y1);
                        context.line_to(x2, y2);
                        context.stroke();
                    }
                }

                context.set_font(PLANET_SYMBOL_FONT_SIZE);
                for planet in &planets {
                    let harmonic_pos_eff = planet.longitude % harmonic_range;
                    let (px, py) = degrees_to_coords(harmonic_pos_eff, RADIUS + 25.0);

                    context.set_stroke_style(&wasm_bindgen::JsValue::from_str(&PLANET_TICK_COLOR));
                    context.set_line_width(1.0);
                    let (tx1, ty1) = degrees_to_coords(harmonic_pos_eff, RADIUS);
                    let (tx2, ty2) = degrees_to_coords(harmonic_pos_eff, RADIUS + 20.0);

                    context.begin_path();
                    context.move_to(tx1, ty1);
                    context.line_to(tx2, ty2);
                    context.stroke();

                    context.set_fill_style(&wasm_bindgen::JsValue::from_str(&planet.color));
                    let _ = context.fill_text(&planet.symbol, px - 5.0, py + 5.0);
                }
            }
        });
    }

    let dragging = create_rw_signal(false);
    let last_x = create_rw_signal(0);
    let last_y = create_rw_signal(0);

    let on_mouse_down = move |e: web_sys::MouseEvent| {
        dragging.set(true);
        last_x.set(e.client_x());
        last_y.set(e.client_y());
    };

    let on_mouse_move = {
        let dial = dial.clone();
        move |e: web_sys::MouseEvent| {
            if dragging.get() {
                if let Some(canvas) = dial.borrow().get_canvas_element() {
                    let rect = canvas.get_bounding_client_rect();
                    let center_x = rect.left() + rect.width() / 2.0;
                    let center_y = rect.top() + rect.height() / 2.0;

                    let curr_x = e.client_x() as f64;
                    let curr_y = e.client_y() as f64;
                    let prev_x = last_x.get() as f64;
                    let prev_y = last_y.get() as f64;

                    let prev_angle = (prev_y - center_y).atan2(prev_x - center_x) * 180.0 / PI;
                    let curr_angle = (curr_y - center_y).atan2(curr_x - center_x) * 180.0 / PI;
                    let delta = curr_angle - prev_angle;

                    let new_rotation = (shared_rotation.get() - delta) % 360.0;
                    set_shared_rotation.set(new_rotation);

                    last_x.set(e.client_x());
                    last_y.set(e.client_y());
                }
            }
        }
    };

    let on_mouse_up = move |_| {
        dragging.set(false);
    };

    view! {
        <div>
            <canvas
                id={canvas_id}
                width="400"
                height="400"
                style="cursor: move; touch-action: none;"
                on:mousedown=on_mouse_down
                on:mousemove=on_mouse_move
                on:mouseup=on_mouse_up
                on:mouseleave=on_mouse_up
            ></canvas>
        </div>
    }
}
