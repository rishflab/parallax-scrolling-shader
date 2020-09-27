use bytemuck::{Pod, Zeroable};
use gltf::{buffer::Source, mesh::util::ReadIndices, Gltf};
use std::{fs, io, ops::Range, path::Path};
use thiserror::Error;

/// An error that occurs when loading a GLTF file
#[derive(Error, Debug)]
pub enum GltfError {
    #[error("Invalid GLTF file.")]
    Gltf(#[from] gltf::Error),
    #[error("Failed to load file.")]
    Io(#[from] io::Error),
    #[error("Binary blob is missing.")]
    MissingBlob,
    #[error("Failed to decode base64 mesh data.")]
    Base64Decode(#[from] base64::DecodeError),
    #[error("Unsupported buffer format.")]
    BufferFormatUnsupported,
}

#[repr(C)]
#[derive(Clone, Copy)]
pub struct Vertex {
    _pos: [f32; 4],
    _tex_coord: [f32; 2],
}

pub type Index = u16;

unsafe impl Pod for Vertex {}
unsafe impl Zeroable for Vertex {}

pub struct MeshData {
    vertex_data: Vec<Vertex>,
    index_data: Vec<u16>,
    static_mesh_handles: Vec<StaticMeshHandle>,
}

impl MeshData {
    pub fn new() -> Self {
        MeshData {
            vertex_data: vec![],
            index_data: vec![],
            static_mesh_handles: vec![],
        }
    }
    // todo: remove this clone, could get huge
    pub fn index_data(&self) -> Vec<u16> {
        self.index_data.clone()
    }
    // todo: remove this clone, could get huge
    pub fn vertex_data(&self) -> Vec<Vertex> {
        self.vertex_data.clone()
    }
    fn indices(&self) -> u32 {
        self.index_data.len() as u32
    }
    fn vertices(&self) -> i32 {
        self.vertex_data.len() as i32
    }
    pub fn static_mesh_handles(&self) -> Vec<StaticMeshHandle> {
        self.static_mesh_handles.clone()
    }
    pub fn insert_static_mesh(&mut self, mut static_mesh: StaticMesh) -> usize {
        let handle = StaticMeshHandle {
            indices: (self.indices()..self.indices() + static_mesh.index_data.len() as u32),
            base_vertex: self.vertices(),
        };
        self.index_data.append(&mut static_mesh.index_data);
        self.vertex_data.append(&mut static_mesh.vertex_data);
        self.static_mesh_handles.push(handle);
        self.static_mesh_handles.len()
    }
}

#[derive(Clone)]
pub struct StaticMeshHandle {
    pub indices: Range<u32>,
    pub base_vertex: i32,
}

#[derive(Clone)]
pub struct StaticMesh {
    pub vertex_data: Vec<Vertex>,
    pub index_data: Vec<u16>,
}

impl StaticMesh {
    pub fn new(path: &Path) -> Self {
        let (gltf, buffers) = load(std::path::Path::new(path)).unwrap();

        let mesh = gltf.meshes().next().unwrap();
        let primitive = mesh.primitives().next().unwrap();

        let reader = primitive.reader(|buffer| Some(&buffers[buffer.index()]));

        let index_data = match reader.read_indices().expect("") {
            ReadIndices::U16(a) => a.collect::<Vec<u16>>(),
            _ => panic!("index size not supported"),
        };

        let positions_reader = reader.read_positions().expect("no position data in gltf");

        let tex_coords_reader = reader.read_tex_coords(0).expect("no tex coord data");

        let zip = positions_reader.zip(tex_coords_reader.into_f32());

        let vertex_data = zip
            .map(|(pos, tex)| Vertex {
                _pos: [pos[0], pos[1], pos[2], 1.0],
                _tex_coord: tex,
            })
            .collect::<Vec<Vertex>>();

        StaticMesh {
            vertex_data,
            index_data,
        }
    }
}

pub fn load(path: &Path) -> Result<(Gltf, Vec<Vec<u8>>), GltfError> {
    let gltf = gltf::Gltf::open(path)?;
    let buffer_data = load_buffers(&gltf, path)?;
    Ok((gltf, buffer_data))
}

fn load_buffers(gltf: &gltf::Gltf, asset_path: &Path) -> Result<Vec<Vec<u8>>, GltfError> {
    const OCTET_STREAM_URI: &str = "data:application/octet-stream;base64,";

    let mut buffer_data = Vec::new();
    for buffer in gltf.buffers() {
        match buffer.source() {
            Source::Uri(uri) => {
                if uri.starts_with("data:") {
                    if uri.starts_with(OCTET_STREAM_URI) {
                        buffer_data.push(base64::decode(&uri[OCTET_STREAM_URI.len()..])?);
                    } else {
                        return Err(GltfError::BufferFormatUnsupported);
                    }
                } else {
                    let buffer_path = asset_path.parent().unwrap().join(uri);
                    let buffer_bytes = fs::read(buffer_path)?;
                    buffer_data.push(buffer_bytes);
                }
            }
            Source::Bin => {
                if let Some(blob) = gltf.blob.as_deref() {
                    buffer_data.push(blob.into());
                } else {
                    return Err(GltfError::MissingBlob);
                }
            }
        }
    }

    Ok(buffer_data)
}
