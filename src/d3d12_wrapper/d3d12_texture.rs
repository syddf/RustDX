#![allow(non_upper_case_globals)]
#![allow(non_camel_case_types)]
#![allow(non_snake_case)]

use crate::raw_bindings::d3d12::*;
use std::{ marker::PhantomData };
use crate::d3d12_common::DxResult;
use crate::d3d12_enum::*;
use crate::d3d12_resource::*;

/// Wrapper around D3D12_SAMPLER_DESC structure
#[derive(Copy, Clone, Default, Debug)]
#[repr(transparent)]
pub struct SamplerDesc(pub(crate) D3D12_SAMPLER_DESC);

/// Wrapper around D3D12_STATIC_SAMPLER_DESC structure
#[repr(transparent)]
#[derive(Copy, Clone, Debug)]
pub struct StaticSamplerDesc(pub(crate) D3D12_STATIC_SAMPLER_DESC);

// based on the first constructor of CD3DX12_STATIC_SAMPLER_DESC
impl Default for StaticSamplerDesc {
    fn default() -> Self {
        Self(D3D12_STATIC_SAMPLER_DESC {
            Filter: D3D12_FILTER_D3D12_FILTER_ANISOTROPIC,
            AddressU:
                D3D12_TEXTURE_ADDRESS_MODE_D3D12_TEXTURE_ADDRESS_MODE_WRAP,
            AddressV:
                D3D12_TEXTURE_ADDRESS_MODE_D3D12_TEXTURE_ADDRESS_MODE_WRAP,
            AddressW:
                D3D12_TEXTURE_ADDRESS_MODE_D3D12_TEXTURE_ADDRESS_MODE_WRAP,
            MipLODBias: 0.,
            MaxAnisotropy: 16,
            ComparisonFunc:
                D3D12_COMPARISON_FUNC_D3D12_COMPARISON_FUNC_LESS_EQUAL,
            BorderColor:
                D3D12_STATIC_BORDER_COLOR_D3D12_STATIC_BORDER_COLOR_OPAQUE_WHITE,
            MinLOD: 0.,
            // ToDo: D3D12_FLOAT32_MAX - for some reason bindgen did not include this constant
            MaxLOD: 3.402823466e+38,
            ShaderRegister: 0,
            RegisterSpace: 0,
            ShaderVisibility:
                D3D12_SHADER_VISIBILITY_D3D12_SHADER_VISIBILITY_ALL,
        })
    }
}

#[repr(i32)]
#[derive(Debug, Clone, Copy)]
#[cfg_attr(feature = "eq", derive(PartialEq, Eq))]
#[cfg_attr(feature = "hash", derive(Hash))]
pub enum TextureLayout {
    Unknown = D3D12_TEXTURE_LAYOUT_D3D12_TEXTURE_LAYOUT_UNKNOWN,
    RowMajor = D3D12_TEXTURE_LAYOUT_D3D12_TEXTURE_LAYOUT_ROW_MAJOR,
    L64KbUndefinedSwizzle =
        D3D12_TEXTURE_LAYOUT_D3D12_TEXTURE_LAYOUT_64KB_UNDEFINED_SWIZZLE,
    L64KbStandardSwizzle =
        D3D12_TEXTURE_LAYOUT_D3D12_TEXTURE_LAYOUT_64KB_STANDARD_SWIZZLE,
}

#[derive(Hash, PartialOrd, Ord, PartialEq, Eq, Copy, Clone, Debug)]
#[repr(transparent)]
pub struct SubresourceFootprint(pub(crate) D3D12_SUBRESOURCE_FOOTPRINT);

impl Default for SubresourceFootprint {
    fn default() -> Self {
        Self(D3D12_SUBRESOURCE_FOOTPRINT {
            Format: Format::R8G8B8A8Unorm as i32,
            Width: 0,
            Height: 1,
            Depth: 1,
            RowPitch: 0,
        })
    }
}

#[repr(transparent)]
#[derive(Debug)]
pub struct TextureCopyLocation(pub(crate) D3D12_TEXTURE_COPY_LOCATION);

impl TextureCopyLocation {
    pub fn new_placed_footprint(
        resource: &Resource,
        footprint: PlacedSubresourceFootprint,
    ) -> Self {
        Self(D3D12_TEXTURE_COPY_LOCATION {
            pResource: resource.this,
            Type: TextureCopyType::PlacedFootprint as i32,
            __bindgen_anon_1: D3D12_TEXTURE_COPY_LOCATION__bindgen_ty_1 {
                PlacedFootprint: footprint.0,
            },
        })
    }

    pub fn new_subresource_index(resource: &Resource, index: u32) -> Self {
        Self(D3D12_TEXTURE_COPY_LOCATION {
            pResource: resource.this,
            Type: TextureCopyType::SubresourceIndex as i32,
            __bindgen_anon_1: D3D12_TEXTURE_COPY_LOCATION__bindgen_ty_1 {
                SubresourceIndex: index,
            },
        })
    }

    pub fn resource(&self) -> Resource {
        let resource = Resource {
            this: self.0.pResource,
        };
        resource.add_ref();
        resource
    }

    pub fn copy_type(&self) -> TextureCopyType {
        unsafe { std::mem::transmute(self.0.Type) }
    }
}
