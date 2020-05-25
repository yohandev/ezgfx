use futures::executor::*;
use winit::event_loop::*;
use winit::window::*;
use winit::dpi::*;
use wgpu::*;

pub struct RenderContext
{
    pub surface: Surface,
    pub adapter: Adapter,
    pub device: Device,
    pub queue: Queue,
    pub sc_desc: SwapChainDescriptor,
    pub sc: SwapChain,

    pub size: PhysicalSize<u32>,
}

pub enum RenderAction
{
    
}

impl RenderContext
{
    /// creates a window and queue
    pub fn create(evt_loop: &EventLoop<()>) -> (Window, Self)
    {
        let window = WindowBuilder::new()
            .build(evt_loop)
            .expect("window could not be created!");

        let queue = block_on(Self::new(&window));

        (window, queue)
    }

    /// creates a queue from a window
    pub async fn new(window: &Window) -> Self
    {
        let size = window.inner_size();     // size
        let surface = Surface::create       // adapter
        (
            window
        );

        let aopt = RequestAdapterOptions    // adapter options
        {
            power_preference: PowerPreference::Default,
            compatible_surface: Some(&surface)
        };
        let adapter = Adapter::request      // adapter
        (
            &aopt,
            BackendBit::PRIMARY
        ).await.unwrap();

        let dq = adapter.request_device     // device, queue
        (
            &DeviceDescriptor
            {
                extensions: Extensions { anisotropic_filtering: false },
                limits: Limits::default()
            }
        ).await;

        let device = dq.0;                  // device
        let queue = dq.1;                   // queue

        let sc_desc = SwapChainDescriptor   // swap chain description
        {
            usage: TextureUsage::OUTPUT_ATTACHMENT,
            format: TextureFormat::Bgra8UnormSrgb,
            width: size.width,
            height: size.height,
            present_mode: PresentMode::Fifo
        };
        let sc = device.create_swap_chain   // swap chain
        (
            &surface,
            &sc_desc
        );

        Self                                // return
        {
            surface,
            adapter,
            device,
            queue,
            sc_desc,
            sc,
            size
        }
    }
}