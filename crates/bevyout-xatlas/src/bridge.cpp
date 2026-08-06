#include "xatlas.h"

#include <cstdint>

struct BevyoutXatlasHandle {
    xatlas::Atlas *atlas;
};

struct BevyoutXatlasVertex {
    int32_t atlas_index;
    int32_t chart_index;
    float uv[2];
    uint32_t xref;
};

static_assert(sizeof(BevyoutXatlasVertex) == sizeof(xatlas::Vertex),
              "Rust and xatlas vertex layouts must remain identical");

extern "C" {

BevyoutXatlasHandle *bevyout_xatlas_create() {
    BevyoutXatlasHandle *handle = new BevyoutXatlasHandle;
    handle->atlas = xatlas::Create();
    return handle;
}

void bevyout_xatlas_destroy(BevyoutXatlasHandle *handle) {
    if (handle == nullptr) {
        return;
    }
    xatlas::Destroy(handle->atlas);
    delete handle;
}

int32_t bevyout_xatlas_add_mesh(
    BevyoutXatlasHandle *handle,
    const float *positions,
    const float *normals,
    const float *uvs,
    const uint32_t *indices,
    uint32_t vertex_count,
    uint32_t index_count) {
    if (handle == nullptr || positions == nullptr || indices == nullptr ||
        index_count == 0 || (index_count % 3) != 0) {
        return static_cast<int32_t>(xatlas::AddMeshError::Error);
    }
    xatlas::MeshDecl declaration;
    declaration.vertexPositionData = positions;
    declaration.vertexPositionStride = sizeof(float) * 3;
    declaration.vertexNormalData = normals;
    declaration.vertexNormalStride = normals == nullptr ? 0 : sizeof(float) * 3;
    declaration.vertexUvData = uvs;
    declaration.vertexUvStride = uvs == nullptr ? 0 : sizeof(float) * 2;
    declaration.indexData = indices;
    declaration.indexCount = index_count;
    declaration.faceCount = index_count / 3;
    declaration.vertexCount = vertex_count;
    declaration.indexFormat = xatlas::IndexFormat::UInt32;
    return static_cast<int32_t>(xatlas::AddMesh(handle->atlas, declaration));
}

void bevyout_xatlas_generate(
    BevyoutXatlasHandle *handle,
    uint32_t resolution,
    float texels_per_unit,
    uint32_t padding,
    uint32_t max_chart_size,
    bool block_align,
    bool fix_winding) {
    if (handle == nullptr) {
        return;
    }
    xatlas::ChartOptions charts;
    charts.fixWinding = fix_winding;
    xatlas::PackOptions pack;
    pack.resolution = resolution;
    pack.texelsPerUnit = texels_per_unit;
    pack.padding = padding;
    pack.maxChartSize = max_chart_size;
    pack.blockAlign = block_align;
    pack.createImage = false;
    xatlas::Generate(handle->atlas, charts, pack);
}

uint32_t bevyout_xatlas_width(const BevyoutXatlasHandle *handle) {
    return handle == nullptr ? 0 : handle->atlas->width;
}

uint32_t bevyout_xatlas_height(const BevyoutXatlasHandle *handle) {
    return handle == nullptr ? 0 : handle->atlas->height;
}

uint32_t bevyout_xatlas_atlas_count(const BevyoutXatlasHandle *handle) {
    if (handle == nullptr) {
        return 0;
    }
    return handle->atlas->atlasCount == 0 ? 1 : handle->atlas->atlasCount;
}

uint32_t bevyout_xatlas_mesh_count(const BevyoutXatlasHandle *handle) {
    return handle == nullptr ? 0 : handle->atlas->meshCount;
}

uint32_t bevyout_xatlas_mesh_vertex_count(
    const BevyoutXatlasHandle *handle,
    uint32_t mesh_index) {
    if (handle == nullptr || mesh_index >= handle->atlas->meshCount) {
        return 0;
    }
    return handle->atlas->meshes[mesh_index].vertexCount;
}

uint32_t bevyout_xatlas_mesh_index_count(
    const BevyoutXatlasHandle *handle,
    uint32_t mesh_index) {
    if (handle == nullptr || mesh_index >= handle->atlas->meshCount) {
        return 0;
    }
    return handle->atlas->meshes[mesh_index].indexCount;
}

const BevyoutXatlasVertex *bevyout_xatlas_mesh_vertices(
    const BevyoutXatlasHandle *handle,
    uint32_t mesh_index) {
    if (handle == nullptr || mesh_index >= handle->atlas->meshCount) {
        return nullptr;
    }
    return reinterpret_cast<const BevyoutXatlasVertex *>(
        handle->atlas->meshes[mesh_index].vertexArray);
}

const uint32_t *bevyout_xatlas_mesh_indices(
    const BevyoutXatlasHandle *handle,
    uint32_t mesh_index) {
    if (handle == nullptr || mesh_index >= handle->atlas->meshCount) {
        return nullptr;
    }
    return handle->atlas->meshes[mesh_index].indexArray;
}

}
