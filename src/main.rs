// started by copying over conway.rs from pixels repo

#![deny(clippy::all)]
#![forbid(unsafe_code)]

use cell::{Cell, HasColor, Randomize};
use config::{read_config, BoundaryConfig, Config, SystemConfig};
use grid::boundary::{Boundary, ConstantBoundary, FreeBoundary};
use grid::grid_view::GridView;
use log::{debug, error};
use pixels::{Error, Pixels, SurfaceTexture};
use winit::event::{Event, KeyboardInput, VirtualKeyCode, WindowEvent};
use winit::event_loop::{ControlFlow, EventLoop};
use winit_input_helper::WinitInputHelper;

use crate::cell::System;
use crate::cell_library::*;
use crate::generic_system::GenericSystem;
use crate::grid::boundary::PeriodicBoundary;
use crate::grid::Grid;

use crate::window::create_window;

mod cell;
mod cell_library;
mod config;
mod generic_system;
mod grid;
mod input;
//mod phased_particle_system;
mod util;
mod window;

fn main() -> Result<(), Error> {
    let config = read_config();
    match config.system {
        SystemConfig::Conway => run_with_cell::<ConwayCell>(config),
        SystemConfig::XYModel => run_with_cell::<XYModelCell>(config),
        SystemConfig::Wave => run_with_cell::<WaveCell>(config),
        SystemConfig::Sandpile => run_with_cell::<AbelianSandpileCell>(config),
        SystemConfig::Schrodinger => run_with_cell::<SchrodingerCell>(config),
    }
}

fn run_with_cell<T: Clone + Default + HasColor + Randomize + Cell + 'static>(
    config: Config,
) -> Result<(), Error>
where
    GenericSystem<T>: SystemInputs,
{
    env_logger::init();
    let event_loop = EventLoop::new();
    let mut input = WinitInputHelper::new();
    let (window, p_width, p_height, mut _hidpi_factor) = create_window(
        "Cellular",
        &event_loop,
        config.grid_width,
        config.grid_height,
    );

    let surface_texture = SurfaceTexture::new(p_width, p_height, &window);

    let mut system = GenericSystem::<T>::new(Grid::new_random(
        config.grid_width as usize,
        config.grid_height as usize,
        match config.boundary {
            BoundaryConfig::Periodic => PeriodicBoundary.into(),
            BoundaryConfig::Constant => ConstantBoundary::empty().into(),
            BoundaryConfig::Free => FreeBoundary.into(),
        },
    ));
    let mut pixels = Pixels::new(config.grid_width, config.grid_height, surface_texture)?;
    let mut paused = false;

    let mut draw_state: Option<bool> = None;

    event_loop.run(move |event, _, control_flow| {
        // The one and only event that winit_input_helper doesn't have for us...
        if let Event::RedrawRequested(_) = event {
            system.grid.draw(pixels.get_frame());
            if pixels
                .render()
                .map_err(|e| error!("pixels.render() failed: {}", e))
                .is_err()
            {
                *control_flow = ControlFlow::Exit;
                return;
            }
        }
        if let Event::WindowEvent { event, .. } = &event {
            if let WindowEvent::KeyboardInput { input, .. } = event {
                input.virtual_keycode.map(|k| system.key_response(k));
            }
        }

        // For everything else, for let winit_input_helper collect events to build its state.
        // It returns `true` when it is time to update our game state and request a redraw.
        if input.update(&event) {
            let input_result = input::handle_input(&input);
            // Close events
            if input_result.request_exit {
                *control_flow = ControlFlow::Exit;
                return;
            }

            if input_result.pause {
                paused = !paused;
            }

            if input_result.randomize {
                system.grid.randomize();
            }

            if input_result.clear {
                system.grid.clear();
            }
            // Handle mouse. This is a bit involved since support some simple
            // line drawing (mostly because it makes nice looking patterns).
            let (mouse_cell, mouse_prev_cell) = input
                .mouse()
                .map(|(mx, my)| {
                    let (dx, dy) = input.mouse_diff();
                    let prev_x = mx - dx;
                    let prev_y = my - dy;

                    let (mx_i, my_i) = pixels
                        .window_pos_to_pixel((mx, my))
                        .unwrap_or_else(|pos| pixels.clamp_pixel_pos(pos));

                    let (px_i, py_i) = pixels
                        .window_pos_to_pixel((prev_x, prev_y))
                        .unwrap_or_else(|pos| pixels.clamp_pixel_pos(pos));

                    (
                        (mx_i as isize, my_i as isize),
                        (px_i as isize, py_i as isize),
                    )
                })
                .unwrap_or_default();

            if input.mouse_pressed(0) {
                debug!("Mouse click at {:?}", mouse_cell);
                draw_state = Some(system.toggle(mouse_cell.0, mouse_cell.1));
            } else if let Some(draw_alive) = draw_state {
                let release = input.mouse_released(0);
                let held = input.mouse_held(0);
                debug!("Draw at {:?} => {:?}", mouse_prev_cell, mouse_cell);
                debug!("Mouse held {:?}, release {:?}", held, release);
                // If they either released (finishing the drawing) or are still
                // in the middle of drawing, keep going.
                if release || held {
                    debug!("Draw line of {:?}", draw_alive);
                    // system.set_line(
                    //     mouse_prev_cell.0,
                    //     mouse_prev_cell.1,
                    //     mouse_cell.0,
                    //     mouse_cell.1,
                    //     draw_alive,
                    // );
                }
                // If they let go or are otherwise not clicking anymore, stop drawing.
                if release || !held {
                    debug!("Draw end");
                    draw_state = None;
                }
            }
            // Adjust high DPI factor
            if let Some(factor) = input.scale_factor_changed() {
                _hidpi_factor = factor;
            }
            // Resize the window
            if let Some(size) = input.window_resized() {
                pixels.resize_surface(size.width, size.height);
            }
            if !paused || input.key_pressed(VirtualKeyCode::Space) {
                for _ in 0..config.per_frame_updates {
                    system.update();
                }
            }
            window.request_redraw();
        }
    });
}

pub trait SystemInputs {
    fn key_response(&self, vkc: VirtualKeyCode) {}
}
impl SystemInputs for GenericSystem<XYModelCell> {
    fn key_response(&self, vkc: VirtualKeyCode) {
        match vkc {
            VirtualKeyCode::E => {
                let total_energy: f32 = self
                    .grid
                    .get_grid_pos_iter()
                    .map(|p| {
                        self.grid
                            .get_cell_at(p)
                            .get_energy(&GridView::new(p, &self.grid))
                    })
                    .sum();
                println!("Total energy {:?}", total_energy)
            }
            _ => (),
        }
    }
}

impl SystemInputs for GenericSystem<ConwayCell> {}
impl SystemInputs for GenericSystem<WaveCell> {}
impl SystemInputs for GenericSystem<AbelianSandpileCell> {}
impl SystemInputs for GenericSystem<SchrodingerCell> {}
