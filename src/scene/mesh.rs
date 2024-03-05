use std::collections::BTreeMap;

pub type MeshChannelData = BTreeMap<MeshDataChannel, Vec<f32>>;

#[derive(PartialEq, Eq, PartialOrd, Ord)]
pub enum MeshDataChannel
{
    Position = 0,
    Color = 1
}

#[derive(Default)]
pub struct Mesh
{
    pub mesh_channel_data : MeshChannelData,
    pub mesh_index_data : Vec<u32>
}
