use winit::{
    event::*,
    event_loop::{ControlFlow, EventLoop},
    window::WindowBuilder,
};
use wgpu::Surface;

async fn run() {
    // Création de l'EventLoop et de la fenêtre
    let event_loop = EventLoop::new();
    let window = WindowBuilder::new().build(&event_loop).unwrap();

    // Instancier wgpu
    let instance = wgpu::Instance::default();
    let surface = unsafe { instance.create_surface(&window) }.unwrap();
    
    // Device, adapter et queue
    let adapter = instance
        .request_adapter(&wgpu::RequestAdapterOptions::default())
        .await
        .unwrap();
    let (device, queue) = adapter
        .request_device(&wgpu::DeviceDescriptor::default(), None)
        .await
        .unwrap();

    // Boucle d'événements
    event_loop.run(move |event, _, control_flow| {
        *control_flow = ControlFlow::Poll;
        match event {
            Event::WindowEvent {
                ref event,
                window_id,
            } if window_id == window.id() => {
                if let WindowEvent::CloseRequested = event {
                    *control_flow = ControlFlow::Exit;
                }
            }
            Event::RedrawRequested(_) => {
                // Ici on pourrait dessiner des trucs avec wgpu
            }
            _ => {}
        }
    });
}

fn main() {
    pollster::block_on(run());
}
