use ash::vk;

use crate::VulkanSurface;

#[derive(Clone)]
pub struct SwapchainSupportDetails {
    pub capabilities: vk::SurfaceCapabilitiesKHR,
    pub formats: Vec<vk::SurfaceFormatKHR>,
    pub present_modes: Vec<vk::PresentModeKHR>,
}

impl SwapchainSupportDetails {
    pub fn new(device: &vk::PhysicalDevice, surface: &VulkanSurface) -> Self {
        let capabilities = unsafe {
            surface
                .loader
                .get_physical_device_surface_capabilities(*device, surface.handle)
        }
        .expect("failed to get surface capabilites!");
        let formats = unsafe {
            surface
                .loader
                .get_physical_device_surface_formats(*device, surface.handle)
        }
        .expect("failed to get device surface formats!");
        let present_modes = unsafe {
            surface
                .loader
                .get_physical_device_surface_present_modes(*device, surface.handle)
        }
        .expect("failed to get device surface present modes!");

        Self {
            capabilities,
            formats,
            present_modes,
        }
    }
}
