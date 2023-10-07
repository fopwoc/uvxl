use bytemuck::{Pod, Zeroable};
use glam::{Mat4, Vec3};
use crate::graphics::instance::Instance;

unsafe impl Zeroable for BakedChunkModel { }
unsafe impl Pod for BakedChunkModel { }

#[derive(Copy, Clone)]
pub struct ChunkModel {
  pub position: Vec3,
}

#[derive(Copy, Clone)]
pub struct BakedChunkModel {
  pub model: Mat4,
}

impl Instance for ChunkModel {
  type Baked = BakedChunkModel;

  fn bake(&self) -> Self::Baked {
    return BakedChunkModel {
      model: Mat4::from_translation(self.position),
    }
  }

  fn describe<'a>() -> wgpu::VertexBufferLayout<'a> {
    use std::mem;
    wgpu::VertexBufferLayout {
      array_stride: mem::size_of::<Self::Baked>() as wgpu::BufferAddress,
      step_mode: wgpu::VertexStepMode::Instance,
      attributes: &[
        wgpu::VertexAttribute {
          offset: 0,
          shader_location: 5,
          format: wgpu::VertexFormat::Float32x4,
        },
        wgpu::VertexAttribute {
          offset: mem::size_of::<[f32; 4]>() as wgpu::BufferAddress,
          shader_location: 6,
          format: wgpu::VertexFormat::Float32x4,
        },
        wgpu::VertexAttribute {
          offset: mem::size_of::<[f32; 8]>() as wgpu::BufferAddress,
          shader_location: 7,
          format: wgpu::VertexFormat::Float32x4,
        },
        wgpu::VertexAttribute {
          offset: mem::size_of::<[f32; 12]>() as wgpu::BufferAddress,
          shader_location: 8,
          format: wgpu::VertexFormat::Float32x4,
        },
      ],
    }
  }
}