use crate::scene::mesh::*;
use crate::scene::scene_proxy::*;

pub struct MeshDrawCommand<'a>
{
    pub mesh: &'a Mesh,
    pub mesh_index_in_gpu_scene: u32
}