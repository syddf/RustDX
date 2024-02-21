#[no_mangle]
pub static D3D12SDKVersion: u32 = 606;

#[no_mangle]
pub static D3D12SDKPath: &[u8; 9] = b".\\D3D12\\\0";

use RustDX::*;
fn main() {
    
    let reflection = shader::get_shader_reflection(
        "PixelShader",
        r#"
        struct PSInput
        {
            float4 position : SV_POSITION;
            float2 uv : TEXCOORD;
        };

        Texture2D g_texture : register(t0);
        SamplerState g_sampler : register(s0);
       
        float4 PSMain(PSInput input) : SV_TARGET
        {
            return g_texture.Sample(g_sampler, input.uv);
        }
"#,
        "PSMain",
        "ps_6_0",
    );

    println!(
        "\tdesciptor_sets: {:?}",
        reflection.unwrap().get_descriptor_sets()
    );
}
