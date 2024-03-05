use crate::scene::scene_proxy::*;
use crate::scene::mesh::*;

#[derive(Default)]
pub struct StaticMesh
{
    name: &'static str,
    mesh: Mesh
}

impl StaticMesh
{
    pub fn new(name: &'static str) -> Self
    {
        let mut static_mesh = StaticMesh::default();
        static_mesh.name = name;
        static_mesh        
    }

    pub fn add_channel_data(&mut self, channel: MeshDataChannel, mut data: Vec<f32>)
    {
        if self.mesh.mesh_channel_data.contains_key(&channel)
        {
            self.mesh.mesh_channel_data.get_mut(&channel).unwrap().append(&mut data);
        }
        else 
        {
            self.mesh.mesh_channel_data.insert(channel, data);
        }
    }

    pub fn set_index_buffer(&mut self, index_buffer: Vec<u32>)
    {
        self.mesh.mesh_index_data = index_buffer;
    }
}

impl SceneProxy for StaticMesh
{
    fn generate_mesh_batch<'a>(&'a self) -> MeshBatch<'a>
    {
        MeshBatch{mesh: &self.mesh}
    }
}