use crate::*;

/// represents a renderpass which records rendering functionalities
pub struct RenderPass<'a>
{
    //ctx: &'a mut RenderContext,
    pass: wgpu::RenderPass<'a>,
    encoder_id: usize
}

// impl<'a> RenderPass<'a>
// {
//     /// create a new un-init render pass
//     /// (new and begin are two distinct functions to overcome Rust's lifetime requirements.)
//     pub(crate) fn new(ctx: &'a mut RenderContext, clear: [f64; 4]) -> RenderPass<'a>
//     {  
//         use self::wgpu::*;

//         ctx.frame_encoders.push
//         (
//             Some
//             (
//                 ctx.device.create_command_encoder
//                 (
//                     &CommandEncoderDescriptor
//                     {
//                         label: Some("render_pass_encoder")
//                     }
//                 )
//             )
//         );

//         let view: &'a TextureView = &ctx.frame.as_ref().unwrap().view;
//         let encoder_id = ctx.frame_encoders.len() - 1;
//         let encoder = ctx.frame_encoders[encoder_id].as_mut().unwrap();
//         let pass = encoder.begin_render_pass
//         (
//             &RenderPassDescriptor
//             {
//                 color_attachments:
//                 &[
//                     RenderPassColorAttachmentDescriptor
//                     {
//                         attachment: view,
//                         resolve_target: None,
//                         load_op: LoadOp::Clear,
//                         store_op: StoreOp::Store,
//                         clear_color: Color { r: clear[0], g: clear[1], b: clear[2], a: clear[3] }
//                     }
//                 ],
//                 depth_stencil_attachment: None,
//             }
//         );

//         Self
//         {
//             //ctx: ctx,

//             encoder_id,
//             pass,
//         }
//     }

//     /// sets the render pipeline(comparable to a material) to be used
//     /// to render geometry.
//     pub fn set_render_pipeline(&mut self, pip: &'a crate::Pipeline) -> &mut Self
//     {  
//         // if let Some(pass) = &mut self.pass
//         // {
//             self.pass.set_pipeline(&pip.pipeline);
                
//             for (slot, binding) in &pip.bindings
//             {
//                 self.pass.set_bind_group(*slot, binding, &[]);
//             }
//         // }
//         // else
//         // {
//         //     panic!("render pass never begun! did you forget to call begin(..)?")
//         // }

//         self
//     }

//     /// draws geometry, uninstanced, with the last pipeline and uniforms
//     /// set in this pass.
//     pub fn draw_geometry(&mut self, geo: &'a crate::Geometry) -> &mut Self
//     {
//         // if let Some(pass) = &mut self.pass
//         // {
//             self.pass.set_index_buffer(&geo.i_buf, 0, 0);       // TODO remember buffers to avoid calls
//             self.pass.set_vertex_buffer(0, &geo.v_buf, 0, 0);

//             self.pass.draw_indexed(0..geo.i_count as u32, 0, 0..1);
//         // }
//         // else
//         // {
//         //     panic!("render pass never begun! did you forget to call begin(..)?")
//         // }

//         self
//     }

//     /// submits this render pass to the render context
//     pub fn submit(&self, ctx: &mut RenderContext)
//     {
//         ctx.queue.submit
//         (
//             &[
//                 ctx.frame_encoders[self.encoder_id]
//                     .take()
//                     .unwrap()
//                     .finish()
//             ]
//         );
//     }
// }