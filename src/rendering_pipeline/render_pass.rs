use crate::scene_proxy::MeshBatch;
use crate::scene::scene::{Scene};
use crate::mesh_draw_command::{MeshDrawCommand};

pub trait RenderPass
{
    fn generate_mesh_batch_from_scene(scene: &Scene) -> Vec<MeshBatch<'_>>;
    fn cache_mesh_draw_commands(mesh_batches: Vec<MeshBatch<'_>>) -> Vec<MeshDrawCommand<'_>>;
}