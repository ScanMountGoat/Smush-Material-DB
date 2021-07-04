use rusqlite::Transaction;
use rusqlite::{params, Result};
use ssbh_lib::formats::matl::{MatlBlendState, MatlRasterizerState, MatlSampler};
use std::fmt::Debug;
use std::sync::atomic::AtomicUsize;
use std::sync::atomic::Ordering;

extern crate sqlinsert_derive;
use sqlinsert_derive::SqlInsert;

// Simulate an autoincrementing primary key.
// Use atomics so no two records receive the same key.
static LAST_BOOL_ID: AtomicUsize = AtomicUsize::new(0);
static LAST_FLOAT_ID: AtomicUsize = AtomicUsize::new(0);
static LAST_VECTOR_ID: AtomicUsize = AtomicUsize::new(0);
static LAST_TEXTURE_ID: AtomicUsize = AtomicUsize::new(0);
static LAST_SAMPLER_ID: AtomicUsize = AtomicUsize::new(0);
static LAST_RASTERIZER_ID: AtomicUsize = AtomicUsize::new(0);
static LAST_BLEND_STATE_ID: AtomicUsize = AtomicUsize::new(0);

static LAST_MATL_ID: AtomicUsize = AtomicUsize::new(0);
static LAST_MATERIAL_ID: AtomicUsize = AtomicUsize::new(0);

static LAST_MODL_ID: AtomicUsize = AtomicUsize::new(0);

static LAST_MESH_ID: AtomicUsize = AtomicUsize::new(0);
static LAST_MESH_OBJECT_ID: AtomicUsize = AtomicUsize::new(0);
static LAST_MESH_ATTRIBUTE_ID: AtomicUsize = AtomicUsize::new(0);

static LAST_XMB_ID: AtomicUsize = AtomicUsize::new(0);
static LAST_XMB_ENTRY_ID: AtomicUsize = AtomicUsize::new(0);
static LAST_XMB_ATTRIBUTE_ID: AtomicUsize = AtomicUsize::new(0);

/// A type that can be converted to SQL params for inserting into a table.
pub trait SqlInsert: Sync + Send + Debug {
    fn insert(&self, transaction: &mut Transaction) -> Result<()>;
}

#[derive(SqlInsert, Debug)]
#[table("CustomBooleanParam")]
pub struct BoolRecord {
    id: i64,
    param_id: u32,
    material_id: i64,
    value: bool,
}

impl BoolRecord {
    pub fn create_record(param_id: u32, material_id: i64, value: bool) -> (i64, BoolRecord) {
        let id = LAST_BOOL_ID.fetch_add(1, Ordering::Relaxed) as i64;
        (
            id,
            BoolRecord {
                id,
                param_id,
                material_id,
                value,
            },
        )
    }
}

#[derive(SqlInsert, Debug)]
#[table("CustomFloatParam")]
pub struct FloatRecord {
    pub id: i64,
    pub param_id: u32,
    pub material_id: i64,
    pub value: f64,
}

impl FloatRecord {
    pub fn create_record(param_id: u32, material_id: i64, value: f64) -> (i64, FloatRecord) {
        let id = LAST_FLOAT_ID.fetch_add(1, Ordering::Relaxed) as i64;
        (
            id,
            FloatRecord {
                id,
                param_id,
                material_id,
                value,
            },
        )
    }
}

#[derive(SqlInsert, Debug)]
#[table("RasterizerState")]
pub struct RasterizerRecord {
    id: i64,
    param_id: u32,
    material_id: i64,
    fill_mode: u32,
    cull_mode: u32,
    depth_bias: f64,
    unk4: f64,
    unk5: f64,
    unk6: u32,
}

impl RasterizerRecord {
    pub fn create_record(
        param_id: u32,
        material_id: i64,
        val: &MatlRasterizerState,
    ) -> (i64, RasterizerRecord) {
        let id = LAST_RASTERIZER_ID.fetch_add(1, Ordering::Relaxed) as i64;
        (
            id,
            RasterizerRecord {
                id,
                param_id,
                material_id,
                fill_mode: val.fill_mode as u32,
                cull_mode: val.cull_mode as u32,
                depth_bias: val.depth_bias as f64,
                unk4: val.unk4 as f64,
                unk5: val.unk5 as f64,
                unk6: val.unk6,
            },
        )
    }
}

#[derive(SqlInsert, Debug)]
#[table("BlendState")]
pub struct BlendStateRecord {
    id: i64,
    param_id: u32,
    material_id: i64,
    source_color: u32,
    unk2: u32,
    destination_color: u32,
    unk4: u32,
    unk5: u32,
    unk6: u32,
    unk7: u32,
    unk8: u32,
    unk9: u32,
    unk10: u32,
}

impl BlendStateRecord {
    pub fn create_record(
        param_id: u32,
        material_id: i64,
        val: &MatlBlendState,
    ) -> (i64, BlendStateRecord) {
        let id = LAST_BLEND_STATE_ID.fetch_add(1, Ordering::Relaxed) as i64;
        (
            id,
            BlendStateRecord {
                id,
                param_id,
                material_id,
                source_color: val.source_color as u32,
                unk2: val.unk2,
                destination_color: val.destination_color as u32,
                unk4: val.unk4,
                unk5: val.unk5,
                unk6: val.unk6,
                unk7: val.unk7,
                unk8: val.unk8,
                unk9: val.unk9,
                unk10: val.unk10,
            },
        )
    }
}

#[derive(SqlInsert, Debug)]
#[table("Sampler")]
pub struct SamplerRecord {
    id: i64,
    param_id: u32,
    material_id: i64,
    wraps: u32,
    wrapt: u32,
    wrapr: u32,
    min_filter: u32,
    mag_filter: u32,
    texture_filtering_type: u32,
    border_color_r: f64,
    border_color_g: f64,
    border_color_b: f64,
    border_color_a: f64,
    unk11: u32,
    unk12: u32,
    lod_bias: f64,
    max_anisotropy: u32,
}

impl SamplerRecord {
    pub fn create_record(
        param_id: u32,
        material_id: i64,
        val: &MatlSampler,
    ) -> (i64, SamplerRecord) {
        let id = LAST_SAMPLER_ID.fetch_add(1, Ordering::Relaxed) as i64;
        (
            id,
            SamplerRecord {
                id,
                param_id,
                material_id,
                wraps: val.wraps as u32,
                wrapt: val.wrapt as u32,
                wrapr: val.wrapr as u32,
                min_filter: val.min_filter as u32,
                mag_filter: val.mag_filter as u32,
                texture_filtering_type: val.texture_filtering_type as u32,
                border_color_r: val.border_color.r as f64,
                border_color_g: val.border_color.g as f64,
                border_color_b: val.border_color.b as f64,
                border_color_a: val.border_color.a as f64,
                unk11: val.unk11,
                unk12: val.unk12,
                lod_bias: val.lod_bias as f64,
                max_anisotropy: val.max_anisotropy,
            },
        )
    }
}

#[derive(SqlInsert, Debug)]
#[table("Material")]
pub struct MaterialRecord {
    id: i64,
    matl_id: i64,
    material_label: String,
    shader_label: String,
}

impl MaterialRecord {
    pub fn create_record(
        matl_id: i64,
        material_label: String,
        shader_label: String,
    ) -> (i64, MaterialRecord) {
        let id = LAST_MATERIAL_ID.fetch_add(1, Ordering::Relaxed) as i64;
        (
            id,
            MaterialRecord {
                id,
                matl_id,
                material_label,
                shader_label,
            },
        )
    }
}

#[derive(SqlInsert, Debug)]
#[table("Texture")]
pub struct TextureRecord {
    id: i64,
    param_id: u32,
    material_id: i64,
    value: String,
}

impl TextureRecord {
    pub fn create_record(param_id: u32, material_id: i64, text: String) -> (i64, TextureRecord) {
        let id = LAST_TEXTURE_ID.fetch_add(1, Ordering::Relaxed) as i64;
        (
            id,
            TextureRecord {
                id,
                param_id,
                material_id,
                value: text,
            },
        )
    }
}

#[derive(SqlInsert, Debug)]
#[table("CustomVectorParam")]
pub struct Vector4Record {
    id: i64,
    param_id: u32,
    material_id: i64,
    x: f64,
    y: f64,
    z: f64,
    w: f64,
}

impl Vector4Record {
    pub fn create_record(
        param_id: u32,
        material_id: i64,
        x: f64,
        y: f64,
        z: f64,
        w: f64,
    ) -> (i64, Vector4Record) {
        let id = LAST_VECTOR_ID.fetch_add(1, Ordering::Relaxed) as i64;
        (
            id,
            Vector4Record {
                id,
                param_id,
                material_id,
                x,
                y,
                z,
                w,
            },
        )
    }
}

#[derive(SqlInsert, Debug)]
#[table("Matl")]
pub struct MatlRecord {
    id: i64,
    directory: String,
    file_name: String,
}

impl MatlRecord {
    pub fn create_record(directory: String, file_name: String) -> (i64, MatlRecord) {
        let id = LAST_MATL_ID.fetch_add(1, Ordering::Relaxed) as i64;
        (
            id as i64,
            MatlRecord {
                id,
                directory,
                file_name,
            },
        )
    }
}

#[derive(SqlInsert, Debug)]
#[table("Xmb")]
pub struct XmbRecord {
    id: i64,
    directory: String,
    file_name: String,
}

impl XmbRecord {
    pub fn create_record(directory: String, file_name: String) -> (i64, XmbRecord) {
        let id = LAST_XMB_ID.fetch_add(1, Ordering::Relaxed) as i64;
        (
            id,
            XmbRecord {
                id,
                directory,
                file_name,
            },
        )
    }
}

#[derive(SqlInsert, Debug)]
#[table("XmbEntry")]
pub struct XmbEntryRecord {
    id: i64,
    xmb_id: i64,
    name: String,
}

impl XmbEntryRecord {
    pub fn create_record(xmb_id: i64, name: String) -> (i64, XmbEntryRecord) {
        let id = LAST_XMB_ENTRY_ID.fetch_add(1, Ordering::Relaxed) as i64;
        (id, XmbEntryRecord { id, xmb_id, name })
    }
}

#[derive(SqlInsert, Debug)]
#[table("XmbAttribute")]
pub struct XmbAttributeRecord {
    id: i64,
    xmb_entry_id: i64,
    name: String,
    value: String,
}

impl XmbAttributeRecord {
    pub fn create_record(
        xmb_entry_id: i64,
        name: String,
        value: String,
    ) -> (i64, XmbAttributeRecord) {
        let id = LAST_XMB_ATTRIBUTE_ID.fetch_add(1, Ordering::Relaxed) as i64;
        (
            id as i64,
            XmbAttributeRecord {
                id,
                xmb_entry_id,
                name,
                value,
            },
        )
    }
}

#[derive(SqlInsert, Debug)]
#[table("Mesh")]
pub struct MeshRecord {
    id: i64,
    directory: String,
    file_name: String,
}

impl MeshRecord {
    pub fn create_record(directory: String, file_name: String) -> (i64, MeshRecord) {
        let id = LAST_MESH_ID.fetch_add(1, Ordering::Relaxed) as i64;
        (
            id,
            MeshRecord {
                id,
                directory,
                file_name,
            },
        )
    }
}

#[derive(SqlInsert, Debug)]
#[table("Modl")]
pub struct ModlRecord {
    id: i64,
    directory: String,
    file_name: String,
    model_file_name: String,
    skeleton_file_name: String,
    material_file_name: String,
}

impl ModlRecord {
    pub fn create_record(
        directory: String,
        file_name: String,
        model_file_name: String,
        skeleton_file_name: String,
        material_file_name: String,
    ) -> (i64, ModlRecord) {
        let id = LAST_MODL_ID.fetch_add(1, Ordering::Relaxed) as i64;
        (
            id,
            ModlRecord {
                id,
                directory,
                file_name,
                model_file_name,
                skeleton_file_name,
                material_file_name,
            },
        )
    }
}

#[derive(SqlInsert, Debug)]
#[table("MeshObject")]
pub struct MeshObjectRecord {
    id: i64,
    mesh_id: i64,
    name: String,
    sub_index: i64,
}

impl MeshObjectRecord {
    pub fn create_record(
        mesh_id: i64,
        mesh_name: String,
        sub_index: i64,
    ) -> (i64, MeshObjectRecord) {
        let id = LAST_MESH_OBJECT_ID.fetch_add(1, Ordering::Relaxed) as i64;
        (
            id as i64,
            MeshObjectRecord {
                id,
                mesh_id,
                name: mesh_name,
                sub_index,
            },
        )
    }
}

#[derive(SqlInsert, Debug)]
#[table("MeshAttribute")]
pub struct MeshAttributeRecord {
    id: i64,
    mesh_object_id: i64,
    name: String,
}

impl MeshAttributeRecord {
    pub fn create_record(
        mesh_object_id: i64,
        attribute_name: String,
    ) -> (i64, MeshAttributeRecord) {
        let id = LAST_MESH_ATTRIBUTE_ID.fetch_add(1, Ordering::Relaxed) as i64;
        (
            id as i64,
            MeshAttributeRecord {
                id,
                mesh_object_id,
                name: attribute_name,
            },
        )
    }
}
