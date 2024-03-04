struct VertexIn
{
#if VERTEX_FACTORY_USE_POSITION
    float3 pos: Position;
#endif
#if VERTEX_FACTORY_USE_COLOR
    float4 color: Color;
#endif
};