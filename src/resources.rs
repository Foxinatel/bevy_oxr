use std::ffi::c_void;
use std::sync::atomic::AtomicBool;
use std::sync::Mutex;

use crate::input::XrInput;
use crate::passthrough::{CompositionLayerPassthrough, XrPassthroughLayer};
use crate::resource_macros::*;
use crate::xr::CompositionLayerFlags;
use bevy::prelude::*;
use bevy::render::extract_resource::{ExtractResource, ExtractResourcePlugin};
use openxr as xr;
#[cfg(all(feature = "d3d12", windows))]
use winapi::um::d3d12::{ID3D12CommandQueue, ID3D12Device};

xr_resource_wrapper!(XrInstance, xr::Instance);
xr_resource_wrapper_copy!(XrEnvironmentBlendMode, xr::EnvironmentBlendMode);
xr_resource_wrapper_copy!(XrResolution, UVec2);
xr_resource_wrapper_copy!(XrFormat, wgpu::TextureFormat);
xr_resource_wrapper_copy!(XrFrameState, xr::FrameState);
xr_resource_wrapper!(XrViews, Vec<xr::View>);
xr_arc_resource_wrapper!(XrSessionRunning, AtomicBool);
xr_arc_resource_wrapper!(XrSwapchain, Swapchain);
xr_no_clone_resource_wrapper!(XrFrameWaiter, xr::FrameWaiter);

#[derive(Clone, Resource, ExtractResource)]
pub enum XrSession {
    #[cfg(feature = "vulkan")]
    Vulkan(xr::Session<xr::Vulkan>),
    #[cfg(all(feature = "d3d12", windows))]
    D3D12(xr::Session<xr::D3D12>),
}

impl std::ops::Deref for XrSession {
    type Target = xr::Session<xr::AnyGraphics>;

    fn deref(&self) -> &Self::Target {
        // SAFTEY: should be fine i think -Schmarni
        unsafe {
            match self {
                #[cfg(feature = "vulkan")]
                XrSession::Vulkan(sess) => std::mem::transmute(sess),
                #[cfg(all(feature = "d3d12", windows))]
                XrSession::D3D12(sess) => std::mem::transmute(sess),
            }
        }
    }
}

#[cfg(feature = "vulkan")]
pub struct VulkanOXrSessionSetupInfo {
    pub(crate) device_ptr: *const c_void,
    pub(crate) physical_device_ptr: *const c_void,
    pub(crate) vk_instance_ptr: *const c_void,
    pub(crate) queue_family_index: u32,
    pub(crate) xr_system_id: xr::SystemId,
}

#[cfg(all(feature = "d3d12", windows))]
pub struct D3D12OXrSessionSetupInfo {
    pub(crate) raw_device: *mut ID3D12Device,
    pub(crate) raw_queue: *mut ID3D12CommandQueue,
    pub(crate) xr_system_id: xr::SystemId,
}

pub enum OXrSessionSetupInfo {
    #[cfg(feature = "vulkan")]
    Vulkan(VulkanOXrSessionSetupInfo),
    #[cfg(all(feature = "d3d12", windows))]
    D3D12(D3D12OXrSessionSetupInfo),
}

pub struct XrResourcePlugin;

impl Plugin for XrResourcePlugin {
    fn build(&self, app: &mut App) {
        app.add_plugins(ExtractResourcePlugin::<XrResolution>::default());
        app.add_plugins(ExtractResourcePlugin::<XrFormat>::default());
        app.add_plugins(ExtractResourcePlugin::<XrSwapchain>::default());
        app.add_plugins(ExtractResourcePlugin::<XrFrameState>::default());
        app.add_plugins(ExtractResourcePlugin::<XrViews>::default());
        app.add_plugins(ExtractResourcePlugin::<XrInput>::default());
        app.add_plugins(ExtractResourcePlugin::<XrEnvironmentBlendMode>::default());
        // app.add_plugins(ExtractResourcePlugin::<XrSessionRunning>::default());
        app.add_plugins(ExtractResourcePlugin::<XrSession>::default());
    }
}

pub enum Swapchain {
    #[cfg(feature = "vulkan")]
    Vulkan(SwapchainInner<xr::Vulkan>),
    #[cfg(all(feature = "d3d12", windows))]
    D3D12(SwapchainInner<xr::D3D12>),
}

impl Swapchain {
    pub(crate) fn begin(&self) -> xr::Result<()> {
        match self {
            #[cfg(feature = "vulkan")]
            Swapchain::Vulkan(swapchain) => swapchain.begin(),
            #[cfg(all(feature = "d3d12", windows))]
            Swapchain::D3D12(swapchain) => swapchain.begin(),
        }
    }

    pub(crate) fn get_render_views(&self) -> (wgpu::TextureView, wgpu::TextureView) {
        match self {
            #[cfg(feature = "vulkan")]
            Swapchain::Vulkan(swapchain) => swapchain.get_render_views(),
            #[cfg(all(feature = "d3d12", windows))]
            Swapchain::D3D12(swapchain) => swapchain.get_render_views(),
        }
    }

    pub(crate) fn acquire_image(&self) -> xr::Result<()> {
        match self {
            #[cfg(feature = "vulkan")]
            Swapchain::Vulkan(swapchain) => swapchain.acquire_image(),
            #[cfg(all(feature = "d3d12", windows))]
            Swapchain::D3D12(swapchain) => swapchain.acquire_image(),
        }
    }

    pub(crate) fn wait_image(&self) -> xr::Result<()> {
        match self {
            #[cfg(feature = "vulkan")]
            Swapchain::Vulkan(swapchain) => swapchain.wait_image(),
            #[cfg(all(feature = "d3d12", windows))]
            Swapchain::D3D12(swapchain) => swapchain.wait_image(),
        }
    }

    pub(crate) fn release_image(&self) -> xr::Result<()> {
        match self {
            #[cfg(feature = "vulkan")]
            Swapchain::Vulkan(swapchain) => swapchain.release_image(),
            #[cfg(all(feature = "d3d12", windows))]
            Swapchain::D3D12(swapchain) => swapchain.release_image(),
        }
    }

    pub(crate) fn end(
        &self,
        predicted_display_time: xr::Time,
        views: &[openxr::View],
        stage: &xr::Space,
        resolution: UVec2,
        environment_blend_mode: xr::EnvironmentBlendMode,
        passthrough_layer: Option<&XrPassthroughLayer>,
    ) -> xr::Result<()> {
        match self {
            #[cfg(feature = "vulkan")]
            Swapchain::Vulkan(swapchain) => swapchain.end(
                predicted_display_time,
                views,
                stage,
                resolution,
                environment_blend_mode,
                passthrough_layer,
            ),
            #[cfg(all(feature = "d3d12", windows))]
            Swapchain::D3D12(swapchain) => swapchain.end(
                predicted_display_time,
                views,
                stage,
                resolution,
                environment_blend_mode,
                passthrough_layer,
            ),
        }
    }
}

pub struct SwapchainInner<G: xr::Graphics> {
    pub(crate) stream: Mutex<xr::FrameStream<G>>,
    pub(crate) handle: Mutex<xr::Swapchain<G>>,
    pub(crate) buffers: Vec<wgpu::Texture>,
    pub(crate) image_index: Mutex<usize>,
}
impl<G: xr::Graphics> Drop for SwapchainInner<G> {
    fn drop(&mut self) {
        for _ in 0..self.buffers.len() {
            let v = self.buffers.remove(0);
            Box::leak(Box::new(v));
        }
    }
}

impl<G: xr::Graphics> SwapchainInner<G> {
    fn begin(&self) -> xr::Result<()> {
        self.stream.lock().unwrap().begin()
    }

    fn get_render_views(&self) -> (wgpu::TextureView, wgpu::TextureView) {
        let texture = &self.buffers[*self.image_index.lock().unwrap()];

        (
            texture.create_view(&wgpu::TextureViewDescriptor {
                dimension: Some(wgpu::TextureViewDimension::D2),
                array_layer_count: Some(1),
                ..Default::default()
            }),
            texture.create_view(&wgpu::TextureViewDescriptor {
                dimension: Some(wgpu::TextureViewDimension::D2),
                array_layer_count: Some(1),
                base_array_layer: 1,
                ..Default::default()
            }),
        )
    }

    fn acquire_image(&self) -> xr::Result<()> {
        let image_index = self.handle.lock().unwrap().acquire_image()?;
        *self.image_index.lock().unwrap() = image_index as _;
        Ok(())
    }

    fn wait_image(&self) -> xr::Result<()> {
        self.handle
            .lock()
            .unwrap()
            .wait_image(xr::Duration::INFINITE)
    }

    fn release_image(&self) -> xr::Result<()> {
        self.handle.lock().unwrap().release_image()
    }

    fn end(
        &self,
        predicted_display_time: xr::Time,
        views: &[openxr::View],
        stage: &xr::Space,
        resolution: UVec2,
        environment_blend_mode: xr::EnvironmentBlendMode,
        passthrough_layer: Option<&XrPassthroughLayer>,
    ) -> xr::Result<()> {
        let rect = xr::Rect2Di {
            offset: xr::Offset2Di { x: 0, y: 0 },
            extent: xr::Extent2Di {
                width: resolution.x as _,
                height: resolution.y as _,
            },
        };
        let swapchain = self.handle.lock().unwrap();
        if views.is_empty() {
            warn!("views are len of 0");
            return Ok(());
        }

        let make_view = |i: usize| {
            xr::CompositionLayerProjectionView::new()
                .pose(views[i].pose)
                .fov(views[i].fov)
                .sub_image(
                    xr::SwapchainSubImage::new()
                        .swapchain(&swapchain)
                        .image_array_index(i as u32)
                        .image_rect(rect),
                )
        };
        let views = [make_view(0), make_view(1)];

        match passthrough_layer {
            Some(pass) => {
                //bevy::log::info!("Rendering with pass through");
                self.stream.lock().unwrap().end(
                    predicted_display_time,
                    environment_blend_mode,
                    &[
                        &CompositionLayerPassthrough::from_xr_passthrough_layer(pass),
                        &xr::CompositionLayerProjection::new()
                            .layer_flags(CompositionLayerFlags::BLEND_TEXTURE_SOURCE_ALPHA)
                            .space(stage)
                            .views(&views),
                    ],
                )
            }
            None => {
                // bevy::log::info!("Rendering without pass through");
                self.stream.lock().unwrap().end(
                    predicted_display_time,
                    environment_blend_mode,
                    &[&xr::CompositionLayerProjection::new()
                        .space(stage)
                        .views(&views)],
                )
            }
        }
    }
}
