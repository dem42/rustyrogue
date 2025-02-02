use crate::{
    presentation::swap_chain::SwapChainContainer,
    textures::images::{Image, MipmapParam},
    util::result::Result,
};
use ash::{vk};
//////////////////////// Structs ///////////////////////
#[derive(Default)]
pub struct ColorResource {
    pub image: Image,
    pub view: vk::ImageView,
}
//////////////////////// Impls ///////////////////////
impl ColorResource {
    pub fn new(
        msaa_samples: vk::SampleCountFlags,
        instance: &ash::Instance,
        logical_device: &ash::Device,
        physical_device: vk::PhysicalDevice,
        swap_chain_container: &SwapChainContainer,
    ) -> Result<Self> {
        let color_format = swap_chain_container.swap_chain_format.format;

        let image = Image::new(
            swap_chain_container.swap_chain_extent.width,
            swap_chain_container.swap_chain_extent.height,
            MipmapParam::NoMipmap,
            msaa_samples,
            color_format,
            vk::ImageTiling::OPTIMAL,
            vk::ImageUsageFlags::TRANSIENT_ATTACHMENT | vk::ImageUsageFlags::COLOR_ATTACHMENT,
            vk::MemoryPropertyFlags::DEVICE_LOCAL,
            instance,
            physical_device,
            logical_device,
        )?;

        let view =
            image.create_image_view(color_format, vk::ImageAspectFlags::COLOR, logical_device)?;

        Ok(Self { image, view })
    }

    /// # Safety
    ///
    /// This method calls unsafe, low-level vulkan api functions to destroy images and free memory.
    /// It must be called with valid vulkan state.
    pub unsafe fn drop(self, logical_device: &ash::Device) {
        logical_device.destroy_image_view(self.view, None);
        logical_device.destroy_image(self.image.image, None);
        logical_device.free_memory(self.image.memory, None);
    }
}
