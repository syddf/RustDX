use std::{collections::BTreeMap, sync::mpsc::channel};
use lazy_static::lazy_static;
use log::debug;

pub type MeshChannelData = BTreeMap<usize, Vec<f32>>;

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
    static ref GLOBAL_DEFAULT_MESHDATA_VALUE: BTreeMap<usize, Vec<f32>> = {
        let mut map = BTreeMap::new();
        map.insert(MeshDataChannel::Position as usize, vec![0.0, 0.0, 0.0]);
        map.insert(MeshDataChannel::Normal as usize, vec![0.0, 0.0, 1.0]);
        map.insert(MeshDataChannel::Tangent as usize, vec![0.0, 1.0, 0.0]);
        map.insert(MeshDataChannel::UV0 as usize, vec![0.0, 0.0]);
        map.insert(MeshDataChannel::Color as usize, vec![0.0, 0.0, 0.0, 1.0]);
        map
    };
}

#[derive(Default)]
pub struct Mesh
{
    pub mesh_channel_data : MeshChannelData,
    pub mesh_index_data : Vec<u32>


}

impl Mesh
{
    pub fn get_vertex_buffer_data(&self) -> (Vec<f32>, i32)
    {
        let mut data = vec![];

        let mut vertex_count = -1;

        for i in 0..=MeshDataChannel::Color as usize
        {
            if self.mesh_channel_data.contains_key(&i)
            {
                let cur_vertex_count: i32 = self.mesh_channel_data.get(&i).unwrap().len() as i32 / GLOBAL_DEFAULT_MESHDATA_VALUE.get(&i).unwrap().len() as i32;
                if vertex_count == -1
                {
                    vertex_count = cur_vertex_count;
                }
                else 
                {
                    if vertex_count != cur_vertex_count
                    {
                        debug!("vertex data error: vertex count is not same.");
                        return (data, -1);
                    }
                }
            }
        }
        for vertex_ind in 0..vertex_count
        {
            for i in 0..=MeshDataChannel::Color as usize
            {
                if self.mesh_channel_data.contains_key(&i)
                {
                    let channel_size = GLOBAL_DEFAULT_MESHDATA_VALUE.get(&i).unwrap().len();
                    for data_ind in 0..channel_size
                    {
                        data.push(self.mesh_channel_data.get(&i).unwrap()[vertex_ind as usize * channel_size + data_ind]);
                    }
                }
                else 
                {
                    data.extend(GLOBAL_DEFAULT_MESHDATA_VALUE.get(&i).unwrap());
                }
            }
        }


        (data, vertex_count)
    }

    pub fn get_index_buffer_data_u16(&self) -> Vec<u16>
    {
        let mut data = vec![];

        for i in 0..self.mesh_index_data.len()
        {
            data.push(self.mesh_index_data[i] as u16);
        }

        data
    }
}

pub fn get_vertex_attribute_offset(channel: u32) -> u32
{
    let mut res = 0;
    if channel == 0
    {
        return 0;
    }
    for i in 0..(channel - 1) as usize 
    {
        let len = GLOBAL_DEFAULT_MESHDATA_VALUE.get(&i).unwrap().len();
        res +=  len as u32 * 4;
    }
    res
}