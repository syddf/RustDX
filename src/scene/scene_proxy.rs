use crate::scene::mesh::*;

pub struct MeshBatch<'a>
{
    pub mesh: &'a Mesh
}

pub trait SceneProxy
{
    fn generate_mesh_batch<'a>(&'a self) -> MeshBatch<'a>;
}