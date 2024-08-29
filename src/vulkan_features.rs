use ash::vk::{self, PhysicalDeviceFeatures};
use ash::vk::{Bool32, PhysicalDeviceVulkan12Features, PhysicalDeviceVulkan13Features};
use vulkan_has_required_features::{
    CheckRequiredFeatures, FromPhysicalDeviceFeatures, FromPhysicalDeviceFeatures12,
    FromPhysicalDeviceFeatures13,
};

impl CheckRequiredFeatures for PhysicalDeviceFeatures {
    fn check_for_required_features(&self, required_features: &Self) -> bool {
        let device_features = PhysicalDeviceFeaturesDuplicated::from_duplicated_type(*self);
        let required = PhysicalDeviceFeaturesDuplicated::from_duplicated_type(*required_features);
        device_features.check_for_required_features(&required)
    }
}

impl<'a> CheckRequiredFeatures for PhysicalDeviceVulkan12Features<'a> {
    fn check_for_required_features(&self, required_features: &Self) -> bool {
        let device_features = PhysicalDeviceVulkan12FeaturesDuplicated::from_duplicated_type(*self);
        let required =
            PhysicalDeviceVulkan12FeaturesDuplicated::from_duplicated_type(*required_features);
        device_features.check_for_required_features(&required)
    }
}

impl<'a> CheckRequiredFeatures for PhysicalDeviceVulkan13Features<'a> {
    fn check_for_required_features(&self, required_features: &Self) -> bool {
        let device_features = PhysicalDeviceVulkan13FeaturesDuplicated::from_duplicated_type(*self);
        let required =
            PhysicalDeviceVulkan13FeaturesDuplicated::from_duplicated_type(*required_features);
        device_features.check_for_required_features(&required)
    }
}

#[derive(Copy, Clone, Default, CheckRequiredFeatures, FromPhysicalDeviceFeatures)]
pub struct PhysicalDeviceFeaturesDuplicated {
    pub robust_buffer_access: Bool32,
    pub full_draw_index_uint32: Bool32,
    pub image_cube_array: Bool32,
    pub independent_blend: Bool32,
    pub geometry_shader: Bool32,
    pub tessellation_shader: Bool32,
    pub sample_rate_shading: Bool32,
    pub dual_src_blend: Bool32,
    pub logic_op: Bool32,
    pub multi_draw_indirect: Bool32,
    pub draw_indirect_first_instance: Bool32,
    pub depth_clamp: Bool32,
    pub depth_bias_clamp: Bool32,
    pub fill_mode_non_solid: Bool32,
    pub depth_bounds: Bool32,
    pub wide_lines: Bool32,
    pub large_points: Bool32,
    pub alpha_to_one: Bool32,
    pub multi_viewport: Bool32,
    pub sampler_anisotropy: Bool32,
    pub texture_compression_etc2: Bool32,
    pub texture_compression_astc_ldr: Bool32,
    pub texture_compression_bc: Bool32,
    pub occlusion_query_precise: Bool32,
    pub pipeline_statistics_query: Bool32,
    pub vertex_pipeline_stores_and_atomics: Bool32,
    pub fragment_stores_and_atomics: Bool32,
    pub shader_tessellation_and_geometry_point_size: Bool32,
    pub shader_image_gather_extended: Bool32,
    pub shader_storage_image_extended_formats: Bool32,
    pub shader_storage_image_multisample: Bool32,
    pub shader_storage_image_read_without_format: Bool32,
    pub shader_storage_image_write_without_format: Bool32,
    pub shader_uniform_buffer_array_dynamic_indexing: Bool32,
    pub shader_sampled_image_array_dynamic_indexing: Bool32,
    pub shader_storage_buffer_array_dynamic_indexing: Bool32,
    pub shader_storage_image_array_dynamic_indexing: Bool32,
    pub shader_clip_distance: Bool32,
    pub shader_cull_distance: Bool32,
    pub shader_float64: Bool32,
    pub shader_int64: Bool32,
    pub shader_int16: Bool32,
    pub shader_resource_residency: Bool32,
    pub shader_resource_min_lod: Bool32,
    pub sparse_binding: Bool32,
    pub sparse_residency_buffer: Bool32,
    pub sparse_residency_image2_d: Bool32,
    pub sparse_residency_image3_d: Bool32,
    pub sparse_residency2_samples: Bool32,
    pub sparse_residency4_samples: Bool32,
    pub sparse_residency8_samples: Bool32,
    pub sparse_residency16_samples: Bool32,
    pub sparse_residency_aliased: Bool32,
    pub variable_multisample_rate: Bool32,
    pub inherited_queries: Bool32,
}

pub trait CheckRequiredFeatures {
    fn check_for_required_features(&self, required_features: &Self) -> bool;
}

trait FromPhysicalDeviceFeatures {
    fn from_duplicated_type(other: vk::PhysicalDeviceFeatures) -> Self;
}

trait FromPhysicalDeviceFeatures12 {
    fn from_duplicated_type(other: vk::PhysicalDeviceVulkan12Features) -> Self;
}

trait FromPhysicalDeviceFeatures13 {
    fn from_duplicated_type(other: vk::PhysicalDeviceVulkan13Features) -> Self;
}

#[derive(Copy, Clone, CheckRequiredFeatures, FromPhysicalDeviceFeatures12)]
pub struct PhysicalDeviceVulkan12FeaturesDuplicated {
    pub sampler_mirror_clamp_to_edge: Bool32,
    pub draw_indirect_count: Bool32,
    pub storage_buffer8_bit_access: Bool32,
    pub uniform_and_storage_buffer8_bit_access: Bool32,
    pub storage_push_constant8: Bool32,
    pub shader_buffer_int64_atomics: Bool32,
    pub shader_shared_int64_atomics: Bool32,
    pub shader_float16: Bool32,
    pub shader_int8: Bool32,
    pub descriptor_indexing: Bool32,
    pub shader_input_attachment_array_dynamic_indexing: Bool32,
    pub shader_uniform_texel_buffer_array_dynamic_indexing: Bool32,
    pub shader_storage_texel_buffer_array_dynamic_indexing: Bool32,
    pub shader_uniform_buffer_array_non_uniform_indexing: Bool32,
    pub shader_sampled_image_array_non_uniform_indexing: Bool32,
    pub shader_storage_buffer_array_non_uniform_indexing: Bool32,
    pub shader_storage_image_array_non_uniform_indexing: Bool32,
    pub shader_input_attachment_array_non_uniform_indexing: Bool32,
    pub shader_uniform_texel_buffer_array_non_uniform_indexing: Bool32,
    pub shader_storage_texel_buffer_array_non_uniform_indexing: Bool32,
    pub descriptor_binding_uniform_buffer_update_after_bind: Bool32,
    pub descriptor_binding_sampled_image_update_after_bind: Bool32,
    pub descriptor_binding_storage_image_update_after_bind: Bool32,
    pub descriptor_binding_storage_buffer_update_after_bind: Bool32,
    pub descriptor_binding_uniform_texel_buffer_update_after_bind: Bool32,
    pub descriptor_binding_storage_texel_buffer_update_after_bind: Bool32,
    pub descriptor_binding_update_unused_while_pending: Bool32,
    pub descriptor_binding_partially_bound: Bool32,
    pub descriptor_binding_variable_descriptor_count: Bool32,
    pub runtime_descriptor_array: Bool32,
    pub sampler_filter_minmax: Bool32,
    pub scalar_block_layout: Bool32,
    pub imageless_framebuffer: Bool32,
    pub uniform_buffer_standard_layout: Bool32,
    pub shader_subgroup_extended_types: Bool32,
    pub separate_depth_stencil_layouts: Bool32,
    pub host_query_reset: Bool32,
    pub timeline_semaphore: Bool32,
    pub buffer_device_address: Bool32,
    pub buffer_device_address_capture_replay: Bool32,
    pub buffer_device_address_multi_device: Bool32,
    pub vulkan_memory_model: Bool32,
    pub vulkan_memory_model_device_scope: Bool32,
    pub vulkan_memory_model_availability_visibility_chains: Bool32,
    pub shader_output_viewport_index: Bool32,
    pub shader_output_layer: Bool32,
    pub subgroup_broadcast_dynamic_id: Bool32,
}

#[derive(Copy, Clone, CheckRequiredFeatures, FromPhysicalDeviceFeatures13)]
pub struct PhysicalDeviceVulkan13FeaturesDuplicated {
    pub robust_image_access: Bool32,
    pub inline_uniform_block: Bool32,
    pub descriptor_binding_inline_uniform_block_update_after_bind: Bool32,
    pub pipeline_creation_cache_control: Bool32,
    pub private_data: Bool32,
    pub shader_demote_to_helper_invocation: Bool32,
    pub shader_terminate_invocation: Bool32,
    pub subgroup_size_control: Bool32,
    pub compute_full_subgroups: Bool32,
    pub synchronization2: Bool32,
    pub texture_compression_astc_hdr: Bool32,
    pub shader_zero_initialize_workgroup_memory: Bool32,
    pub dynamic_rendering: Bool32,
    pub shader_integer_dot_product: Bool32,
    pub maintenance4: Bool32,
}

#[cfg(test)]
pub mod feature_tests {
    use super::*;

    #[test]
    pub fn check_for_required_features() {
        let required_features = vk::PhysicalDeviceFeatures::default()
            .geometry_shader(true)
            .sampler_anisotropy(true);
        let device_features = vk::PhysicalDeviceFeatures::default()
            .geometry_shader(true)
            .sampler_anisotropy(true)
            .alpha_to_one(false);

        assert!(device_features.check_for_required_features(&required_features));
    }

    #[test]
    pub fn check_for_required_features_missing() {
        let required_features = vk::PhysicalDeviceFeatures::default()
            .geometry_shader(true)
            .sampler_anisotropy(true);
        let device_features = vk::PhysicalDeviceFeatures::default()
            .geometry_shader(false)
            .sampler_anisotropy(true);

        assert!(!device_features.check_for_required_features(&required_features));
    }

    #[test]
    pub fn check_for_required_feature_12() {
        let required_features = vk::PhysicalDeviceVulkan12Features::default()
            .buffer_device_address(true)
            .descriptor_indexing(true);
        let features12 = vk::PhysicalDeviceVulkan12Features::default()
            .buffer_device_address(true)
            .descriptor_indexing(true);

        assert!(features12.check_for_required_features(&required_features));
    }

    #[test]
    pub fn check_for_required_feature_13() {
        let required_features = vk::PhysicalDeviceVulkan13Features::default()
            .dynamic_rendering(true)
            .synchronization2(true);
        let features13 = vk::PhysicalDeviceVulkan13Features::default()
            .dynamic_rendering(true)
            .synchronization2(true);

        assert!(features13.check_for_required_features(&required_features));
    }
}
