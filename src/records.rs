use rusqlite::Transaction;
use rusqlite::{params, Result};
use std::fmt::Debug;
use std::sync::atomic::AtomicUsize;
use std::sync::atomic::Ordering;

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

#[derive(Debug)]
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

impl SqlInsert for BoolRecord {
    fn insert(&self, transaction: &mut Transaction) -> Result<()> {
        transaction
            .prepare_cached(
                "INSERT INTO CustomBooleanParam(ID, ParamID, MaterialID, Value) VALUES(?,?,?,?)",
            )?
            .execute(params![
                self.id,
                self.param_id,
                self.material_id,
                self.value
            ])?;
        Ok(())
    }
}

#[derive(Debug)]
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

impl SqlInsert for FloatRecord {
    fn insert(&self, transaction: &mut Transaction) -> Result<()> {
        transaction
            .prepare_cached(
                "INSERT INTO CustomFloatParam(ID, ParamID, MaterialID, Value) VALUES(?,?,?,?)",
            )?
            .execute(params![
                self.id,
                self.param_id,
                self.material_id,
                self.value
            ])?;
        Ok(())
    }
}

#[derive(Debug)]
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
    unk7: u32,
    unk8: f64,
}

impl RasterizerRecord {
    pub fn create_record(
        param_id: u32,
        material_id: i64,
        fill_mode: u32,
        cull_mode: u32,
        depth_bias: f64,
        unk4: f64,
        unk5: f64,
        unk6: u32,
        unk7: u32,
        unk8: f64,
    ) -> (i64, RasterizerRecord) {
        let id = LAST_RASTERIZER_ID.fetch_add(1, Ordering::Relaxed) as i64;
        (
            id,
            RasterizerRecord {
                id,
                param_id,
                material_id,
                fill_mode,
                cull_mode,
                depth_bias,
                unk4,
                unk5,
                unk6,
                unk7,
                unk8,
            },
        )
    }
}

impl SqlInsert for RasterizerRecord {
    fn insert(&self, transaction: &mut Transaction) -> Result<()> {
        transaction
            .prepare_cached("INSERT INTO RasterizerState(ID, ParamID, MaterialID, Value1, Value2, Value3, Value4, Value5, Value6, Value7, Value8) VALUES(?,?,?,?,?,?,?,?,?,?,?)")?
            .execute(params![
                self.id,
                self.param_id,
                self.material_id,
                self.fill_mode,
                self.cull_mode,
                self.depth_bias,
                self.unk4,
                self.unk5,
                self.unk6,
                self.unk7,
                self.unk8,
            ])?;
        Ok(())
    }
}

#[derive(Debug)]
pub struct BlendStateRecord {
    id: i64,
    param_id: u32,
    material_id: i64,
    unk1: u32,
    unk2: u32,
    blend_factor1: u32,
    unk4: u32,
    unk5: u32,
    blend_factor2: u32,
    unk7: u32,
    unk8: u32,
    unk9: u32,
    unk10: u32,
    unk11: u32,
    unk12: u32,
}

impl BlendStateRecord {
    pub fn create_record(
        param_id: u32,
        material_id: i64,
        unk1: u32,
        unk2: u32,
        blend_factor1: u32,
        unk4: u32,
        unk5: u32,
        blend_factor2: u32,
        unk7: u32,
        unk8: u32,
        unk9: u32,
        unk10: u32,
        unk11: u32,
        unk12: u32,
    ) -> (i64, BlendStateRecord) {
        let id = LAST_BLEND_STATE_ID.fetch_add(1, Ordering::Relaxed) as i64;
        (
            id,
            BlendStateRecord {
                id,
                param_id,
                material_id,
                unk1,
                unk2,
                blend_factor1,
                unk4,
                unk5,
                blend_factor2,
                unk7,
                unk8,
                unk9,
                unk10,
                unk11,
                unk12,
            },
        )
    }
}

impl SqlInsert for BlendStateRecord {
    fn insert(&self, transaction: &mut Transaction) -> Result<()> {
        transaction
            .prepare_cached("INSERT INTO BlendState(ID, ParamID, MaterialID, Value1, Value2, Value3, Value4, Value5, Value6, Value7, Value8, Value9, Value10, Value11, Value12) VALUES(?,?,?,?,?,?,?,?,?,?,?,?,?,?,?)")?
            .execute(params![
                self.id,
                self.param_id,
                self.material_id,
                self.unk1,
                self.unk2,
                self.blend_factor1,
                self.unk4,
                self.unk5,
                self.blend_factor2,
                self.unk7,
                self.unk8,
                self.unk9,
                self.unk10,
                self.unk11,
                self.unk12
            ])?;
        Ok(())
    }
}

#[derive(Debug)]
pub struct SamplerRecord {
    id: i64,
    param_id: u32,
    material_id: i64,
    wraps: u32,
    wrapt: u32,
    wrapr: u32,
    min_filter: u32,
    mag_filter: u32,
    unk6: u32,
    unk7: u32,
    unk8: u32,
    unk9: u32,
    unk10: u32,
    unk11: u32,
    unk12: u32,
    lod_bias: f64,
    max_anisotropy: u32,
}

impl SamplerRecord {
    pub fn create_record(
        param_id: u32,
        material_id: i64,
        wraps: u32,
        wrapt: u32,
        wrapr: u32,
        min_filter: u32,
        mag_filter: u32,
        unk6: u32,
        unk7: u32,
        unk8: u32,
        unk9: u32,
        unk10: u32,
        unk11: u32,
        unk12: u32,
        lod_bias: f64,
        max_anisotropy: u32,
    ) -> (i64, SamplerRecord) {
        let id = LAST_SAMPLER_ID.fetch_add(1, Ordering::Relaxed) as i64;
        (
            id,
            SamplerRecord {
                id,
                param_id,
                material_id,
                wraps,
                wrapt,
                wrapr,
                min_filter,
                mag_filter,
                unk6,
                unk7,
                unk8,
                unk9,
                unk10,
                unk11,
                unk12,
                lod_bias,
                max_anisotropy,
            },
        )
    }
}

impl SqlInsert for SamplerRecord {
    fn insert(&self, transaction: &mut Transaction) -> Result<()> {
        transaction
            .prepare_cached("INSERT INTO Sampler(ID, ParamID, MaterialID, Value1, Value2, Value3, Value4, Value5, Value6, Value7, Value8, Value9, Value10, Value11, Value12, Value13, Value14) VALUES(?,?,?,?,?,?,?,?,?,?,?,?,?,?,?,?,?)")?
            .execute(params![
                self.id,
                self.param_id,
                self.material_id,
                self.wraps,
                self.wrapt,
                self.wrapr,
                self.min_filter,
                self.mag_filter,
                self.unk6,
                self.unk7,
                self.unk8,
                self.unk9,
                self.unk10,
                self.unk11,
                self.unk12,
                self.lod_bias,
                self.max_anisotropy
            ])?;
        Ok(())
    }
}

#[derive(Debug)]
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

impl SqlInsert for MaterialRecord {
    fn insert(&self, transaction: &mut Transaction) -> Result<()> {
        transaction
            .prepare_cached(
                "INSERT INTO Material(ID, MatlID, MaterialLabel, ShaderLabel) VALUES(?,?,?,?)",
            )?
            .execute(params![
                self.id,
                self.matl_id,
                self.material_label,
                &self.shader_label
            ])?;
        Ok(())
    }
}

#[derive(Debug)]
pub struct TextureRecord {
    id: i64,
    param_id: u32,
    material_id: i64,
    text: String,
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
                text,
            },
        )
    }
}

impl SqlInsert for TextureRecord {
    fn insert(&self, transaction: &mut Transaction) -> Result<()> {
        transaction
            .prepare_cached("INSERT INTO Texture(ID, ParamID, MaterialID, Value) VALUES(?,?,?,?)")?
            .execute(params![
                self.id,
                self.param_id,
                self.material_id,
                &self.text
            ])?;
        Ok(())
    }
}

#[derive(Debug)]
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

impl SqlInsert for Vector4Record {
    fn insert(&self, transaction: &mut Transaction) -> Result<()> {
        transaction
            .prepare_cached("INSERT INTO CustomVectorParam(ID, ParamID, MaterialID, Value1, Value2, Value3, Value4) VALUES(?,?,?,?,?,?,?)")?
            .execute(params![
                self.id,
                self.param_id,
                self.material_id,
                self.x,
                self.y,
                self.z,
                self.w
            ])?;
        Ok(())
    }
}

#[derive(Debug)]
pub struct MatlRecord {
    id: i64,
    directory_id: String,
    file_name: String,
}

impl MatlRecord {
    pub fn create_record(directory_id: String, file_name: String) -> (i64, MatlRecord) {
        let id = LAST_MATL_ID.fetch_add(1, Ordering::Relaxed) as i64;
        (
            id as i64,
            MatlRecord {
                id,
                directory_id,
                file_name,
            },
        )
    }
}

impl SqlInsert for MatlRecord {
    fn insert(&self, transaction: &mut Transaction) -> Result<()> {
        transaction
            .prepare_cached("INSERT INTO Matl(ID, Directory, FileName) VALUES(?,?,?)")?
            .execute(params![self.id, self.directory_id, self.file_name])?;
        Ok(())
    }
}

#[derive(Debug)]
pub struct XmbRecord {
    id: i64,
    directory_id: String,
    file_name: String,
}

impl XmbRecord {
    pub fn create_record(directory_id: String, file_name: String) -> (i64, XmbRecord) {
        // TODO: change ordering to relaxed?
        let id = LAST_XMB_ID.fetch_add(1, Ordering::Relaxed) as i64;
        (
            id,
            XmbRecord {
                id,
                directory_id,
                file_name,
            },
        )
    }
}

impl SqlInsert for XmbRecord {
    fn insert(&self, transaction: &mut Transaction) -> Result<()> {
        transaction
            .prepare_cached("INSERT INTO Xmb(ID, Directory, FileName) VALUES(?,?,?)")?
            .execute(params![self.id, self.directory_id, self.file_name])?;
        Ok(())
    }
}

#[derive(Debug)]
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

impl SqlInsert for XmbEntryRecord {
    fn insert(&self, transaction: &mut Transaction) -> Result<()> {
        transaction
            .prepare_cached("INSERT INTO XmbEntry(ID, XmbID, Name) VALUES(?,?,?)")?
            .execute(params![self.id, self.xmb_id, self.name])?;
        Ok(())
    }
}

#[derive(Debug)]
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

impl SqlInsert for XmbAttributeRecord {
    fn insert(&self, transaction: &mut Transaction) -> Result<()> {
        transaction
            .prepare_cached(
                "INSERT INTO XmbAttribute(ID, XmbEntryID, Name, Value) VALUES(?,?,?,?)",
            )?
            .execute(params![self.id, self.xmb_entry_id, self.name, self.value])?;
        Ok(())
    }
}

#[derive(Debug)]
pub struct MeshRecord {
    id: i64,
    directory_id: String,
    file_name: String,
}

impl MeshRecord {
    pub fn create_record(directory_id: String, file_name: String) -> (i64, MeshRecord) {
        let id = LAST_MESH_ID.fetch_add(1, Ordering::Relaxed) as i64;
        (
            id,
            MeshRecord {
                id,
                directory_id,
                file_name,
            },
        )
    }
}

impl SqlInsert for MeshRecord {
    fn insert(&self, transaction: &mut Transaction) -> Result<()> {
        transaction
            .prepare_cached("INSERT INTO Mesh(ID, Directory, FileName) VALUES(?,?,?)")?
            .execute(params![self.id, self.directory_id, self.file_name])?;
        Ok(())
    }
}

#[derive(Debug)]
pub struct ModlRecord {
    id: i64,
    directory_id: String,
    file_name: String,
    model_file_name: String,
    skeleton_file_name: String,
    material_file_name: String,
}

impl ModlRecord {
    pub fn create_record(
        directory_id: String,
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
                directory_id,
                file_name,
                model_file_name,
                skeleton_file_name,
                material_file_name,
            },
        )
    }
}

impl SqlInsert for ModlRecord {
    fn insert(&self, transaction: &mut Transaction) -> Result<()> {
        transaction
            .prepare_cached("INSERT INTO Modl(ID, Directory, FileName, ModelFileName, SkeletonFileName, MaterialFileName) VALUES(?,?,?,?,?,?)")?
            .execute(params![self.id, self.directory_id, self.file_name, self.model_file_name, self.skeleton_file_name, self.material_file_name])?;
        Ok(())
    }
}

#[derive(Debug)]
pub struct MeshObjectRecord {
    id: i64,
    mesh_id: i64,
    mesh_name: String,
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
                mesh_name,
                sub_index,
            },
        )
    }
}

impl SqlInsert for MeshObjectRecord {
    fn insert(&self, transaction: &mut Transaction) -> Result<()> {
        transaction
            .prepare_cached("INSERT INTO MeshObject(ID, MeshID, Name, SubIndex) VALUES(?,?,?,?)")?
            .execute(params![
                self.id,
                self.mesh_id,
                self.mesh_name,
                self.sub_index
            ])?;
        Ok(())
    }
}

#[derive(Debug)]
pub struct MeshAttributeRecord {
    id: i64,
    mesh_object_id: i64,
    attribute_name: String,
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
                attribute_name,
            },
        )
    }
}

impl SqlInsert for MeshAttributeRecord {
    fn insert(&self, transaction: &mut Transaction) -> Result<()> {
        transaction
            .prepare_cached("INSERT INTO MeshAttribute(ID, MeshObjectID, Name) VALUES(?,?,?)")?
            .execute(params![self.id, self.mesh_object_id, self.attribute_name])?;
        Ok(())
    }
}
