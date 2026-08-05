use std::ptr::NonNull;
use std::slice;

#[repr(C)]
#[derive(Clone, Copy, Debug)]
struct RawVertex {
    atlas_index: i32,
    chart_index: i32,
    uv: [f32; 2],
    xref: u32,
}

#[repr(C)]
struct RawHandle {
    _private: [u8; 0],
}

unsafe extern "C" {
    fn bevyout_xatlas_create() -> *mut RawHandle;
    fn bevyout_xatlas_destroy(handle: *mut RawHandle);
    fn bevyout_xatlas_add_mesh(
        handle: *mut RawHandle,
        positions: *const f32,
        normals: *const f32,
        uvs: *const f32,
        indices: *const u32,
        vertex_count: u32,
        index_count: u32,
    ) -> i32;
    fn bevyout_xatlas_generate(
        handle: *mut RawHandle,
        resolution: u32,
        texels_per_unit: f32,
        padding: u32,
        max_chart_size: u32,
        block_align: bool,
        fix_winding: bool,
    );
    fn bevyout_xatlas_width(handle: *const RawHandle) -> u32;
    fn bevyout_xatlas_height(handle: *const RawHandle) -> u32;
    fn bevyout_xatlas_atlas_count(handle: *const RawHandle) -> u32;
    fn bevyout_xatlas_mesh_count(handle: *const RawHandle) -> u32;
    fn bevyout_xatlas_mesh_vertex_count(handle: *const RawHandle, mesh_index: u32) -> u32;
    fn bevyout_xatlas_mesh_index_count(handle: *const RawHandle, mesh_index: u32) -> u32;
    fn bevyout_xatlas_mesh_vertices(handle: *const RawHandle, mesh_index: u32) -> *const RawVertex;
    fn bevyout_xatlas_mesh_indices(handle: *const RawHandle, mesh_index: u32) -> *const u32;
}

#[derive(Clone, Copy, Debug)]
pub struct Options {
    pub resolution: u32,
    pub texels_per_unit: f32,
    pub padding: u32,
    pub max_chart_size: u32,
    pub block_align: bool,
    pub fix_winding: bool,
}

impl Default for Options {
    fn default() -> Self {
        Self {
            resolution: 4096,
            texels_per_unit: 16.0,
            padding: 12,
            max_chart_size: 4096,
            block_align: true,
            fix_winding: true,
        }
    }
}

pub struct MeshInput<'a> {
    pub positions: &'a [[f32; 3]],
    pub normals: Option<&'a [[f32; 3]]>,
    pub uvs: Option<&'a [[f32; 2]]>,
    pub indices: &'a [u32],
}

#[derive(Clone, Copy, Debug)]
pub struct Vertex {
    pub atlas_index: i32,
    pub chart_index: i32,
    pub uv: [f32; 2],
    pub xref: u32,
}

#[derive(Debug)]
pub struct Mesh {
    pub width: u32,
    pub height: u32,
    pub atlas_count: u32,
    pub vertices: Vec<Vertex>,
    pub indices: Vec<u32>,
}

#[derive(Debug)]
pub enum Error {
    InvalidInput(&'static str),
    Native(String),
    NoOutput,
}

impl std::fmt::Display for Error {
    fn fmt(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Self::InvalidInput(message) => formatter.write_str(message),
            Self::Native(code) => write!(formatter, "native xatlas error {code}"),
            Self::NoOutput => formatter.write_str("xatlas produced no output mesh"),
        }
    }
}

impl std::error::Error for Error {}

pub fn generate(input: MeshInput<'_>, options: Options) -> Result<Mesh, Error> {
    if input.positions.is_empty() {
        return Err(Error::InvalidInput("xatlas positions are empty"));
    }
    if input.indices.is_empty() || !input.indices.len().is_multiple_of(3) {
        return Err(Error::InvalidInput("xatlas indices must contain triangles"));
    }
    if input
        .indices
        .iter()
        .any(|index| *index as usize >= input.positions.len())
    {
        return Err(Error::InvalidInput("xatlas index is out of range"));
    }
    if input
        .normals
        .is_some_and(|normals| normals.len() != input.positions.len())
    {
        return Err(Error::InvalidInput("xatlas normals do not match positions"));
    }
    if input
        .uvs
        .is_some_and(|uvs| uvs.len() != input.positions.len())
    {
        return Err(Error::InvalidInput("xatlas UVs do not match positions"));
    }

    let positions = input
        .positions
        .iter()
        .flat_map(|value| value.iter().copied())
        .collect::<Vec<_>>();
    let normals = input.normals.map(|values| {
        values
            .iter()
            .flat_map(|value| value.iter().copied())
            .collect::<Vec<_>>()
    });
    let uvs = input.uvs.map(|values| {
        values
            .iter()
            .flat_map(|value| value.iter().copied())
            .collect::<Vec<_>>()
    });

    let handle = NonNull::new(unsafe { bevyout_xatlas_create() }).ok_or(Error::NoOutput)?;
    let result = generate_mesh(
        handle,
        &positions,
        normals.as_deref(),
        uvs.as_deref(),
        input.indices,
        options,
    );
    unsafe { bevyout_xatlas_destroy(handle.as_ptr()) };
    result
}

fn generate_mesh(
    handle: NonNull<RawHandle>,
    positions: &[f32],
    normals: Option<&[f32]>,
    uvs: Option<&[f32]>,
    indices: &[u32],
    options: Options,
) -> Result<Mesh, Error> {
    let result = unsafe {
        bevyout_xatlas_add_mesh(
            handle.as_ptr(),
            positions.as_ptr(),
            normals.map_or(std::ptr::null(), |values| values.as_ptr()),
            uvs.map_or(std::ptr::null(), |values| values.as_ptr()),
            indices.as_ptr(),
            (positions.len() / 3) as u32,
            indices.len() as u32,
        )
    };
    if result != 0 {
        return Err(Error::Native(result.to_string()));
    }
    unsafe {
        bevyout_xatlas_generate(
            handle.as_ptr(),
            options.resolution,
            options.texels_per_unit,
            options.padding,
            options.max_chart_size,
            options.block_align,
            options.fix_winding,
        );
    }
    let mesh_count = unsafe { bevyout_xatlas_mesh_count(handle.as_ptr()) };
    if mesh_count == 0 {
        return Err(Error::NoOutput);
    }
    let vertex_count = unsafe { bevyout_xatlas_mesh_vertex_count(handle.as_ptr(), 0) } as usize;
    let index_count = unsafe { bevyout_xatlas_mesh_index_count(handle.as_ptr(), 0) } as usize;
    let vertices = unsafe {
        slice::from_raw_parts(
            bevyout_xatlas_mesh_vertices(handle.as_ptr(), 0),
            vertex_count,
        )
    }
    .iter()
    .map(|vertex| Vertex {
        atlas_index: vertex.atlas_index,
        chart_index: vertex.chart_index,
        uv: vertex.uv,
        xref: vertex.xref,
    })
    .collect();
    let indices = unsafe {
        slice::from_raw_parts(bevyout_xatlas_mesh_indices(handle.as_ptr(), 0), index_count)
    }
    .to_vec();
    Ok(Mesh {
        width: unsafe { bevyout_xatlas_width(handle.as_ptr()) },
        height: unsafe { bevyout_xatlas_height(handle.as_ptr()) },
        atlas_count: unsafe { bevyout_xatlas_atlas_count(handle.as_ptr()) },
        vertices,
        indices,
    })
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn unwraps_a_quad_and_preserves_source_xrefs() {
        let positions = [
            [0.0, 0.0, 0.0],
            [1.0, 0.0, 0.0],
            [1.0, 1.0, 0.0],
            [0.0, 1.0, 0.0],
        ];
        let normals = [[0.0, 0.0, 1.0]; 4];
        let uvs = [[0.0, 0.0], [1.0, 0.0], [1.0, 1.0], [0.0, 1.0]];
        let mesh = generate(
            MeshInput {
                positions: &positions,
                normals: Some(&normals),
                uvs: Some(&uvs),
                indices: &[0, 1, 2, 0, 2, 3],
            },
            Options {
                resolution: 128,
                texels_per_unit: 16.0,
                padding: 2,
                max_chart_size: 128,
                ..Default::default()
            },
        )
        .unwrap();
        assert_eq!(mesh.indices.len(), 6);
        assert_eq!(mesh.vertices.len(), 4);
        assert_eq!(mesh.atlas_count, 1);
        assert!(mesh.width > 0 && mesh.height > 0);
        assert!(mesh.vertices.iter().all(|vertex| {
            vertex.xref < positions.len() as u32
                && vertex.uv.iter().all(|coordinate| coordinate.is_finite())
        }));
    }
}
