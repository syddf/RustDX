struct PSInput
{
    float4 position : SV_POSITION;
    float2 uv : TEXCOORD;
};

Texture3D g_texture : register(t0);
SamplerState g_sampler : register(s0);
       
float4 PSMain(PSInput input) : SV_TARGET
{
    return g_texture.Sample(g_sampler, float3(input.uv, 0.0f));
}