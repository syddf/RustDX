#![allow(non_upper_case_globals)]
#![allow(non_camel_case_types)]
#![allow(non_snake_case)]

use crate::raw_bindings::d3d12::*;
use std::{ marker::PhantomData };
use crate::d3d12_common::DxResult;
use crate::d3d12_enum::*;

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
