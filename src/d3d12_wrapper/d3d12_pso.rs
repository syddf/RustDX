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

#[repr(transparent)]
#[derive(Debug)]
pub struct GraphicsPipelineStateDesc<'rs, 'sh, 'so, 'il>(
    pub D3D12_GRAPHICS_PIPELINE_STATE_DESC,
    PhantomData<&'rs RootSignature>,
    PhantomData<&'sh ShaderBytecode<'sh>>,
    PhantomData<&'so StreamOutputDesc<'so>>,
    PhantomData<&'il InputLayoutDesc<'il>>,
);


#[derive(Debug)]
#[repr(transparent)]
pub struct RootSignature {
    pub this: *mut ID3D12RootSignature,
}

impl_com_object_set_get_name!(RootSignature);
impl_com_object_refcount_named!(RootSignature);
impl_com_object_clone_drop!(RootSignature);

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
    pub(crate) D3D12_VERSIONED_ROOT_SIGNATURE_DESC,
);

impl VersionedRootSignatureDesc {
    // RS v1.0 is not supported
    // pub fn set_desc_1_0(self, _desc_1_0: &RootSignatureDesc) -> Self {
    //     unimplemented!();
    // }

    pub fn set_desc_1_1(&mut self, desc_1_1: &RootSignatureDesc) -> &mut Self {
        self.0.Version =
            D3D_ROOT_SIGNATURE_VERSION_D3D_ROOT_SIGNATURE_VERSION_1_1;
        self.0.__bindgen_anon_1.Desc_1_1 = desc_1_1.0;
        self
    }

    pub fn with_desc_1_1(mut self, desc_1_1: &RootSignatureDesc) -> Self {
        self.set_desc_1_1(desc_1_1);
        self
    }

    pub fn desc_1_1(&self) -> RootSignatureDesc {
        unsafe {
            RootSignatureDesc(
                self.0.__bindgen_anon_1.Desc_1_1,
                PhantomData,
                PhantomData,
            )
        }
    }
}

/// Wrapper around D3D12_ROOT_SIGNATURE_DESC1 structure
#[derive(Hash, PartialOrd, Ord, PartialEq, Eq, Default, Debug)]
#[repr(transparent)]
pub struct RootSignatureDesc<'a, 'b>(
    pub D3D12_ROOT_SIGNATURE_DESC1,
    PhantomData<&'a RootParameter<'a>>,
    PhantomData<&'b StaticSamplerDesc>,
);

impl<'a, 'b> RootSignatureDesc<'a, 'b> {
    pub fn set_parameters(
        &mut self,
        parameters: &'a [RootParameter],
    ) -> &mut Self {
        self.0.NumParameters = parameters.len() as u32;
        self.0.pParameters =
            parameters.as_ptr() as *const D3D12_ROOT_PARAMETER1;
        self.1 = PhantomData;
        self
    }

    pub fn with_parameters(mut self, parameters: &'a [RootParameter]) -> Self {
        self.set_parameters(parameters);
        self
    }

    pub fn parameters(&self) -> &'a [RootParameter] {
        unsafe {
            slice::from_raw_parts(
                self.0.pParameters as *const D3D12_ROOT_PARAMETER1
                    as *const RootParameter,
                self.0.NumParameters as usize,
            )
        }
    }

    pub fn set_static_samplers(
        &mut self,
        static_samplers: &'b [StaticSamplerDesc],
    ) -> &mut Self {
        self.0.NumStaticSamplers = static_samplers.len() as u32;
        self.0.pStaticSamplers =
            static_samplers.as_ptr() as *const D3D12_STATIC_SAMPLER_DESC;
        self.2 = PhantomData;
        self
    }

    pub fn with_static_samplers(
        mut self,
        static_samplers: &'b [StaticSamplerDesc],
    ) -> Self {
        self.set_static_samplers(static_samplers);
        self
    }

    pub fn static_samplers(&self) -> &'a [StaticSamplerDesc] {
        unsafe {
            slice::from_raw_parts(
                self.0.pStaticSamplers as *const D3D12_STATIC_SAMPLER_DESC
                    as *const StaticSamplerDesc,
                self.0.NumStaticSamplers as usize,
            )
        }
    }

    pub fn set_flags(&mut self, flags: RootSignatureFlags) -> &mut Self {
        self.0.Flags = flags.bits();
        self
    }

    pub fn with_flags(mut self, flags: RootSignatureFlags) -> Self {
        self.set_flags(flags);
        self
    }

    pub fn flags(&self) -> RootSignatureFlags {
        unsafe { RootSignatureFlags::from_bits_unchecked(self.0.Flags) }
    }
}

/// Wrapper around D3D12_ROOT_PARAMETER1 structure
#[derive(Debug, Default)]
#[repr(transparent)]
pub struct RootParameter<'a>(
    pub(crate) D3D12_ROOT_PARAMETER1,
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

/// Wrapper around D3D12_ROOT_DESCRIPTOR_TABLE1 structure
#[derive(Hash, PartialOrd, Ord, PartialEq, Eq, Default, Debug)]
#[repr(transparent)]
pub struct RootDescriptorTable<'a>(
    pub D3D12_ROOT_DESCRIPTOR_TABLE1,
    PhantomData<&'a DescriptorRange>,
);

impl<'a> RootDescriptorTable<'a> {
    pub fn set_descriptor_ranges(
        &mut self,
        ranges: &'a [DescriptorRange],
    ) -> &mut Self {
        self.0.NumDescriptorRanges = ranges.len() as u32;
        self.0.pDescriptorRanges =
            ranges.as_ptr() as *const D3D12_DESCRIPTOR_RANGE1;
        self.1 = PhantomData;
        self
    }

    pub fn with_descriptor_ranges(
        mut self,
        ranges: &'a [DescriptorRange],
    ) -> Self {
        self.set_descriptor_ranges(ranges);
        self
    }

    pub fn descriptor_ranges(&self) -> &'a [DescriptorRange] {
        unsafe {
            std::slice::from_raw_parts(
                self.0.pDescriptorRanges as *const D3D12_DESCRIPTOR_RANGE1
                    as *const DescriptorRange,
                self.0.NumDescriptorRanges as usize,
            )
        }
    }
}

/// Wrapper around D3D12_SHADER_BYTECODE structure
#[repr(transparent)]
#[derive(Hash, PartialOrd, Ord, PartialEq, Eq, Debug)]
pub struct ShaderBytecode<'a>(
    pub(crate) D3D12_SHADER_BYTECODE,
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

impl<'a> SoDeclarationEntry<'a> {
    pub fn set_stream(&mut self, stream: u32) -> &mut Self {
        self.0.Stream = stream;
        self
    }

    pub fn with_stream(mut self, stream: u32) -> Self {
        self.set_stream(stream);
        self
    }

    pub fn stream(&self) -> u32 {
        self.0.Stream
    }

    pub fn set_semantic_name(
        &mut self,
        name: &'a str,
    ) -> Result<&mut Self, NulError> {
        let owned = CString::new(name)?;
        self.0.SemanticName = owned.into_raw() as *const i8;
        self.1 = PhantomData;
        Ok(self)
    }

    pub fn with_semantic_name(
        mut self,
        name: &'a str,
    ) -> Result<Self, NulError> {
        match self.set_semantic_name(name) {
            Ok(_) => Ok(self),
            Err(err) => Err(err),
        }
    }

    pub fn semantic_name(&self) -> Result<&'a str, Utf8Error> {
        Ok(unsafe { std::ffi::CStr::from_ptr(self.0.SemanticName).to_str()? })
    }

    pub fn set_semantic_index(&mut self, semantic_index: u32) -> &mut Self {
        self.0.SemanticIndex = semantic_index;
        self
    }

    pub fn with_semantic_index(mut self, semantic_index: u32) -> Self {
        self.set_semantic_index(semantic_index);
        self
    }

    pub fn semantic_index(&self) -> u32 {
        self.0.SemanticIndex
    }

    pub fn set_start_component(&mut self, start_component: u8) -> &mut Self {
        self.0.StartComponent = start_component;
        self
    }

    pub fn with_start_component(mut self, start_component: u8) -> Self {
        self.set_start_component(start_component);
        self
    }

    pub fn start_component(&self) -> u8 {
        self.0.StartComponent
    }

    pub fn set_component_count(&mut self, component_count: u8) -> &mut Self {
        self.0.ComponentCount = component_count;
        self
    }

    pub fn with_component_count(mut self, component_count: u8) -> Self {
        self.set_component_count(component_count);
        self
    }

    pub fn component_count(&self) -> u8 {
        self.0.ComponentCount
    }

    pub fn set_output_slot(&mut self, output_slot: u8) -> &mut Self {
        self.0.OutputSlot = output_slot;
        self
    }

    pub fn with_output_slot(mut self, output_slot: u8) -> Self {
        self.set_output_slot(output_slot);
        self
    }

    pub fn output_slot(&self) -> u8 {
        self.0.OutputSlot
    }
}

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

impl<'a> StreamOutputDesc<'a> {
    pub fn set_so_declarations(
        &mut self,
        so_declarations: &'a [SoDeclarationEntry],
    ) -> &mut StreamOutputDesc<'a> {
        self.0.pSODeclaration =
            so_declarations.as_ptr() as *const D3D12_SO_DECLARATION_ENTRY;
        self.0.NumEntries = so_declarations.len() as u32;
        self.1 = PhantomData;
        self
    }

    pub fn with_so_declarations(
        mut self,
        so_declarations: &'a [SoDeclarationEntry],
    ) -> Self {
        self.set_so_declarations(so_declarations);
        self
    }

    pub fn so_declarations(&self) -> &'a [SoDeclarationEntry] {
        unsafe {
            slice::from_raw_parts(
                self.0.pSODeclaration as *const SoDeclarationEntry,
                self.0.NumEntries as usize,
            )
        }
    }

    // Note there are no setters since they are both useless and can break the invariant
    pub fn num_entries(&self) -> u32 {
        self.0.NumEntries
    }

    pub fn set_buffer_strides(&mut self, buffer_strides: &[u32]) -> &mut Self {
        self.0.pBufferStrides = buffer_strides.as_ptr();
        self.0.NumStrides = buffer_strides.len() as u32;
        self.1 = PhantomData;
        self
    }

    pub fn with_buffer_strides(mut self, buffer_strides: &[u32]) -> Self {
        self.set_buffer_strides(buffer_strides);
        self
    }

    pub fn buffer_strides(&self) -> &'a [u32] {
        unsafe {
            slice::from_raw_parts(
                self.0.pBufferStrides as *const u32,
                self.0.NumStrides as usize,
            )
        }
    }

    // Note there are no setters since they are both useless and can break the invariant
    pub fn num_strides(&self) -> u32 {
        self.0.NumStrides
    }

    pub fn set_rasterized_stream(
        &mut self,
        rasterized_stream: u32,
    ) -> &mut Self {
        self.0.RasterizedStream = rasterized_stream;
        self
    }

    pub fn with_rasterized_stream(mut self, rasterized_stream: u32) -> Self {
        self.set_rasterized_stream(rasterized_stream);
        self
    }

    pub fn rasterized_stream(&self) -> u32 {
        self.0.RasterizedStream
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

// ToDo: ShaderBytecode is a similar struct, but it uses new() method
impl<'a> InputLayoutDesc<'a> {
    pub fn set_input_elements(
        &mut self,
        layout: &'a [InputElementDesc<'a>],
    ) -> &mut Self {
        self.0.pInputElementDescs =
            layout.as_ptr() as *const D3D12_INPUT_ELEMENT_DESC;
        self.0.NumElements = layout.len() as u32;
        self.1 = PhantomData;
        self
    }

    pub fn with_input_elements(
        mut self,
        layout: &'a [InputElementDesc<'a>],
    ) -> Self {
        self.set_input_elements(layout);
        self
    }

    pub fn input_elements(&self) -> &'a [InputElementDesc] {
        unsafe {
            slice::from_raw_parts(
                self.0.pInputElementDescs as *const InputElementDesc,
                self.0.NumElements as usize,
            )
        }
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

// ToDo: macro for generating input element desc from vertex struct type?

impl<'a> InputElementDesc<'a> {
    pub fn set_semantic_name(
        &mut self,
        name: &'a str,
    ) -> Result<&mut Self, NulError> {
        let owned = CString::new(name)?;
        self.0.SemanticName = owned.into_raw() as *const i8;
        self.1 = PhantomData;
        Ok(self)
    }

    pub fn with_semantic_name(
        mut self,
        name: &'a str,
    ) -> Result<Self, NulError> {
        match self.set_semantic_name(name) {
            Ok(_) => Ok(self),
            Err(err) => Err(err),
        }
    }

    pub fn semantic_name(&self) -> Result<&'a str, Utf8Error> {
        Ok(unsafe { std::ffi::CStr::from_ptr(self.0.SemanticName).to_str()? })
    }

    pub fn set_semantic_index(&mut self, semantic_index: u32) -> &mut Self {
        self.0.SemanticIndex = semantic_index;
        self
    }

    pub fn with_semantic_index(mut self, semantic_index: u32) -> Self {
        self.set_semantic_index(semantic_index);
        self
    }

    pub fn semantic_index(&self) -> u32 {
        self.0.SemanticIndex
    }

    pub fn set_format(&mut self, format: Format) -> &mut Self {
        self.0.Format = format as i32;
        self
    }

    pub fn with_format(mut self, format: Format) -> Self {
        self.set_format(format);
        self
    }

    pub fn format(&self) -> Format {
        unsafe { std::mem::transmute(self.0.Format) }
    }

    pub fn set_input_slot(&mut self, input_slot: u32) -> &mut Self {
        self.0.InputSlot = input_slot;
        self
    }

    pub fn with_input_slot(mut self, input_slot: u32) -> Self {
        self.set_input_slot(input_slot);
        self
    }

    pub fn input_slot(&self) -> u32 {
        self.0.InputSlot
    }

    pub fn set_aligned_byte_offset(
        &mut self,
        aligned_byte_offset: ByteCount,
    ) -> &mut Self {
        self.0.AlignedByteOffset = aligned_byte_offset.0 as u32;
        self
    }

    pub fn with_aligned_byte_offset(
        mut self,
        aligned_byte_offset: ByteCount,
    ) -> Self {
        self.set_aligned_byte_offset(aligned_byte_offset);
        self
    }

    pub fn aligned_byte_offset(&self) -> ByteCount {
        ByteCount::from(self.0.AlignedByteOffset)
    }

    pub fn set_input_slot_class(
        &mut self,
        input_slot_class: InputClassification,
    ) -> &mut Self {
        self.0.InputSlotClass = input_slot_class as i32;
        self
    }

    pub fn with_input_slot_class(
        mut self,
        input_slot_class: InputClassification,
    ) -> Self {
        self.set_input_slot_class(input_slot_class);
        self
    }

    pub fn input_slot_class(&self) -> InputClassification {
        unsafe { std::mem::transmute(self.0.InputSlotClass) }
    }

    pub fn set_instance_data_step_rate(
        &mut self,
        instance_data_step_rate: u32,
    ) -> &mut Self {
        self.0.InstanceDataStepRate = instance_data_step_rate;
        self
    }

    pub fn with_instance_data_step_rate(
        mut self,
        instance_data_step_rate: u32,
    ) -> Self {
        self.set_instance_data_step_rate(instance_data_step_rate);
        self
    }

    pub fn instance_data_step_rate(&self) -> u32 {
        self.0.InstanceDataStepRate
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
pub struct RootConstants(pub(crate) D3D12_ROOT_CONSTANTS);

impl RootConstants {
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

    pub fn set_num_32_bit_values(
        &mut self,
        num_32_bit_values: u32,
    ) -> &mut Self {
        self.0.Num32BitValues = num_32_bit_values;
        self
    }

    pub fn with_num_32_bit_values(mut self, num_32_bit_values: u32) -> Self {
        self.set_num_32_bit_values(num_32_bit_values);
        self
    }

    pub fn num_32_bit_values(&self) -> u32 {
        self.0.Num32BitValues
    }
}

/// Wrapper around D3D12_ROOT_DESCRIPTOR1 structure
#[derive(Hash, PartialOrd, Ord, PartialEq, Eq, Copy, Clone, Default, Debug)]
#[repr(transparent)]
pub struct RootDescriptor(pub(crate) D3D12_ROOT_DESCRIPTOR1);

impl RootDescriptor {
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

    pub fn set_flags(&mut self, flags: RootDescriptorFlags) -> &mut Self {
        self.0.Flags = flags.bits();
        self
    }

    pub fn with_flags(mut self, flags: RootDescriptorFlags) -> Self {
        self.set_flags(flags);
        self
    }

    pub fn flags(&self) -> RootDescriptorFlags {
        unsafe { RootDescriptorFlags::from_bits_unchecked(self.0.Flags) }
    }
}

/// Newtype around [u32] since it has a special value of [DESCRIPTOR_RANGE_OFFSET_APPEND]
#[derive(Hash, PartialOrd, Ord, PartialEq, Eq, Copy, Clone, Debug)]
pub struct DescriptorRangeOffset(pub(crate) u32);

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
pub struct DescriptorRange(pub(crate) D3D12_DESCRIPTOR_RANGE1);

impl DescriptorRange {
    pub fn set_range_type(
        &mut self,
        range_type: DescriptorRangeType,
    ) -> &mut Self {
        self.0.RangeType = range_type as i32;
        self
    }

    pub fn with_range_type(mut self, range_type: DescriptorRangeType) -> Self {
        self.set_range_type(range_type);
        self
    }

    pub fn range_type(&self) -> DescriptorRangeType {
        unsafe { std::mem::transmute(self.0.RangeType) }
    }

    pub fn set_num_descriptors(&mut self, num_descriptors: u32) -> &mut Self {
        self.0.NumDescriptors = num_descriptors;
        self
    }

    pub fn with_num_descriptors(mut self, num_descriptors: u32) -> Self {
        self.set_num_descriptors(num_descriptors);
        self
    }

    pub fn num_descriptors(&self) -> u32 {
        self.0.NumDescriptors
    }

    pub fn set_base_shader_register(
        &mut self,
        base_shader_register: u32,
    ) -> &mut Self {
        self.0.BaseShaderRegister = base_shader_register;
        self
    }

    pub fn with_base_shader_register(
        mut self,
        base_shader_register: u32,
    ) -> Self {
        self.set_base_shader_register(base_shader_register);
        self
    }

    pub fn base_shader_register(&self) -> u32 {
        self.0.BaseShaderRegister
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

    pub fn set_flags(&mut self, flags: DescriptorRangeFlags) -> &mut Self {
        self.0.Flags = flags.bits();
        self
    }

    pub fn with_flags(mut self, flags: DescriptorRangeFlags) -> Self {
        self.set_flags(flags);
        self
    }

    pub fn flags(&self) -> DescriptorRangeFlags {
        unsafe { DescriptorRangeFlags::from_bits_unchecked(self.0.Flags) }
    }

    pub fn set_offset_in_descriptors_from_table_start(
        &mut self,
        offset_in_descriptors_from_table_start: DescriptorRangeOffset,
    ) -> &mut Self {
        self.0.OffsetInDescriptorsFromTableStart =
            offset_in_descriptors_from_table_start.0;
        self
    }

    pub fn with_offset_in_descriptors_from_table_start(
        mut self,
        offset_in_descriptors_from_table_start: DescriptorRangeOffset,
    ) -> Self {
        self.set_offset_in_descriptors_from_table_start(
            offset_in_descriptors_from_table_start,
        );
        self
    }

    pub fn offset_in_descriptors_from_table_start(
        &self,
    ) -> DescriptorRangeOffset {
        self.0.OffsetInDescriptorsFromTableStart.into()
    }
}
