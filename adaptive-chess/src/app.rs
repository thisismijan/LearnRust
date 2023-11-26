use log;
use pixels::{Pixels, SurfaceTexture};
use winit::dpi::LogicalSize;

use winit::event::{Event, WindowEvent};
use winit::event_loop::{ControlFlow, EventLoop};
use winit::window::WindowBuilder;

use crate::model::board::Board;

const WIN_WIDTH: u32 = 854;
const WIN_HEIGHT: u32 = 854;

pub struct AppRunner;

impl AppRunner {
pub fn run() {
    let event_loop = EventLoop::new().unwrap();
    let builder = WindowBuilder::new();
    let window_size = LogicalSize::new(WIN_WIDTH, WIN_HEIGHT);
    let window = builder
        .with_title("Adaptive-Chess")
        .with_inner_size(window_size)
        .build(&event_loop)
        .unwrap();

    let mut board = Board::default();

    let mut pixels = {
        let window_size = window.inner_size();
        let surface_texture = SurfaceTexture::new(window_size.width, window_size.height, &window);
        let board_size = board.get_side_length();
        // TODO: use new_async for web
        Pixels::new(board_size, board_size, surface_texture).unwrap()
    };
    
    event_loop.set_control_flow(ControlFlow::Poll);
    event_loop.run(move |event, event_loop_window_target| {

        match event {
            Event::WindowEvent {
                event: WindowEvent::CloseRequested,
                ..
            } => {
                log::info!("The close Button was pressed.");
                event_loop_window_target.exit();
            }
            Event::WindowEvent {
                event: WindowEvent::Resized(size),
                ..
            } => {
                if let Err(err) = pixels.resize_surface(size.width, size.height) {
                    log::error!("Pixels failed to resize error: {}", err);
                    event_loop_window_target.exit();
                }
            }
            Event::AboutToWait => {
                window.request_redraw();
            }
            Event::WindowEvent {
                event: WindowEvent::RedrawRequested,
                ..
             } => {
                board.draw(pixels.frame_mut());
                if let Err(err) = pixels.render() {
                    log::error!("pixels.render() failed: {err}");
                    event_loop_window_target.exit();
                    return;
                }
            }
            _ => (),
        }
    }).unwrap()
}
}
