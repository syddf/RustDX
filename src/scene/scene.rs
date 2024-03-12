use crate::scene::mesh::*;
use crate::scene::scene_proxy::*;

pub struct Scene
{
    scene_proxies: Vec<Box<dyn SceneProxy>>
}

impl Scene
{
    pub fn add_scene_proxy(&mut self, in_proxy: Box<dyn SceneProxy>)
    {
        self.scene_proxies.push(in_proxy);
    }

    pub fn get_scene_proxies(&self) -> &Vec<Box<dyn SceneProxy>>
    {
        &self.scene_proxies
    }
}