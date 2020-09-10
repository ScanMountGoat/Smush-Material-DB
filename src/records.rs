use rusqlite::Transaction;
use rusqlite::{params, Result};
use std::fmt::Debug;
use std::sync::atomic::AtomicUsize;
use std::sync::atomic::Ordering;

// TODO: Hard code these into the functions.
const INSERT_BOOLEAN: &str =
    "INSERT INTO CustomBooleanParam(ParamID, MaterialID, Value) VALUES(?,?,?)";
const INSERT_FLOAT: &str = "INSERT INTO CustomFloatParam(ParamID, MaterialID, Value) VALUES(?,?,?)";
const INSERT_VECTOR4: &str = "INSERT INTO CustomVectorParam(ParamID, MaterialID, Value1, Value2, Value3, Value4) VALUES(?,?,?,?,?,?)";
const INSERT_TEXTURE: &str = "INSERT INTO Texture(ParamID, MaterialID, Value) VALUES(?,?,?)";
const INSERT_SAMPLER: &str = "INSERT INTO Sampler(ParamID, MaterialID, Value1, Value2, Value3, Value4, Value5, Value6, Value7, Value8, Value9, Value10, Value11, Value12, Value13, Value14) 
VALUES(?,?,?,?,?,?,?,?,?,?,?,?,?,?,?,?)";
const INSERT_RASTERIZER: &str = "INSERT INTO RasterizerState(ParamID, MaterialID, Value1, Value2, Value3, Value4, Value5, Value6, Value7, Value8) 
VALUES(?,?,?,?,?,?,?,?,?,?)";
const INSERT_BLEND_STATE: &str = "INSERT INTO BlendState(ParamID, MaterialID, Value1, Value2, Value3, Value4, Value5, Value6, Value7, Value8, Value9, Value10, Value11, Value12) 
VALUES(?,?,?,?,?,?,?,?,?,?,?,?,?,?)";
const INSERT_MATERIAL: &str = "INSERT INTO Material(MatlID, MaterialLabel, ShaderLabel) 
VALUES(?,?,?)";
const INSERT_MESH_ATTRIBUTE: &str = "INSERT INTO MeshAttribute(MeshObjectID, Name) VALUES(?,?)";
const INSERT_MATL: &str = "INSERT INTO Matl(Directory, FileName) VALUES(?,?)";
const INSERT_MESH: &str = "INSERT INTO Mesh(Directory, FileName) VALUES(?,?)";
const INSERT_MESH_OBJECT: &str = "INSERT INTO MeshObject(MeshID, Name, SubIndex) VALUES(?,?,?)";

// Simulate an autoincrementing primary key.
// Use atomics so no two records receive the same key.
// The first call to ::create_record() will return the old value of 1.
static CURRENT_MATL_ID: AtomicUsize = AtomicUsize::new(1);
static CURRENT_MATERIAL_ID: AtomicUsize = AtomicUsize::new(1);
static CURRENT_MESH_ID: AtomicUsize = AtomicUsize::new(1);
static CURRENT_MESH_OBJECT_ID: AtomicUsize = AtomicUsize::new(1);
static CURRENT_XMB_ID: AtomicUsize = AtomicUsize::new(1);
static CURRENT_XMB_ENTRY_ID: AtomicUsize = AtomicUsize::new(1);

/// A type that can be converted to SQL params for inserting into a table.
pub trait SqlInsert: Sync + Send + Debug {
    fn insert(&self, transaction: &mut Transaction) -> Result<()>;
}

#[derive(Debug)]
pub struct BoolRecord {
    pub param_id: u32,
    pub material_id: i64,
    pub value: bool,
}

impl SqlInsert for BoolRecord {
    fn insert(&self, transaction: &mut Transaction) -> Result<()> {
        transaction
            .prepare_cached(INSERT_BOOLEAN)?
            .execute(params![self.param_id, self.material_id, self.value])?;
        Ok(())
    }
}

#[derive(Debug)]
pub struct FloatRecord {
    pub param_id: u32,
    pub material_id: i64,
    pub value: f64,
}

impl SqlInsert for FloatRecord {
    fn insert(&self, transaction: &mut Transaction) -> Result<()> {
        transaction.prepare_cached(INSERT_FLOAT)?.execute(params![
            self.param_id,
            self.material_id,
            self.value
        ])?;
        Ok(())
    }
}

#[derive(Debug)]
pub struct RasterizerRecord {
    pub param_id: u32,
    pub material_id: i64,
    pub fill_mode: u32,
    pub cull_mode: u32,
    pub depth_bias: f64,
    pub unk4: f64,
    pub unk5: f64,
    pub unk6: u32,
    pub unk7: u32,
    pub unk8: f64,
}

impl SqlInsert for RasterizerRecord {
    fn insert(&self, transaction: &mut Transaction) -> Result<()> {
        transaction
            .prepare_cached(INSERT_RASTERIZER)?
            .execute(params![
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
    pub param_id: u32,
    pub material_id: i64,
    pub unk1: u32,
    pub unk2: u32,
    pub blend_factor1: u32,
    pub unk4: u32,
    pub unk5: u32,
    pub blend_factor2: u32,
    pub unk7: u32,
    pub unk8: u32,
    pub unk9: u32,
    pub unk10: u32,
    pub unk11: u32,
    pub unk12: u32,
}

impl SqlInsert for BlendStateRecord {
    fn insert(&self, transaction: &mut Transaction) -> Result<()> {
        transaction
            .prepare_cached(INSERT_BLEND_STATE)?
            .execute(params![
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
    pub param_id: u32,
    pub material_id: i64,
    pub wraps: u32,
    pub wrapt: u32,
    pub wrapr: u32,
    pub min_filter: u32,
    pub mag_filter: u32,
    pub unk6: u32,
    pub unk7: u32,
    pub unk8: u32,
    pub unk9: u32,
    pub unk10: u32,
    pub unk11: u32,
    pub unk12: u32,
    pub lod_bias: f64,
    pub max_anisotropy: u32,
}

impl SqlInsert for SamplerRecord {
    fn insert(&self, transaction: &mut Transaction) -> Result<()> {
        transaction
            .prepare_cached(INSERT_SAMPLER)?
            .execute(params![
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
    pub matl_id: i64,
    pub material_label: String,
    pub shader_label: String,
}

impl MaterialRecord {
    pub fn create_record(
        matl_id: i64,
        material_label: String,
        shader_label: String,
    ) -> (i64, MaterialRecord) {
        let new_id = CURRENT_MATERIAL_ID.fetch_add(1, Ordering::SeqCst);
        (
            new_id as i64,
            MaterialRecord {
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
            .prepare_cached(INSERT_MATERIAL)?
            .execute(params![
                self.matl_id,
                self.material_label,
                &self.shader_label
            ])?;
        Ok(())
    }
}

#[derive(Debug)]
pub struct TextureRecord {
    pub param_id: u32,
    pub material_id: i64,
    pub text: String,
}

impl SqlInsert for TextureRecord {
    fn insert(&self, transaction: &mut Transaction) -> Result<()> {
        transaction
            .prepare_cached(INSERT_TEXTURE)?
            .execute(params![self.param_id, self.material_id, &self.text])?;
        Ok(())
    }
}

#[derive(Debug)]
pub struct Vector4Record {
    pub param_id: u32,
    pub material_id: i64,
    pub x: f64,
    pub y: f64,
    pub z: f64,
    pub w: f64,
}

impl SqlInsert for Vector4Record {
    fn insert(&self, transaction: &mut Transaction) -> Result<()> {
        transaction
            .prepare_cached(INSERT_VECTOR4)?
            .execute(params![
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
    pub directory_id: String,
    pub file_name: String,
}

impl MatlRecord {
    pub fn create_record(directory_id: String, file_name: String) -> (i64, MatlRecord) {
        let new_id = CURRENT_MATL_ID.fetch_add(1, Ordering::SeqCst);
        (
            new_id as i64,
            MatlRecord {
                directory_id,
                file_name,
            },
        )
    }
}

impl SqlInsert for MatlRecord {
    fn insert(&self, transaction: &mut Transaction) -> Result<()> {
        transaction
            .prepare_cached(INSERT_MATL)?
            .execute(params![self.directory_id, self.file_name])?;
        Ok(())
    }
}

#[derive(Debug)]
pub struct XmbRecord {
    pub directory_id: String,
    pub file_name: String,
}

impl XmbRecord {
    pub fn create_record(directory_id: String, file_name: String) -> (i64, XmbRecord) {
        let new_id = CURRENT_XMB_ID.fetch_add(1, Ordering::SeqCst);
        (
            new_id as i64,
            XmbRecord {
                directory_id,
                file_name,
            },
        )
    }
}

impl SqlInsert for XmbRecord {
    fn insert(&self, transaction: &mut Transaction) -> Result<()> {
        transaction
            .prepare_cached("INSERT INTO Xmb(Directory, FileName) VALUES(?,?)")?
            .execute(params![self.directory_id, self.file_name])?;
        Ok(())
    }
}

#[derive(Debug)]
pub struct XmbEntryRecord {
    pub xmb_id: i64,
    pub name: String,
}

impl XmbEntryRecord {
    pub fn create_record(xmb_id: i64, name: String) -> (i64, XmbEntryRecord) {
        let new_id = CURRENT_XMB_ENTRY_ID.fetch_add(1, Ordering::SeqCst);
        (new_id as i64, XmbEntryRecord { xmb_id, name })
    }
}

impl SqlInsert for XmbEntryRecord {
    fn insert(&self, transaction: &mut Transaction) -> Result<()> {
        transaction
            .prepare_cached("INSERT INTO XmbEntry(XmbID, Name) VALUES(?,?)")?
            .execute(params![self.xmb_id, self.name])?;
        Ok(())
    }
}

#[derive(Debug)]
pub struct XmbAttributeRecord {
    pub xmb_entry_id: i64,
    pub name: String,
    pub value: String,
}

impl SqlInsert for XmbAttributeRecord {
    fn insert(&self, transaction: &mut Transaction) -> Result<()> {
        transaction
            .prepare_cached("INSERT INTO XmbAttribute(XmbEntryID, Name, Value) VALUES(?,?,?)")?
            .execute(params![self.xmb_entry_id, self.name, self.value])?;
        Ok(())
    }
}

#[derive(Debug)]
pub struct MeshRecord {
    pub directory_id: String,
    pub file_name: String,
}

impl MeshRecord {
    pub fn create_record(directory_id: String, file_name: String) -> (i64, MeshRecord) {
        let new_id = CURRENT_MESH_ID.fetch_add(1, Ordering::SeqCst);
        (
            new_id as i64,
            MeshRecord {
                directory_id,
                file_name,
            },
        )
    }
}

impl SqlInsert for MeshRecord {
    fn insert(&self, transaction: &mut Transaction) -> Result<()> {
        transaction
            .prepare_cached(INSERT_MESH)?
            .execute(params![self.directory_id, self.file_name])?;
        Ok(())
    }
}

#[derive(Debug)]
pub struct ModlRecord {
    pub directory_id: String,
    pub file_name: String,
    pub model_file_name: String,
    pub skeleton_file_name: String,
    pub material_file_name: String,
}

impl ModlRecord {
    pub fn create_record(
        directory_id: String,
        file_name: String,
        model_file_name: String,
        skeleton_file_name: String,
        material_file_name: String,
    ) -> ModlRecord {
        ModlRecord {
            directory_id,
            file_name,
            model_file_name,
            skeleton_file_name,
            material_file_name,
        }
    }
}

impl SqlInsert for ModlRecord {
    fn insert(&self, transaction: &mut Transaction) -> Result<()> {
        transaction
            .prepare_cached("INSERT INTO Modl(Directory, FileName, ModelFileName, SkeletonFileName, MaterialFileName) VALUES(?,?,?,?,?)")?
            .execute(params![self.directory_id, self.file_name, self.model_file_name, self.skeleton_file_name, self.material_file_name])?;
        Ok(())
    }
}

#[derive(Debug)]
pub struct MeshObjectRecord {
    pub mesh_id: i64,
    pub mesh_name: String,
    pub sub_index: i64,
}

impl MeshObjectRecord {
    pub fn create_record(
        mesh_id: i64,
        mesh_name: String,
        sub_index: i64,
    ) -> (i64, MeshObjectRecord) {
        let new_id = CURRENT_MESH_OBJECT_ID.fetch_add(1, Ordering::SeqCst);
        (
            new_id as i64,
            MeshObjectRecord {
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
            .prepare_cached(INSERT_MESH_OBJECT)?
            .execute(params![self.mesh_id, self.mesh_name, self.sub_index])?;
        Ok(())
    }
}

#[derive(Debug)]
pub struct MeshAttributeRecord {
    pub mesh_object_id: i64,
    pub attribute_name: String,
}

impl SqlInsert for MeshAttributeRecord {
    fn insert(&self, transaction: &mut Transaction) -> Result<()> {
        transaction
            .prepare_cached(INSERT_MESH_ATTRIBUTE)?
            .execute(params![self.mesh_object_id, self.attribute_name])?;
        Ok(())
    }
}
