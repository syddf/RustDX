use crate::d3d12_buffer::IndexBufferView;
use crate::d3d12_buffer::VertexBufferView;
use crate::d3d12_common::ByteCount;
use crate::d3d12_enum::Format;
use crate::d3d12_enum::ResourceDimension;
use crate::d3d12_enum::ResourceStates;
use crate::d3d12_texture::TextureLayout;
use crate::d3d12_window::*;
use crate::scene::scene_proxy::*;
use crate::scene::mesh::*;
use crate::d3d12_resource::*;
use crate::D3D12_HEAP_PROPERTIES;
use crate::d3d12_wrapper::d3d12_device::*;
use crate::d3d12_wrapper::d3d12_command::*;

#[derive(Default)]
pub struct StaticMesh
{
    name: &'static str,
    mesh: Mesh,

    vertex_buffer_resource: Resource,
    index_buffer_resource: Resource,
    vertex_buffer_view: VertexBufferView,
    index_buffer_view: IndexBufferView
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
        let channel_val = channel as usize;
        if self.mesh.mesh_channel_data.contains_key(&channel_val)
        {
            self.mesh.mesh_channel_data.get_mut(&channel_val).unwrap().append(&mut data);
        }
        else 
        {
            self.mesh.mesh_channel_data.insert(channel_val, data);
        }
    }

    pub fn set_index_buffer(&mut self, index_buffer: Vec<u32>)
    {
        self.mesh.mesh_index_data = index_buffer;
    }

    pub fn generate_gpu_resource(&mut self, g_device: &Device)
    {
        let copy_comand_list = G_COPY_COMMAND_LIST.lock().unwrap();
        let (vertex_data, vertex_size) = self.mesh.get_vertex_buffer_data();
        
        let vertex_buffer_size = ByteCount::from(
            vertex_data.len() * std::mem::size_of::<f32>(),
        );

        let vertex_staging_buffer = g_device.create_staging_buffer(vertex_buffer_size).expect("Cannot create staging buffer");
        let data = vertex_staging_buffer
            .map(0, None)
            .expect("Cannot map staging buffer");

        unsafe {
            std::ptr::copy_nonoverlapping(
                vertex_data.as_ptr() as *const u8,
                data,
                vertex_buffer_size.0 as usize,
            );
        }
        vertex_staging_buffer.unmap(0, None);

        let vertex_default_buffer = g_device.create_default_buffer(vertex_buffer_size).expect("Cannot create default buffer");
        copy_comand_list.add_resource_barrier(&vertex_default_buffer, ResourceStates::Common, ResourceStates::CopyDest);

        copy_comand_list.copy_buffer_region(
            &vertex_default_buffer,
            ByteCount(0),
            &vertex_staging_buffer,
            ByteCount(0),
            vertex_buffer_size,
        );
        copy_comand_list.add_resource_barrier(&vertex_default_buffer, ResourceStates::CopyDest, ResourceStates::Common);

        self.vertex_buffer_view = VertexBufferView::default();
        self.vertex_buffer_view.0.BufferLocation = vertex_default_buffer.get_gpu_virtual_address().0;
        self.vertex_buffer_view.0.SizeInBytes = vertex_buffer_size.0 as u32;
        self.vertex_buffer_view.0.StrideInBytes = vertex_buffer_size.0 as u32 / vertex_size as u32;
        self.vertex_buffer_resource = vertex_default_buffer;

        let index_buffer_data_32 = self.mesh.mesh_index_data.clone();
        let index_buffer_data_16 = self.mesh.get_index_buffer_data_u16();
        let mut index_buffer_size = ByteCount::from(index_buffer_data_32.len() * std::mem::size_of::<u32>());
        let mut index_buffer_stride = 4;
        if (vertex_size as u32) < (u16::max_value() as u32)
        {
            index_buffer_size = ByteCount::from(
                index_buffer_data_16.len() * std::mem::size_of::<u16>(),
            );

            index_buffer_stride = 2;
        }
        
        let index_staging_buffer = g_device.create_staging_buffer(index_buffer_size).expect("Cannot create staging buffer");
        let data = index_staging_buffer
            .map(0, None)
            .expect("Cannot map staging buffer");
        if (vertex_size as u32) < (u16::max_value() as u32)
        {
            unsafe {
                std::ptr::copy_nonoverlapping(
                    index_buffer_data_16.as_ptr() as *const u8,
                    data,
                    vertex_buffer_size.0 as usize,
                );
            }
        }
        else
        {
            unsafe {
                std::ptr::copy_nonoverlapping(
                    index_buffer_data_32.as_ptr() as *const u8,
                    data,
                    vertex_buffer_size.0 as usize,
                );
            }    
        }
        index_staging_buffer.unmap(0, None);

    
        let index_default_buffer = g_device.create_default_buffer(index_buffer_size).expect("Cannot create default buffer");
        copy_comand_list.add_resource_barrier(&index_default_buffer, ResourceStates::Common, ResourceStates::CopyDest);

        copy_comand_list.copy_buffer_region(
            &index_default_buffer,
            ByteCount(0),
            &index_staging_buffer,
            ByteCount(0),
            index_buffer_size,
        );
        copy_comand_list.add_resource_barrier(&index_default_buffer, ResourceStates::CopyDest, ResourceStates::Common);

        self.index_buffer_view = IndexBufferView::default();
        self.index_buffer_view.0.BufferLocation = index_default_buffer.get_gpu_virtual_address().0;
        self.index_buffer_view.0.SizeInBytes = index_buffer_size.0 as u32;
        self.index_buffer_view.0.Format = Format::R32Uint as i32;
        if (vertex_size as u32) < (u16::max_value() as u32)
        {
            self.index_buffer_view.0.Format = Format::R16Uint as i32;
        }
        self.index_buffer_resource = index_default_buffer;


        copy_comand_list.close().expect("Cannot close command list");
        let command_queue = G_COPY_COMMAND_QUEUE.lock().unwrap();
        command_queue
            .execute_command_lists(std::slice::from_ref(&copy_comand_list));

    }

    pub fn get_gpu_resource(&self) -> (&Resource, &Resource, &VertexBufferView, &IndexBufferView)
    {
        (&self.vertex_buffer_resource, &self.index_buffer_resource, &self.vertex_buffer_view, &self.index_buffer_view)
    }
}

impl SceneProxy for StaticMesh
{
    fn generate_mesh_batch<'a>(&'a self) -> MeshBatch<'a>
    {
        MeshBatch{mesh: &self.mesh}
    }
}