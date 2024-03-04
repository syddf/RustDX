#include "common/VertexFactory.hlsl"

struct VertexOut
{
    float4 pos: SV_POSITION;
    float4 color: Color;
};

VertexOut VSMain(VertexIn input)
{
    VertexOut result = (VertexOut)0;
    result.pos = float4(input.pos, 1.);
    result.color = input.color;

    return result;
}