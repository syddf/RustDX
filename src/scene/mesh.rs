use std::collections::BTreeMap;
use lazy_static::lazy_static;

pub type MeshChannelData = BTreeMap<MeshDataChannel, Vec<f32>>;

#[derive(PartialEq, Eq, PartialOrd, Ord)]
pub enum MeshDataChannel
{
    Position = 0,
    Normal = 1,
    Tangent = 2,
    UV0 = 3,
    Color = 4
}

lazy_static! {
    static ref GLOBAL_DEFAULT_MESHDATA_VALUE: BTreeMap<MeshDataChannel, Vec<f32>> = {
        let mut map = BTreeMap::new();
        map.insert(MeshDataChannel::Position, vec![0.0, 0.0, 0.0]);
        map.insert(MeshDataChannel::Normal, vec![0.0, 0.0, 1.0]);
        map.insert(MeshDataChannel::Tangent, vec![0.0, 1.0, 0.0]);
        map.insert(MeshDataChannel::UV0, vec![0.0, 0.0]);
        map.insert(MeshDataChannel::Color, vec![0.0, 0.0, 0.0, 1.0]);
        map
    };
}

#[derive(Default)]
pub struct Mesh
{
    pub mesh_channel_data : MeshChannelData,
    pub mesh_index_data : Vec<u32>
}
