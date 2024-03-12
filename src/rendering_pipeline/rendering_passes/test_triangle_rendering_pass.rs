use crate::{mesh_draw_command::MeshDrawCommand, render_pass::RenderPass, scene::scene::Scene, scene_proxy::MeshBatch};

pub struct TestTriangleRenderingPass
{

}

impl RenderPass for TestTriangleRenderingPass
{
    fn generate_mesh_batch_from_scene(scene: &Scene) -> Vec<MeshBatch<'_>>
    {
        let scene_proxies = scene.get_scene_proxies();
        let mut mesh_batches = vec![];
        for scene_proxy in scene_proxies
        {
            mesh_batches.push(scene_proxy.generate_mesh_batch());
        }
        mesh_batches
    }

    fn cache_mesh_draw_commands(mesh_batches: Vec<MeshBatch<'_>>) -> Vec<MeshDrawCommand<'_>>
    {
        let mut draw_commands = vec![];
        for mesh_batch in &mesh_batches
        {
            let mesh_draw_command = MeshDrawCommand
            {
                mesh: mesh_batch.mesh, 
                mesh_index_in_gpu_scene: mesh_batch.mesh_index_in_gpu_scene
            };
            draw_commands.push(mesh_draw_command)
        }
        draw_commands
    }
}