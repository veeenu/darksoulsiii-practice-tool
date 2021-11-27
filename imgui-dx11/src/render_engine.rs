use crate::buffers::Buffers;
use crate::device_and_swapchain::*;
use crate::shader_program::ShaderProgram;
use crate::texture::Texture;

use imgui::internal::RawWrapper;
use imgui::DrawCmd;
use imgui::DrawVert;
use winapi::shared::dxgi::IDXGISwapChain;
use winapi::shared::dxgiformat::*;
use winapi::shared::windef::*;
use winapi::um::d3d11::*;
use winapi::um::d3dcommon::D3D11_PRIMITIVE_TOPOLOGY_TRIANGLELIST;

pub struct RenderEngine {
    ctx: imgui::Context,
    dasc: DeviceAndSwapChain,
    shader_program: ShaderProgram,
    buffers: Buffers,
    texture: Texture,
}

impl RenderEngine {
    pub fn new(hwnd: HWND) -> Self {
        let mut ctx = imgui::Context::create();
        let dasc = DeviceAndSwapChain::new(hwnd);
        let shader_program = ShaderProgram::new(&dasc);
        let buffers = Buffers::new(&dasc);
        let texture = Texture::new(&dasc, &mut ctx.fonts());
        RenderEngine {
            ctx,
            dasc,
            shader_program,
            buffers,
            texture,
        }
    }

    pub fn new_with_ptrs(
        dev: *mut ID3D11Device,
        dev_ctx: *mut ID3D11DeviceContext,
        swap_chain: *mut IDXGISwapChain,
    ) -> Self {
        let mut ctx = imgui::Context::create();
        let dasc = DeviceAndSwapChain::new_with_ptrs(dev, dev_ctx, swap_chain);
        let shader_program = ShaderProgram::new(&dasc);
        let buffers = Buffers::new(&dasc);
        let texture = Texture::new(&dasc, &mut ctx.fonts());
        RenderEngine {
            ctx,
            dasc,
            shader_program,
            buffers,
            texture,
        }
    }

    pub fn ctx(&mut self) -> &mut imgui::Context {
        &mut self.ctx
    }

    pub fn dev(&self) -> &ID3D11Device {
        self.dasc.dev()
    }

    pub fn dev_ctx(&self) -> &ID3D11DeviceContext {
        self.dasc.dev_ctx()
    }

    pub fn swap_chain(&self) -> &IDXGISwapChain {
        self.dasc.swap_chain()
    }

    pub fn render<F: FnOnce(&mut imgui::Ui)>(&mut self, f: F) -> Result<(), String> {
        if let Some(mut rect) = self.dasc.get_window_rect() {
            self.ctx.io_mut().display_size = [
                (rect.right - rect.left) as f32,
                (rect.bottom - rect.top) as f32,
            ];
            rect.right = rect.right - rect.left;
            rect.bottom = rect.bottom - rect.top;
            rect.top = 0;
            rect.left = 0;
            self.dasc.set_viewport(rect);
        }

        self.dasc.set_render_target();

        let mut ui = self.ctx.frame();
        f(&mut ui);
        let draw_data = ui.render();

        let [x, y] = draw_data.display_pos;
        let [width, height] = draw_data.display_size;

        if width <= 0. && height <= 0. {
            return Err(format!("Insufficient display size {} x {}", width, height).into());
        }

        unsafe {
            let dev_ctx = self.dasc.dev_ctx();

            self.shader_program.set_state(&self.dasc);

            self.buffers
                .set_constant_buffer(&self.dasc, [x, y, x + width, y + height]);
            self.buffers.set_buffers(&self.dasc, draw_data.draw_lists());

            dev_ctx.IASetVertexBuffers(
                0,
                1,
                &self.buffers.vtx_buffer(),
                &(std::mem::size_of::<DrawVert>() as u32),
                &0,
            );
            dev_ctx.IASetIndexBuffer(
                self.buffers.idx_buffer(),
                if std::mem::size_of::<imgui::DrawIdx>() == 2 {
                    DXGI_FORMAT_R16_UINT
                } else {
                    DXGI_FORMAT_R32_UINT
                },
                0,
            );
            dev_ctx.IASetPrimitiveTopology(D3D11_PRIMITIVE_TOPOLOGY_TRIANGLELIST);
            dev_ctx.VSSetConstantBuffers(0, 1, &self.buffers.mtx_buffer());
            dev_ctx.PSSetShaderResources(0, 1, &self.texture.tex_view());

            for cl in draw_data.draw_lists() {
                for cmd in cl.commands() {
                    match cmd {
                        DrawCmd::Elements { count, cmd_params } => {
                            let [cx, cy, cw, ch] = cmd_params.clip_rect;
                            dev_ctx.RSSetScissorRects(
                                1,
                                &D3D11_RECT {
                                    left: (cx - x) as i32,
                                    top: (cy - y) as i32,
                                    right: (cw - x) as i32,
                                    bottom: (ch - y) as i32,
                                },
                            );

                            dev_ctx.DrawIndexed(
                                count as u32,
                                cmd_params.idx_offset as _,
                                cmd_params.vtx_offset as _,
                            );
                        }
                        DrawCmd::ResetRenderState => self.dasc.setup_state(&draw_data),
                        DrawCmd::RawCallback { callback, raw_cmd } => callback(cl.raw(), raw_cmd),
                    }
                }
            }

            // self.dasc.swap_chain().Present(1, 0);
        }

        Ok(())
    }

    pub fn present(&self) {
        unsafe { self.dasc.swap_chain().Present(1, 0) };
    }
}
