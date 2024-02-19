#[no_mangle]
pub static D3D12SDKVersion: u32 = 606;

#[no_mangle]
pub static D3D12SDKPath: &[u8; 9] = b".\\D3D12\\\0";

use RustDX::*;

fn main() {
    println!("Hello, world!");
}
