#[no_mangle]
pub static D3D12SDKVersion: u32 = 606;

#[no_mangle]
pub static D3D12SDKPath: &[u8; 9] = b".\\D3D12\\\0";

use RustDX::*;

fn main() {
    let mut shader_manager = shader::ShaderManager::default();
    shader_manager.update_all_shader();
    shader_manager.load_all_shader();
}
