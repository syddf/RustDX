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

impl SamplerDesc {
    pub fn set_filter(&mut self, filter: Filter) -> &mut Self {
        self.0.Filter = filter as i32;
        self
    }

    pub fn with_filter(mut self, filter: Filter) -> Self {
        self.set_filter(filter);
        self
    }

    pub fn filter(&self) -> Filter {
        unsafe { std::mem::transmute(self.0.Filter) }
    }

    pub fn set_address_u(
        &mut self,
        address_u: TextureAddressMode,
    ) -> &mut Self {
        self.0.AddressU = address_u as i32;
        self
    }

    pub fn with_address_u(mut self, address_u: TextureAddressMode) -> Self {
        self.set_address_u(address_u);
        self
    }

    pub fn address_u(&self) -> TextureAddressMode {
        unsafe { std::mem::transmute(self.0.AddressU) }
    }

    pub fn set_address_v(
        &mut self,
        address_v: TextureAddressMode,
    ) -> &mut Self {
        self.0.AddressV = address_v as i32;
        self
    }

    pub fn with_address_v(mut self, address_v: TextureAddressMode) -> Self {
        self.set_address_v(address_v);
        self
    }

    pub fn address_v(&self) -> TextureAddressMode {
        unsafe { std::mem::transmute(self.0.AddressV) }
    }

    pub fn set_address_w(
        &mut self,
        address_w: TextureAddressMode,
    ) -> &mut Self {
        self.0.AddressW = address_w as i32;
        self
    }

    pub fn with_address_w(mut self, address_w: TextureAddressMode) -> Self {
        self.set_address_w(address_w);
        self
    }

    pub fn address_w(&self) -> TextureAddressMode {
        unsafe { std::mem::transmute(self.0.AddressW) }
    }

    pub fn set_mip_lod_bias(&mut self, mip_lod_bias: f32) -> &mut Self {
        self.0.MipLODBias = mip_lod_bias;
        self
    }

    pub fn with_mip_lod_bias(mut self, mip_lod_bias: f32) -> Self {
        self.set_mip_lod_bias(mip_lod_bias);
        self
    }

    pub fn mip_lod_bias(&self) -> f32 {
        self.0.MipLODBias
    }

    pub fn set_max_anisotropy(&mut self, max_anisotropy: u32) -> &mut Self {
        self.0.MaxAnisotropy = max_anisotropy;
        self
    }

    pub fn with_max_anisotropy(mut self, max_anisotropy: u32) -> Self {
        self.set_max_anisotropy(max_anisotropy);
        self
    }

    pub fn max_anisotropy(&self) -> u32 {
        self.0.MaxAnisotropy
    }

    pub fn set_comparison_func(
        &mut self,
        comparison_func: ComparisonFunc,
    ) -> &mut Self {
        self.0.ComparisonFunc = comparison_func as i32;
        self
    }

    pub fn with_comparison_func(
        mut self,
        comparison_func: ComparisonFunc,
    ) -> Self {
        self.set_comparison_func(comparison_func);
        self
    }

    pub fn comparison_func(&self) -> ComparisonFunc {
        unsafe { std::mem::transmute(self.0.ComparisonFunc) }
    }

    pub fn set_border_color(
        &mut self,
        border_color: [f32; 4usize],
    ) -> &mut Self {
        self.0.BorderColor = border_color;
        self
    }

    pub fn with_border_color(mut self, border_color: [f32; 4usize]) -> Self {
        self.set_border_color(border_color);
        self
    }

    pub fn border_color(&self) -> [f32; 4usize] {
        self.0.BorderColor
    }

    pub fn set_min_lod(&mut self, min_lod: f32) -> &mut Self {
        self.0.MinLOD = min_lod;
        self
    }

    pub fn with_min_lod(mut self, min_lod: f32) -> Self {
        self.set_min_lod(min_lod);
        self
    }

    pub fn min_lod(&self) -> f32 {
        self.0.MinLOD
    }

    pub fn set_max_lod(&mut self, max_lod: f32) -> &mut Self {
        self.0.MaxLOD = max_lod;
        self
    }

    pub fn with_max_lod(mut self, max_lod: f32) -> Self {
        self.set_max_lod(max_lod);
        self
    }

    pub fn max_lod(&self) -> f32 {
        self.0.MaxLOD
    }
}

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

impl StaticSamplerDesc {
    pub fn set_filter(&mut self, filter: Filter) -> &mut Self {
        self.0.Filter = filter as i32;
        self
    }

    pub fn with_filter(mut self, filter: Filter) -> Self {
        self.set_filter(filter);
        self
    }

    pub fn filter(&self) -> Filter {
        unsafe { std::mem::transmute(self.0.Filter) }
    }

    pub fn set_address_u(
        &mut self,
        address_u: TextureAddressMode,
    ) -> &mut Self {
        self.0.AddressU = address_u as i32;
        self
    }

    pub fn with_address_u(mut self, address_u: TextureAddressMode) -> Self {
        self.set_address_u(address_u);
        self
    }

    pub fn address_u(&self) -> TextureAddressMode {
        unsafe { std::mem::transmute(self.0.AddressU) }
    }

    pub fn set_address_v(
        &mut self,
        address_v: TextureAddressMode,
    ) -> &mut Self {
        self.0.AddressV = address_v as i32;
        self
    }

    pub fn with_address_v(mut self, address_v: TextureAddressMode) -> Self {
        self.set_address_v(address_v);
        self
    }

    pub fn address_v(&self) -> TextureAddressMode {
        unsafe { std::mem::transmute(self.0.AddressV) }
    }

    pub fn set_address_w(
        &mut self,
        address_w: TextureAddressMode,
    ) -> &mut Self {
        self.0.AddressW = address_w as i32;
        self
    }

    pub fn with_address_w(mut self, address_w: TextureAddressMode) -> Self {
        self.set_address_w(address_w);
        self
    }

    pub fn address_w(&self) -> TextureAddressMode {
        unsafe { std::mem::transmute(self.0.AddressW) }
    }

    pub fn set_mip_lod_bias(&mut self, mip_lod_bias: f32) -> &mut Self {
        self.0.MipLODBias = mip_lod_bias;
        self
    }

    pub fn with_mip_lod_bias(mut self, mip_lod_bias: f32) -> Self {
        self.set_mip_lod_bias(mip_lod_bias);
        self
    }

    pub fn mip_lod_bias(&self) -> f32 {
        self.0.MipLODBias
    }

    pub fn set_max_anisotropy(&mut self, max_anisotropy: u32) -> &mut Self {
        self.0.MaxAnisotropy = max_anisotropy;
        self
    }

    pub fn with_max_anisotropy(mut self, max_anisotropy: u32) -> Self {
        self.set_max_anisotropy(max_anisotropy);
        self
    }

    pub fn max_anisotropy(&self) -> u32 {
        self.0.MaxAnisotropy
    }

    pub fn set_comparison_func(
        &mut self,
        comparison_func: ComparisonFunc,
    ) -> &mut Self {
        self.0.ComparisonFunc = comparison_func as i32;
        self
    }

    pub fn with_comparison_func(
        mut self,
        comparison_func: ComparisonFunc,
    ) -> Self {
        self.set_comparison_func(comparison_func);
        self
    }

    pub fn comparison_func(&self) -> ComparisonFunc {
        unsafe { std::mem::transmute(self.0.ComparisonFunc) }
    }

    pub fn set_border_color(
        &mut self,
        border_color: StaticBorderColor,
    ) -> &mut Self {
        self.0.BorderColor = border_color as i32;
        self
    }

    pub fn with_border_color(
        mut self,
        border_color: StaticBorderColor,
    ) -> Self {
        self.set_border_color(border_color);
        self
    }

    pub fn border_color(&self) -> StaticBorderColor {
        unsafe { std::mem::transmute(self.0.BorderColor) }
    }

    pub fn set_min_lod(&mut self, min_lod: f32) -> &mut Self {
        self.0.MinLOD = min_lod;
        self
    }

    pub fn with_min_lod(mut self, min_lod: f32) -> Self {
        self.set_min_lod(min_lod);
        self
    }

    pub fn min_lod(&self) -> f32 {
        self.0.MinLOD
    }

    pub fn set_max_lod(&mut self, max_lod: f32) -> &mut Self {
        self.0.MaxLOD = max_lod;
        self
    }

    pub fn with_max_lod(mut self, max_lod: f32) -> Self {
        self.set_max_lod(max_lod);
        self
    }

    pub fn max_lod(&self) -> f32 {
        self.0.MaxLOD
    }

    pub fn set_shader_register(&mut self, shader_register: u32) -> &mut Self {
        self.0.ShaderRegister = shader_register;
        self
    }

    pub fn with_shader_register(mut self, shader_register: u32) -> Self {
        self.set_shader_register(shader_register);
        self
    }

    pub fn shader_register(&self) -> u32 {
        self.0.ShaderRegister
    }

    pub fn set_register_space(&mut self, register_space: u32) -> &mut Self {
        self.0.RegisterSpace = register_space;
        self
    }

    pub fn with_register_space(mut self, register_space: u32) -> Self {
        self.set_register_space(register_space);
        self
    }

    pub fn register_space(&self) -> u32 {
        self.0.RegisterSpace
    }

    pub fn set_shader_visibility(
        &mut self,
        shader_visibility: ShaderVisibility,
    ) -> &mut Self {
        self.0.ShaderVisibility = shader_visibility as i32;
        self
    }

    pub fn with_shader_visibility(
        mut self,
        shader_visibility: ShaderVisibility,
    ) -> Self {
        self.set_shader_visibility(shader_visibility);
        self
    }

    pub fn shader_visibility(&self) -> ShaderVisibility {
        unsafe { std::mem::transmute(self.0.ShaderVisibility) }
    }
}
