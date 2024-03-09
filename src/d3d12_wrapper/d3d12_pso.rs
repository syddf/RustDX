#![allow(non_upper_case_globals)]
#![allow(non_camel_case_types)]
#![allow(non_snake_case)]

use std::ffi::{CStr, CString, NulError};
use crate::raw_bindings::d3d12::*;
use std::{ marker::PhantomData };
use crate::d3d12_common::DxResult;
use crate::d3d12_texture::*;
use std::slice;
use std::str::Utf8Error;
use crate::d3d12_enum::*;
use crate::d3d12_common::*;
use crate::d3d12_buffer::*;

#[repr(transparent)]
#[derive(Debug)]
pub struct GraphicsPipelineStateDesc<'rs, 'sh, 'so, 'il>(
    pub D3D12_GRAPHICS_PIPELINE_STATE_DESC,
    PhantomData<&'rs RootSignature>,
    PhantomData<&'sh ShaderBytecode<'sh>>,
    PhantomData<&'so StreamOutputDesc<'so>>,
    PhantomData<&'il InputLayoutDesc<'il>>,
);

impl<'rs, 'sh, 'so, 'il> Default
    for GraphicsPipelineStateDesc<'rs, 'sh, 'so, 'il>
{
    fn default() -> Self {
        Self(
            D3D12_GRAPHICS_PIPELINE_STATE_DESC {
                pRootSignature: std::ptr::null_mut(),
                VS: ShaderBytecode::default().0,
                PS: ShaderBytecode::default().0,
                DS: ShaderBytecode::default().0,
                HS: ShaderBytecode::default().0,
                GS: ShaderBytecode::default().0,
                StreamOutput: StreamOutputDesc::default().0,
                BlendState: BlendDesc::default().0,
                SampleMask: std::u32::MAX,
                RasterizerState: RasterizerDesc::default().0,
                DepthStencilState: DepthStencilDesc::default().0,
                InputLayout: InputLayoutDesc::default().0,
                IBStripCutValue: IndexBufferStripCutValue::Disabled as i32,
                PrimitiveTopologyType: PrimitiveTopologyType::Undefined as i32,
                NumRenderTargets: D3D12_SIMULTANEOUS_RENDER_TARGET_COUNT as u32,
                RTVFormats: [Format::Unknown as i32;
                    D3D12_SIMULTANEOUS_RENDER_TARGET_COUNT as usize],
                DSVFormat: Format::Unknown as i32,
                SampleDesc: SampleDesc::default().0,
                NodeMask: 0,
                CachedPSO: CachedPipelineState::default().0,
                Flags: PipelineStateFlags::None.bits(),
            },
            PhantomData, // rs
            PhantomData, // sh
            PhantomData, // so
            PhantomData, // il
        )
    }
}


#[derive(Debug)]
#[repr(transparent)]
pub struct RootSignature {
    pub this: *mut ID3D12RootSignature,
}

impl_com_object_set_get_name!(RootSignature);

unsafe impl Send for RootSignature {}

impl RootSignature {
    // ToDo: rename this function or move it elsewhere?
    pub fn serialize_versioned(
        desc: &VersionedRootSignatureDesc,
    ) -> (Blob, DxResult<()>) {
        let mut blob: *mut ID3DBlob = std::ptr::null_mut();
        let mut error_blob: *mut ID3DBlob = std::ptr::null_mut();
        unsafe {
            let ret_code = D3D12SerializeVersionedRootSignature(
                &desc.0,
                &mut blob,
                &mut error_blob,
            );

            if success!(ret_code) {
                (Blob { this: blob }, Ok(()))
            } else {
                (
                    Blob { this: error_blob },
                    Err(DxError::new(
                        "D3D12SerializeVersionedRootSignature",
                        ret_code,
                    )),
                )
            }
        }
    }
}

/// Wrapper around D3D12_VERSIONED_ROOT_SIGNATURE_DESC structure
#[derive(Copy, Clone, Default, Debug)]
#[repr(transparent)]
pub struct VersionedRootSignatureDesc(
    pub D3D12_VERSIONED_ROOT_SIGNATURE_DESC,
);
/// Wrapper around D3D12_ROOT_SIGNATURE_DESC1 structure
#[derive(Hash, PartialOrd, Ord, PartialEq, Eq, Default, Debug)]
#[repr(transparent)]
pub struct RootSignatureDesc<'a, 'b>(
    pub D3D12_ROOT_SIGNATURE_DESC1,
    PhantomData<&'a RootParameter<'a>>,
    PhantomData<&'b StaticSamplerDesc>,
);

impl<'a, 'b> RootSignatureDesc<'a, 'b> {
    pub fn parameters(&self) -> &'a [RootParameter] {
        unsafe {
            slice::from_raw_parts(
                self.0.pParameters as *const D3D12_ROOT_PARAMETER1
                    as *const RootParameter,
                self.0.NumParameters as usize,
            )
        }
    }
}

/// Wrapper around D3D12_ROOT_PARAMETER1 structure
#[derive(Debug, Default)]
#[repr(transparent)]
pub struct RootParameter<'a>(
    pub D3D12_ROOT_PARAMETER1,
    PhantomData<&'a RootDescriptorTable<'a>>,
);

impl<'a> RootParameter<'a> {
    pub fn parameter_type(&self) -> RootParameterType {
        unsafe { std::mem::transmute(self.0.ParameterType) }
    }

    pub fn new_descriptor_table(
        mut self,
        descriptor_table: &'a RootDescriptorTable<'a>,
    ) -> Self {
        self.0.ParameterType = RootParameterType::DescriptorTable as i32;
        self.0.__bindgen_anon_1.DescriptorTable = descriptor_table.0;
        self.1 = PhantomData;
        self
    }

    pub fn descriptor_table(&self) -> Option<RootDescriptorTable> {
        unsafe {
            match self.parameter_type() {
                RootParameterType::DescriptorTable => {
                    Some(RootDescriptorTable(
                        self.0.__bindgen_anon_1.DescriptorTable,
                        PhantomData,
                    ))
                }
                _ => None,
            }
        }
    }

    pub fn new_constants(mut self, constants: &RootConstants) -> Self {
        self.0.ParameterType = RootParameterType::T32BitConstants as i32;
        self.0.__bindgen_anon_1.Constants = constants.0;
        self
    }

    pub fn constants(&self) -> Option<RootConstants> {
        unsafe {
            match self.parameter_type() {
                RootParameterType::T32BitConstants => {
                    Some(RootConstants(self.0.__bindgen_anon_1.Constants))
                }
                _ => None,
            }
        }
    }

    pub fn new_descriptor(
        mut self,
        descriptor: &RootDescriptor,
        descriptor_type: RootParameterType,
    ) -> Self {
        assert!(
            descriptor_type == RootParameterType::Cbv
                || descriptor_type == RootParameterType::Srv
                || descriptor_type == RootParameterType::Uav
        );
        self.0.ParameterType = descriptor_type as i32;
        self.0.__bindgen_anon_1.Descriptor = descriptor.0;
        self
    }

    pub fn descriptor(&self) -> Option<RootDescriptor> {
        unsafe {
            match self.parameter_type() {
                RootParameterType::Cbv
                | RootParameterType::Srv
                | RootParameterType::Uav => {
                    Some(RootDescriptor(self.0.__bindgen_anon_1.Descriptor))
                }
                _ => None,
            }
        }
    }
}

/// Wrapper around D3D12_ROOT_DESCRIPTOR_TABLE1 structure
#[derive(Hash, PartialOrd, Ord, PartialEq, Eq, Default, Debug)]
#[repr(transparent)]
pub struct RootDescriptorTable<'a>(
    pub D3D12_ROOT_DESCRIPTOR_TABLE1,
    PhantomData<&'a DescriptorRange>,
);

/// Wrapper around D3D12_SHADER_BYTECODE structure
#[repr(transparent)]
#[derive(Hash, PartialOrd, Ord, PartialEq, Eq, Debug)]
pub struct ShaderBytecode<'a>(
    pub D3D12_SHADER_BYTECODE,
    PhantomData<&'a [u8]>,
);

impl<'a> Default for ShaderBytecode<'a> {
    fn default() -> ShaderBytecode<'a> {
        ShaderBytecode(
            D3D12_SHADER_BYTECODE {
                pShaderBytecode: std::ptr::null(),
                BytecodeLength: 0,
            },
            PhantomData,
        )
    }
}

impl<'a> ShaderBytecode<'a> {
    pub fn new(data: &'a [u8]) -> ShaderBytecode<'a> {
        Self(
            D3D12_SHADER_BYTECODE {
                pShaderBytecode: data.as_ptr() as *const std::ffi::c_void,
                BytecodeLength: data.len() as u64,
            },
            PhantomData,
        )
    }
}

/// Wrapper around D3D12_SO_DECLARATION_ENTRY structure
#[derive(Hash, PartialOrd, Ord, PartialEq, Eq, Debug)]
pub struct SoDeclarationEntry<'a>(
    pub D3D12_SO_DECLARATION_ENTRY,
    PhantomData<&'a str>,
);

// We need this because we transfer ownership of the CString "name" into
// the raw C string (const char*) "SemanticName". Since this memory has to be
// valid until the destruction of this struct, we need to regain that memory
// back so it can be destroyed correctly
impl<'a> Drop for SoDeclarationEntry<'a> {
    fn drop(&mut self) {
        unsafe {
            let _regained_name = CString::from_raw(
                self.0.SemanticName as *mut std::os::raw::c_char,
            );
        }
    }
}

/// Wrapper around D3D12_STREAM_OUTPUT_DESC structure
#[repr(transparent)]
#[derive(Hash, PartialOrd, Ord, PartialEq, Eq, Debug)]
pub struct StreamOutputDesc<'a>(
    pub D3D12_STREAM_OUTPUT_DESC,
    PhantomData<&'a [SoDeclarationEntry<'a>]>,
);

impl<'a> Default for StreamOutputDesc<'a> {
    fn default() -> Self {
        Self(
            D3D12_STREAM_OUTPUT_DESC {
                pSODeclaration: std::ptr::null(),
                NumEntries: 0,
                pBufferStrides: std::ptr::null(),
                NumStrides: 0,
                RasterizedStream: 0,
            },
            PhantomData,
        )
    }
}

/// Wrapper around D3D12_INPUT_LAYOUT_DESC structure
#[repr(transparent)]
#[derive(Hash, PartialOrd, Ord, PartialEq, Eq, Debug)]
pub struct InputLayoutDesc<'a>(
    pub D3D12_INPUT_LAYOUT_DESC,
    PhantomData<&'a [InputElementDesc<'a>]>,
);

impl Default for InputLayoutDesc<'_> {
    fn default() -> Self {
        Self(
            D3D12_INPUT_LAYOUT_DESC {
                pInputElementDescs: std::ptr::null(),
                NumElements: 0,
            },
            PhantomData,
        )
    }
}

/// Wrapper around D3D12_INPUT_ELEMENT_DESC structure
#[repr(transparent)]
#[derive(Debug, Hash, PartialOrd, Ord, PartialEq, Eq)]
pub struct InputElementDesc<'a>(
    pub D3D12_INPUT_ELEMENT_DESC,
    PhantomData<&'a CStr>,
);

impl<'a> Default for InputElementDesc<'a> {
    fn default() -> InputElementDesc<'a> {
        InputElementDesc(D3D12_INPUT_ELEMENT_DESC {
            SemanticName: std::ptr::null(),
            SemanticIndex: 0,
            Format: Format::Unknown as i32,
            InputSlot: 0,
            AlignedByteOffset: 0,
            InputSlotClass:
        D3D12_INPUT_CLASSIFICATION_D3D12_INPUT_CLASSIFICATION_PER_VERTEX_DATA,
            InstanceDataStepRate: 0,
        },
        PhantomData
    )
    }
}

// We need this because we transfer ownership of the CString "name" into
// the raw C string (const char*) "SemanticName". Since this memory has to be
// valid until the destruction of this struct, we need to regain that memory
// back so it can be destroyed correctly
impl<'a> Drop for InputElementDesc<'a> {
    fn drop(&mut self) {
        unsafe {
            let _regained_name = CString::from_raw(
                self.0.SemanticName as *mut std::os::raw::c_char,
            );
        }
    }
}

/// Wrapper around D3D12_ROOT_CONSTANTS structure
#[derive(Default, Debug, Hash, PartialOrd, Ord, PartialEq, Eq, Clone)]
#[repr(transparent)]
pub struct RootConstants(pub D3D12_ROOT_CONSTANTS);

/// Wrapper around D3D12_ROOT_DESCRIPTOR1 structure
#[derive(Hash, PartialOrd, Ord, PartialEq, Eq, Copy, Clone, Default, Debug)]
#[repr(transparent)]
pub struct RootDescriptor(pub D3D12_ROOT_DESCRIPTOR1);

/// Newtype around [u32] since it has a special value of [DESCRIPTOR_RANGE_OFFSET_APPEND]
#[derive(Hash, PartialOrd, Ord, PartialEq, Eq, Copy, Clone, Debug)]
pub struct DescriptorRangeOffset(pub u32);

impl From<u32> for DescriptorRangeOffset {
    fn from(count: u32) -> Self {
        Self(count)
    }
}

impl DescriptorRangeOffset {
    pub fn append() -> Self {
        Self(D3D12_DESCRIPTOR_RANGE_OFFSET_APPEND)
    }
}

/// Wrapper around D3D12_DESCRIPTOR_RANGE1 structure
#[derive(Default, Debug, Hash, PartialOrd, Ord, PartialEq, Eq, Clone)]
#[repr(transparent)]
pub struct DescriptorRange(pub D3D12_DESCRIPTOR_RANGE1);

#[derive(Debug, Hash, PartialOrd, Ord, PartialEq, Eq, Clone, Copy)]
#[repr(transparent)]
pub struct SampleDesc(pub DXGI_SAMPLE_DESC);

impl Default for SampleDesc {
    fn default() -> Self {
        Self(DXGI_SAMPLE_DESC {
            Count: 1,
            Quality: 0,
        })
    }
}

#[derive(Debug)]
#[repr(transparent)]
pub struct PipelineState {
    pub this: *mut ID3D12PipelineState,
}
impl_com_object_set_get_name!(PipelineState);
unsafe impl Send for PipelineState {}


#[derive(Debug, PartialOrd, PartialEq, Clone, Copy)]
#[repr(transparent)]
pub struct Viewport(pub D3D12_VIEWPORT);

impl Default for Viewport {
    fn default() -> Self {
        Viewport(D3D12_VIEWPORT {
            TopLeftX: 0.,
            TopLeftY: 0.,
            Width: 0.,
            Height: 0.,
            MinDepth: 0.,
            MaxDepth: 1.,
        })
    }
}

#[derive(Hash, PartialOrd, Ord, PartialEq, Eq, Copy, Clone, Debug)]
#[repr(transparent)]
pub struct RenderTargetBlendDesc(pub D3D12_RENDER_TARGET_BLEND_DESC);

// defaults from d3dx12.h
impl Default for RenderTargetBlendDesc {
    fn default() -> Self {
        Self(D3D12_RENDER_TARGET_BLEND_DESC {
            BlendEnable: 0,
            LogicOpEnable: 0,
            SrcBlend: Blend::One as i32,
            DestBlend: Blend::Zero as i32,
            BlendOp: BlendOp::Add as i32,
            SrcBlendAlpha: Blend::One as i32,
            DestBlendAlpha: Blend::Zero as i32,
            BlendOpAlpha: BlendOp::Add as i32,
            LogicOp: LogicOp::NoOp as i32,
            RenderTargetWriteMask:
                D3D12_COLOR_WRITE_ENABLE_D3D12_COLOR_WRITE_ENABLE_ALL as u8,
        })
    }
}


#[derive(Hash, PartialOrd, Ord, PartialEq, Eq, Copy, Clone, Debug)]
#[repr(transparent)]
pub struct BlendDesc(pub D3D12_BLEND_DESC);

// defaults from d3dx12.h
impl Default for BlendDesc {
    fn default() -> Self {
        Self(D3D12_BLEND_DESC {
            AlphaToCoverageEnable: 0,
            IndependentBlendEnable: 0,
            RenderTarget: [RenderTargetBlendDesc::default().0; 8usize],
        })
    }
}

#[repr(transparent)]
#[derive(Copy, Clone, Debug)]
pub struct RasterizerDesc(pub D3D12_RASTERIZER_DESC);

// defaults from d3dx12.h
impl Default for RasterizerDesc {
    fn default() -> Self {
        Self(D3D12_RASTERIZER_DESC {
            FillMode: FillMode::Solid as i32,
            CullMode: CullMode::Back as i32,
            FrontCounterClockwise: 0,
            DepthBias: D3D12_DEFAULT_DEPTH_BIAS as i32,
            DepthBiasClamp: D3D12_DEFAULT_DEPTH_BIAS_CLAMP as f32,
            SlopeScaledDepthBias: D3D12_DEFAULT_SLOPE_SCALED_DEPTH_BIAS as f32,
            DepthClipEnable: 1,
            MultisampleEnable: 0,
            AntialiasedLineEnable: 0,
            ForcedSampleCount: 0,
            ConservativeRaster: ConservativeRasterizationMode::Off as i32,
        })
    }
}

// Padding fields are zeroed in Default impl, so this should be okay
#[cfg(feature = "hash")]
impl std::hash::Hash for RasterizerDesc {
    fn hash<H: std::hash::Hasher>(&self, state: &mut H) {
        unsafe {
            let slice = std::slice::from_raw_parts(
                self as *const _ as *const u8,
                std::mem::size_of::<Self>(),
            );

            slice.hash(state);
        }
    }
}

#[cfg(feature = "eq")]
impl PartialEq for RasterizerDesc {
    fn eq(&self, other: &Self) -> bool {
        unsafe {
            let self_slice = std::slice::from_raw_parts(
                self as *const _ as *const u8,
                std::mem::size_of::<Self>(),
            );

            let other_slice = std::slice::from_raw_parts(
                other as *const _ as *const u8,
                std::mem::size_of::<Self>(),
            );

            self_slice == other_slice
        }
    }
}

impl Eq for RasterizerDesc {}

#[repr(i32)]
#[derive(Debug, Clone, Copy)]
#[cfg_attr(feature = "eq", derive(PartialEq, Eq))]
#[cfg_attr(feature = "hash", derive(Hash))]
pub enum ConservativeRasterizationMode {
    Off = D3D12_CONSERVATIVE_RASTERIZATION_MODE_D3D12_CONSERVATIVE_RASTERIZATION_MODE_OFF,
    On = D3D12_CONSERVATIVE_RASTERIZATION_MODE_D3D12_CONSERVATIVE_RASTERIZATION_MODE_ON,
}

#[derive(Hash, PartialOrd, Ord, PartialEq, Eq, Copy, Clone, Debug)]
#[repr(transparent)]
pub struct DepthStencilDesc(pub D3D12_DEPTH_STENCIL_DESC);

// defaults from d3dx12.h: less depth test with writes; no stencil
impl Default for DepthStencilDesc {
    fn default() -> Self {
        Self(D3D12_DEPTH_STENCIL_DESC {
            DepthEnable: 1,
            DepthWriteMask: DepthWriteMask::All as i32,
            DepthFunc: ComparisonFunc::Less as i32,
            StencilEnable: 0,
            StencilReadMask: D3D12_DEFAULT_STENCIL_READ_MASK as u8,
            StencilWriteMask: D3D12_DEFAULT_STENCIL_WRITE_MASK as u8,
            FrontFace: DepthStencilOpDesc::default().0,
            BackFace: DepthStencilOpDesc::default().0,
        })
    }
}

#[derive(Hash, PartialOrd, Ord, PartialEq, Eq, Copy, Clone, Debug)]
#[repr(transparent)]
pub struct DepthStencilOpDesc(pub D3D12_DEPTH_STENCILOP_DESC);

// defaults from d3dx12.h
impl Default for DepthStencilOpDesc {
    fn default() -> Self {
        Self(D3D12_DEPTH_STENCILOP_DESC {
            StencilFailOp: StencilOp::Keep as i32,
            StencilDepthFailOp: StencilOp::Keep as i32,
            StencilPassOp: StencilOp::Keep as i32,
            StencilFunc: ComparisonFunc::Always as i32,
        })
    }
}

#[repr(transparent)]
#[derive(Hash, PartialOrd, Ord, PartialEq, Eq, Debug)]
pub struct CachedPipelineState<'a>(
    pub D3D12_CACHED_PIPELINE_STATE,
    PhantomData<&'a [u8]>,
);

impl<'a> Default for CachedPipelineState<'a> {
    fn default() -> Self {
        Self(
            D3D12_CACHED_PIPELINE_STATE {
                pCachedBlob: std::ptr::null_mut(),
                CachedBlobSizeInBytes: 0,
            },
            PhantomData,
        )
    }
}