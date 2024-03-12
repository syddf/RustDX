use crate::scene::mesh::*;

pub struct MeshBatch<'a>
{
    pub mesh: &'a Mesh,
    pub mesh_index_in_gpu_scene: u32
}

pub trait SceneProxy
{
    fn generate_mesh_batch<'a>(&'a self) -> MeshBatch<'a>;
}