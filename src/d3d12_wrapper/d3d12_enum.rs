#![allow(non_upper_case_globals)]
#![allow(non_camel_case_types)]
#![allow(non_snake_case)]

use crate::raw_bindings::d3d12::*;
use bitflags::bitflags;

bitflags! {
    pub struct FenceFlags: i32 {
        const None = D3D12_FENCE_FLAGS_D3D12_FENCE_FLAG_NONE;
        const Shared =
        D3D12_FENCE_FLAGS_D3D12_FENCE_FLAG_SHARED;
        const CrossAdapter =
        D3D12_FENCE_FLAGS_D3D12_FENCE_FLAG_SHARED_CROSS_ADAPTER;
        const NonMonitored =
        D3D12_FENCE_FLAGS_D3D12_FENCE_FLAG_NON_MONITORED;
    }
}