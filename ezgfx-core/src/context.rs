use futures::executor::*;
use winit::event_loop::*;
use winit::window::*;
use winit::dpi::*;
use wgpu::*;

pub struct RenderContext
{
    // -- context info --
    pub surface: Surface,
    pub adapter: Adapter,
    pub device: Device,
    pub queue: Queue,
    pub sc_desc: SwapChainDescriptor,
    pub sc: SwapChain,

    // -- window info --
    pub size: PhysicalSize<u32>,

    // -- frame info --
    frame: Option<SwapChainOutput>,
    encoder: Option<CommandEncoder>,
}

impl RenderContext
{
    /// creates a window and queue
    pub fn create(evt_loop: &EventLoop<()>) -> (Window, Self)
    {
        let window = WindowBuilder::new()
            .build(evt_loop)
            .expect("window could not be created!");

        let queue = block_on(Self::create_from_window(&window));

        (window, queue)
    }

    /// creates a queue from a window
    pub async fn create_from_window(window: &Window) -> Self
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
            size,
            
            frame: None,
            encoder: None,
        }
    }

    pub fn resize(&mut self, size: PhysicalSize<u32>)
    {
        self.size = size;                   // size
        self.sc_desc.width = size.width;    // width
        self.sc_desc.height = size.height;  // height
        self.sc =                           // swap chain
        self.device.create_swap_chain
        (
            &self.surface,
            &self.sc_desc
        );
    }

    pub fn begin_frame(&mut self)
    {
        assert!(self.frame.is_none() && self.encoder.is_none(), "called begin_frame before submitting the last frame!");

        self.frame.replace                  // frame texture
        (
            self.sc                 
                .get_next_texture()
                .expect("timeout getting texture")
        );
        
        self.encoder.replace                // command encoder
        (
            self.device
                .create_command_encoder
                (
                    &CommandEncoderDescriptor
                    {
                        label: Some("Render Encoder")
                    }
                )
        );
    }

    pub fn begin_render_pass(&mut self, clear: [f64; 4])
    {
        assert!(self.encoder.is_some() && self.frame.is_some(), "missing command encoder or frame. did you forget to call begin_frame?");

        let view = &self.frame.as_ref().unwrap().view;
        let encoder = self.encoder.as_mut().unwrap();

        encoder.begin_render_pass    
            (
                &RenderPassDescriptor
                {
                    color_attachments:
                    &[
                        RenderPassColorAttachmentDescriptor
                        {
                            attachment: view,
                            resolve_target: None,
                            load_op: LoadOp::Clear,
                            store_op: StoreOp::Store,
                            clear_color: Color { r: clear[0], g: clear[1], b: clear[2], a: clear[3] }
                        }
                    ],
                    depth_stencil_attachment: None,
                }
            );
    }

    
}