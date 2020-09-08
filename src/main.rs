use rayon::prelude::*;
use rusqlite::Transaction;
use rusqlite::{params, Connection, Result, NO_PARAMS};
use serde::Serialize;
use serde_rusqlite::*;
use std::collections::HashMap;
use std::env;
use std::error::Error;
use std::fs;
use std::path::Path;
use std::time::Instant;

const CUSTOM_PARAM_NAMES: [&'static str; 366] = [
    "Diffuse",
    "Specular",
    "Ambient",
    "BlendMap",
    "Transparency",
    "DiffuseMapLayer1",
    "CosinePower",
    "SpecularPower",
    "Fresnel",
    "Roughness",
    "EmissiveScale",
    "EnableDiffuse",
    "EnableSpecular",
    "EnableAmbient",
    "DiffuseMapLayer2",
    "EnableTransparency",
    "EnableOpacity",
    "EnableCosinePower",
    "EnableSpecularPower",
    "EnableFresnel",
    "EnableRoughness",
    "EnableEmissiveScale",
    "WorldMatrix",
    "ViewMatrix",
    "ProjectionMatrix",
    "WorldViewMatrix",
    "ViewInverseMatrix",
    "ViewProjectionMatrix",
    "WorldViewProjectionMatrix",
    "WorldInverseTransposeMatrix",
    "DiffuseMap",
    "SpecularMap",
    "AmbientMap",
    "EmissiveMap",
    "SpecularMapLayer1",
    "TransparencyMap",
    "NormalMap",
    "DiffuseCubeMap",
    "ReflectionMap",
    "ReflectionCubeMap",
    "RefractionMap",
    "AmbientOcclusionMap",
    "LightMap",
    "AnisotropicMap",
    "RoughnessMap",
    "ReflectionMask",
    "OpacityMask",
    "UseDiffuseMap",
    "UseSpecularMap",
    "UseAmbientMap",
    "UseEmissiveMap",
    "UseTranslucencyMap",
    "UseTransparencyMap",
    "UseNormalMap",
    "UseDiffuseCubeMap",
    "UseReflectionMap",
    "UseReflectionCubeMap",
    "UseRefractionMap",
    "UseAmbientOcclusionMap",
    "UseLightMap",
    "UseAnisotropicMap",
    "UseRoughnessMap",
    "UseReflectionMask",
    "UseOpacityMask",
    "DiffuseSampler",
    "SpecularSampler",
    "NormalSampler",
    "ReflectionSampler",
    "SpecularMapLayer2",
    "NormalMapLayer1",
    "NormalMapBc5",
    "NormalMapLayer2",
    "RoughnessMapLayer1",
    "RoughnessMapLayer2",
    "UseDiffuseUvTransform1",
    "UseDiffuseUvTransform2",
    "UseSpecularUvTransform1",
    "UseSpecularUvTransform2",
    "UseNormalUvTransform1",
    "UseNormalUvTransform2",
    "ShadowDepthBias",
    "ShadowMap0",
    "ShadowMap1",
    "ShadowMap2",
    "ShadowMap3",
    "ShadowMap4",
    "ShadowMap5",
    "ShadowMap6",
    "ShadowMap7",
    "CastShadow",
    "ReceiveShadow",
    "ShadowMapSampler",
    "Texture0",
    "Texture1",
    "Texture2",
    "Texture3",
    "Texture4",
    "Texture5",
    "Texture6",
    "Texture7",
    "Texture8",
    "Texture9",
    "Texture10",
    "Texture11",
    "Texture12",
    "Texture13",
    "Texture14",
    "Texture15",
    "Sampler0",
    "Sampler1",
    "Sampler2",
    "Sampler3",
    "Sampler4",
    "Sampler5",
    "Sampler6",
    "Sampler7",
    "Sampler8",
    "Sampler9",
    "Sampler10",
    "Sampler11",
    "Sampler12",
    "Sampler13",
    "Sampler14",
    "Sampler15",
    "CustomBuffer0",
    "CustomBuffer1",
    "CustomBuffer2",
    "CustomBuffer3",
    "CustomBuffer4",
    "CustomBuffer5",
    "CustomBuffer6",
    "CustomBuffer7",
    "CustomMatrix0",
    "CustomMatrix1",
    "CustomMatrix2",
    "CustomMatrix3",
    "CustomMatrix4",
    "CustomMatrix5",
    "CustomMatrix6",
    "CustomMatrix7",
    "CustomMatrix8",
    "CustomMatrix9",
    "CustomMatrix10",
    "CustomMatrix11",
    "CustomMatrix12",
    "CustomMatrix13",
    "CustomMatrix14",
    "CustomMatrix15",
    "CustomMatrix16",
    "CustomMatrix17",
    "CustomMatrix18",
    "CustomMatrix19",
    "CustomVector0",
    "CustomVector1",
    "CustomVector2",
    "CustomVector3",
    "CustomVector4",
    "CustomVector5",
    "CustomVector6",
    "CustomVector7",
    "CustomVector8",
    "CustomVector9",
    "CustomVector10",
    "CustomVector11",
    "CustomVector12",
    "CustomVector13",
    "CustomVector14",
    "CustomVector15",
    "CustomVector16",
    "CustomVector17",
    "CustomVector18",
    "CustomVector19",
    "CustomColor0",
    "CustomColor1",
    "CustomColor2",
    "CustomColor3",
    "CustomColor4",
    "CustomColor5",
    "CustomColor6",
    "CustomColor7",
    "CustomColor8",
    "CustomColor9",
    "CustomColor10",
    "CustomColor11",
    "CustomColor12",
    "CustomColor13",
    "CustomColor14",
    "CustomColor15",
    "CustomColor16",
    "CustomColor17",
    "CustomColor18",
    "CustomColor19",
    "CustomFloat0",
    "CustomFloat1",
    "CustomFloat2",
    "CustomFloat3",
    "CustomFloat4",
    "CustomFloat5",
    "CustomFloat6",
    "CustomFloat7",
    "CustomFloat8",
    "CustomFloat9",
    "CustomFloat10",
    "CustomFloat11",
    "CustomFloat12",
    "CustomFloat13",
    "CustomFloat14",
    "CustomFloat15",
    "CustomFloat16",
    "CustomFloat17",
    "CustomFloat18",
    "CustomFloat19",
    "CustomInteger0",
    "CustomInteger1",
    "CustomInteger2",
    "CustomInteger3",
    "CustomInteger4",
    "CustomInteger5",
    "CustomInteger6",
    "CustomInteger7",
    "CustomInteger8",
    "CustomInteger9",
    "CustomInteger10",
    "CustomInteger11",
    "CustomInteger12",
    "CustomInteger13",
    "CustomInteger14",
    "CustomInteger15",
    "CustomInteger16",
    "CustomInteger17",
    "CustomInteger18",
    "CustomInteger19",
    "CustomBoolean0",
    "CustomBoolean1",
    "CustomBoolean2",
    "CustomBoolean3",
    "CustomBoolean4",
    "CustomBoolean5",
    "CustomBoolean6",
    "CustomBoolean7",
    "CustomBoolean8",
    "CustomBoolean9",
    "CustomBoolean10",
    "CustomBoolean11",
    "CustomBoolean12",
    "CustomBoolean13",
    "CustomBoolean14",
    "CustomBoolean15",
    "CustomBoolean16",
    "CustomBoolean17",
    "CustomBoolean18",
    "CustomBoolean19",
    "UvTransform0",
    "UvTransform1",
    "UvTransform2",
    "UvTransform3",
    "UvTransform4",
    "UvTransform5",
    "UvTransform6",
    "UvTransform7",
    "UvTransform8",
    "UvTransform9",
    "UvTransform10",
    "UvTransform11",
    "UvTransform12",
    "UvTransform13",
    "UvTransform14",
    "UvTransform15",
    "DiffuseUvTransform1",
    "DiffuseUvTransform2",
    "SpecularUvTransform1",
    "SpecularUvTransform2",
    "NormalUvTransform1",
    "NormalUvTransform2",
    "DiffuseUvTransform",
    "SpecularUvTransform",
    "NormalUvTransform",
    "UseDiffuseUvTransform",
    "UseSpecularUvTransform",
    "UseNormalUvTransform",
    "BlendState0",
    "BlendState1",
    "BlendState2",
    "BlendState3",
    "BlendState4",
    "BlendState5",
    "BlendState6",
    "BlendState7",
    "BlendState8",
    "BlendState9",
    "BlendState10",
    "RasterizerState0",
    "RasterizerState1",
    "RasterizerState2",
    "RasterizerState3",
    "RasterizerState4",
    "RasterizerState5",
    "RasterizerState6",
    "RasterizerState7",
    "RasterizerState8",
    "RasterizerState9",
    "RasterizerState10",
    "ShadowColor",
    "EmissiveMapLayer1",
    "EmissiveMapLayer2",
    "AlphaTestFunc",
    "AlphaTestRef",
    "Texture16",
    "Texture17",
    "Texture18",
    "Texture19",
    "Sampler16",
    "Sampler17",
    "Sampler18",
    "Sampler19",
    "CustomVector20",
    "CustomVector21",
    "CustomVector22",
    "CustomVector23",
    "CustomVector24",
    "CustomVector25",
    "CustomVector26",
    "CustomVector27",
    "CustomVector28",
    "CustomVector29",
    "CustomVector30",
    "CustomVector31",
    "CustomVector32",
    "CustomVector33",
    "CustomVector34",
    "CustomVector35",
    "CustomVector36",
    "CustomVector37",
    "CustomVector38",
    "CustomVector39",
    "CustomVector40",
    "CustomVector41",
    "CustomVector42",
    "CustomVector43",
    "CustomVector44",
    "CustomVector45",
    "CustomVector46",
    "CustomVector47",
    "CustomVector48",
    "CustomVector49",
    "CustomVector50",
    "CustomVector51",
    "CustomVector52",
    "CustomVector53",
    "CustomVector54",
    "CustomVector55",
    "CustomVector56",
    "CustomVector57",
    "CustomVector58",
    "CustomVector59",
    "CustomVector60",
    "CustomVector61",
    "CustomVector62",
    "CustomVector63",
    "UseBaseColorMap",
    "UseMetallicMap",
    "BaseColorMap",
    "BaseColorMapLayer1",
    "MetallicMap",
    "MetallicMapLayer1",
    "DiffuseLightingAoOffset",
];

static CREATE_DIRECTORY_TABLE: &str = r#"CREATE TABLE "Directory" (
	"ID"	INTEGER NOT NULL UNIQUE,
	"Path"	TEXT NOT NULL,
	PRIMARY KEY("ID" AUTOINCREMENT)
)"#;

static CREATE_XMB_TABLE: &str = r#"CREATE TABLE "Xmb" (
	"ID"	INTEGER NOT NULL UNIQUE,
	"FileName"	TEXT NOT NULL,
	"DirectoryID"	INTEGER NOT NULL,
	PRIMARY KEY("ID" AUTOINCREMENT),
	FOREIGN KEY("DirectoryID") REFERENCES "Directory"("ID")
)"#;

static CREATE_MODL_TABLE: &str = r#"CREATE TABLE "Modl" (
	"ID"	INTEGER NOT NULL UNIQUE,
	"FileName"	TEXT NOT NULL,
	"DirectoryID"	INTEGER NOT NULL,
	PRIMARY KEY("ID" AUTOINCREMENT),
	FOREIGN KEY("DirectoryID") REFERENCES "Directory"("ID")
)"#;

static CREATE_MESH_TABLE: &str = r#"CREATE TABLE "Mesh" (
	"ID"	INTEGER NOT NULL UNIQUE,
	"FileName"	TEXT NOT NULL,
	"DirectoryID"	INTEGER NOT NULL,
	PRIMARY KEY("ID" AUTOINCREMENT),
	FOREIGN KEY("DirectoryID") REFERENCES "Directory"("ID")
)"#;

static CREATE_MESH_OBJECT_TABLE: &str = r#"CREATE TABLE "MeshObject" (
    "ID"	INTEGER NOT NULL UNIQUE,
    "MeshID" INTEGER NOT NULL,
	"Name"	TEXT NOT NULL,
	"SubIndex"	INTEGER NOT NULL,
	PRIMARY KEY("ID" AUTOINCREMENT),
	FOREIGN KEY("MeshID") REFERENCES "Mesh"("ID")
)"#;

static CREATE_MESH_ATTRIBUTE_TABLE: &str = r#"CREATE TABLE "MeshAttribute" (
    "ID"	INTEGER NOT NULL UNIQUE,
    "MeshObjectID" INTEGER NOT NULL,
	"Name"	TEXT NOT NULL,
	PRIMARY KEY("ID" AUTOINCREMENT),
	FOREIGN KEY("MeshObjectID") REFERENCES "MeshObject"("ID")
)"#;

static CREATE_MATL_TABLE: &str = r#"CREATE TABLE "Matl" (
	"ID"	INTEGER NOT NULL UNIQUE,
	"FileName"	TEXT NOT NULL,
	"DirectoryID"	INTEGER NOT NULL,
	PRIMARY KEY("ID" AUTOINCREMENT),
	FOREIGN KEY("DirectoryID") REFERENCES "Directory"("ID")
)"#;

static CREATE_MATERIAL_TABLE: &str = r#"CREATE TABLE "Material" (
	"ID"	INTEGER NOT NULL UNIQUE,
	"MatlID"	INTEGER NOT NULL,
	"MaterialLabel"	TEXT NOT NULL,
	"ShaderLabel"	TEXT NOT NULL,
	PRIMARY KEY("ID"),
	FOREIGN KEY("MatlID") REFERENCES "Matl"("ID")
)"#;

static CREATE_VECTOR_TABLE: &str = r#"CREATE TABLE "CustomVectorParam" (
	"ID"	INTEGER NOT NULL UNIQUE,
	"ParamID"	INTEGER NOT NULL,
	"MaterialID"	INTEGER NOT NULL,
	"Value1"	REAL NOT NULL,
	"Value2"	REAL NOT NULL,
	"Value3"	REAL NOT NULL,
    "Value4"	REAL NOT NULL,
    FOREIGN KEY("MaterialID")REFERENCES "Material"("ID"),
	FOREIGN KEY("ParamID") REFERENCES "CustomParam"("ID"),
	PRIMARY KEY("ID" AUTOINCREMENT)
)"#;

static CREATE_PARAM_TABLE: &str = r#"CREATE TABLE "CustomParam" (
	"ID"	INTEGER NOT NULL UNIQUE,
	"Name"	TEXT NOT NULL,
	PRIMARY KEY("ID" AUTOINCREMENT)
)"#;

static CREATE_FLOAT_TABLE: &str = r#"CREATE TABLE "CustomFloatParam" (
	"ID"	INTEGER NOT NULL UNIQUE,
	"ParamID"	INTEGER,
	"MaterialID"	INTEGER NOT NULL,
    "Value"	INTEGER NOT NULL,
    FOREIGN KEY("MaterialID")REFERENCES "Material"("ID"),
	FOREIGN KEY("ParamID") REFERENCES "CustomParam"("ID"),
	PRIMARY KEY("ID" AUTOINCREMENT)
)"#;

static CREATE_BOOLEAN_TABLE: &str = r#"CREATE TABLE "CustomBooleanParam" (
	"ID"	INTEGER NOT NULL UNIQUE,
	"ParamID"	INTEGER NOT NULL,
	"MaterialID"	INTEGER NOT NULL,
    "Value"	INTEGER NOT NULL,
    FOREIGN KEY("MaterialID")REFERENCES "Material"("ID"),
	PRIMARY KEY("ID" AUTOINCREMENT),
	FOREIGN KEY("ParamID") REFERENCES "CustomParam"("ID")
)"#;

static CREATE_TEXTURE_TABLE: &str = r#"CREATE TABLE "Texture" (
	"ID"	INTEGER NOT NULL UNIQUE,
	"ParamID"	INTEGER NOT NULL,
	"MaterialID"	INTEGER NOT NULL,
    "Value"	TEXT,
    FOREIGN KEY("MaterialID")REFERENCES "Material"("ID"),
	FOREIGN KEY("ParamID") REFERENCES "CustomParam"("ID"),
	PRIMARY KEY("ID" AUTOINCREMENT)
)"#;

static CREATE_BLENDSTATE_TABLE: &str = r#"CREATE TABLE "BlendState" (
	"ID"	INTEGER NOT NULL UNIQUE,
    "ParamID"	INTEGER NOT NULL,
	"MaterialID"	INTEGER NOT NULL,
	"Value1"	INTEGER NOT NULL,
	"Value2"	INTEGER NOT NULL,
	"Value3"	INTEGER NOT NULL,
	"Value4"	INTEGER NOT NULL,
	"Value5"	INTEGER NOT NULL,
	"Value6"	INTEGER NOT NULL,
	"Value7"	INTEGER NOT NULL,
	"Value8"	INTEGER NOT NULL,
	"Value9"	INTEGER NOT NULL,
	"Value10"	INTEGER NOT NULL,
	"Value11"	INTEGER NOT NULL,
    "Value12"	INTEGER NOT NULL,
    FOREIGN KEY("MaterialID")REFERENCES "Material"("ID"),
	FOREIGN KEY("ParamID") REFERENCES "CustomParam"("ID"),
	PRIMARY KEY("ID" AUTOINCREMENT)
)"#;

static CREATE_RASTERIZERSTATE_TABLE: &str = r#"CREATE TABLE "RasterizerState" (
	"ID"	INTEGER NOT NULL UNIQUE,
    "ParamID"	INTEGER NOT NULL,
	"MaterialID"	INTEGER NOT NULL,
	"Value1"	INTEGER NOT NULL,
	"Value2"	INTEGER NOT NULL,
	"Value3"	REAL NOT NULL,
	"Value4"	REAL NOT NULL,
	"Value5"	REAL NOT NULL,
	"Value6"	INTEGER NOT NULL,
	"Value7"	INTEGER NOT NULL,
    "Value8"	REAL NOT NULL,
    FOREIGN KEY("MaterialID")REFERENCES "Material"("ID"),
	FOREIGN KEY("ParamID") REFERENCES "CustomParam"("ID"),
	PRIMARY KEY("ID" AUTOINCREMENT)
)"#;

static CREATE_SAMPLER_TABLE: &str = r#"CREATE TABLE "Sampler" (
	"ID"	INTEGER NOT NULL UNIQUE,
	"ParamID"	INTEGER NOT NULL,
	"MaterialID"	INTEGER NOT NULL,
	"Value1"	INTEGER NOT NULL,
	"Value2"	INTEGER NOT NULL,
	"Value3"	INTEGER NOT NULL,
	"Value4"	INTEGER NOT NULL,
	"Value5"	INTEGER NOT NULL,
	"Value6"	INTEGER NOT NULL,
	"Value7"	INTEGER NOT NULL,
	"Value8"	INTEGER NOT NULL,
	"Value9"	INTEGER NOT NULL,
	"Value10"	INTEGER NOT NULL,
	"Value11"	INTEGER NOT NULL,
	"Value12"	INTEGER NOT NULL,
	"Value13"	REAL NOT NULL,
    "Value14"	INTEGER NOT NULL,
    FOREIGN KEY("MaterialID")REFERENCES "Material"("ID"),
	PRIMARY KEY("ID" AUTOINCREMENT),
	FOREIGN KEY("ParamID") REFERENCES "CustomParam"("ID")
)"#;

// TODO: These can probably be hardcoded into functions.
static INSERT_BOOLEAN: &str =
    "INSERT INTO CustomBooleanParam(ParamID, MaterialID, Value) VALUES(?,?,?)";
static INSERT_FLOAT: &str =
    "INSERT INTO CustomFloatParam(ParamID, MaterialID, Value) VALUES(?,?,?)";
static INSERT_VECTOR4: &str = "INSERT INTO CustomVectorParam(ParamID, MaterialID, Value1, Value2, Value3, Value4) VALUES(?,?,?,?,?,?)";
static INSERT_TEXTURE: &str = "INSERT INTO Texture(ParamID, MaterialID, Value) VALUES(?,?,?)";
static INSERT_SAMPLER: &str = "INSERT INTO Sampler(ParamID, MaterialID, Value1, Value2, Value3, Value4, Value5, Value6, Value7, Value8, Value9, Value10, Value11, Value12, Value13, Value14) VALUES(?,?,?,?,?,?,?,?,?,?,?,?,?,?,?,?)";
static INSERT_RASTERIZER: &str = "INSERT INTO RasterizerState(ParamID, MaterialID, Value1, Value2, Value3, Value4, Value5, Value6, Value7, Value8) VALUES(?,?,?,?,?,?,?,?,?,?)";
static INSERT_BLEND_STATE: &str = "INSERT INTO BlendState(ParamID, MaterialID, Value1, Value2, Value3, Value4, Value5, Value6, Value7, Value8, Value9, Value10, Value11, Value12) VALUES(?,?,?,?,?,?,?,?,?,?,?,?,?,?)";
static INSERT_MATERIAL: &str =
    "INSERT INTO Material(MatlID, MaterialLabel, ShaderLabel) VALUES(?,?,?)";
static INSERT_CUSTOM_PARAM: &str = "INSERT INTO CustomParam(ID,Name) VALUES(?,?)";
static INSERT_DIRECTORY: &str = "INSERT INTO Directory(Path) VALUES (?)";

#[derive(Debug)]
struct Person {
    id: i32,
    name: String,
    data: Option<Vec<u8>>,
}

fn create_tables(transaction: &mut Transaction) -> Result<()> {
    transaction.execute(CREATE_DIRECTORY_TABLE, NO_PARAMS)?;
    transaction.execute(CREATE_MODL_TABLE, NO_PARAMS)?;
    transaction.execute(CREATE_MESH_TABLE, NO_PARAMS)?;
    transaction.execute(CREATE_MESH_OBJECT_TABLE, NO_PARAMS)?;
    transaction.execute(CREATE_MESH_ATTRIBUTE_TABLE, NO_PARAMS)?;
    transaction.execute(CREATE_MATL_TABLE, NO_PARAMS)?;
    transaction.execute(CREATE_XMB_TABLE, NO_PARAMS)?;
    transaction.execute(CREATE_MATERIAL_TABLE, NO_PARAMS)?;
    transaction.execute(CREATE_PARAM_TABLE, NO_PARAMS)?;
    transaction.execute(CREATE_VECTOR_TABLE, NO_PARAMS)?;
    transaction.execute(CREATE_FLOAT_TABLE, NO_PARAMS)?;
    transaction.execute(CREATE_BOOLEAN_TABLE, NO_PARAMS)?;
    transaction.execute(CREATE_TEXTURE_TABLE, NO_PARAMS)?;
    transaction.execute(CREATE_BLENDSTATE_TABLE, NO_PARAMS)?;
    transaction.execute(CREATE_RASTERIZERSTATE_TABLE, NO_PARAMS)?;
    transaction.execute(CREATE_SAMPLER_TABLE, NO_PARAMS)?;
    Ok(())
}

fn insert_custom_params(transaction: &Transaction) -> Result<()> {
    let mut statement = transaction.prepare(INSERT_CUSTOM_PARAM)?;

    for i in 0..CUSTOM_PARAM_NAMES.len() {
        statement.execute(params![i as u32, CUSTOM_PARAM_NAMES[i]])?;
    }

    Ok(())
}

fn execute_many<P: Serialize>(transaction: &Transaction, sql: &str, params: &Vec<P>) -> Result<()> {
    let mut statement = transaction.prepare(sql)?;

    for param in params {
        statement.execute(&to_params(param).unwrap().to_slice())?;
    }

    Ok(())
}

fn process_matl(
    transaction: &mut Transaction,
    matl: &ssbh_lib::formats::matl::Matl,
    file_name: &str,
    directory_id: i64,
) -> Result<()> {
    // TODO: Insert matl get id function.
    transaction.execute(
        "INSERT INTO Matl(DirectoryID, FileName) VALUES(?,?)",
        params![directory_id, file_name,],
    )?;
    let matl_id = transaction.last_insert_rowid();

    let mut booleans = Vec::new();
    let mut floats = Vec::new();
    let mut rasterizers = Vec::new();
    let mut textures = Vec::new();
    let mut vec4s = Vec::new();
    let mut samplers = Vec::new();
    let mut blendstates = Vec::new();

    for entry in &matl.entries.elements {
        // TODO: Insert material get id function.
        let material_label = entry.material_label.get_string().unwrap();
        let shader_label = entry.shader_label.get_string().unwrap();
        transaction.execute(
            INSERT_MATERIAL,
            params![matl_id, material_label, shader_label],
        )?;
        let material_id = transaction.last_insert_rowid();

        for attribute in &entry.attributes.elements {
            let param_id = attribute.param_id as u32;

            match &attribute.param.data {
                ssbh_lib::formats::matl::Param::Boolean(val) => {
                    booleans.push((param_id, material_id, val));
                }
                ssbh_lib::formats::matl::Param::Float(val) => {
                    floats.push((param_id, material_id, *val as f64));
                }
                ssbh_lib::formats::matl::Param::Vector4(val) => {
                    vec4s.push((
                        param_id,
                        material_id,
                        val.x as f64,
                        val.y as f64,
                        val.z as f64,
                        val.w as f64,
                    ));
                }
                ssbh_lib::formats::matl::Param::MatlString(val) => {
                    let text = val.get_string().unwrap();
                    textures.push((param_id, material_id, text));
                }
                ssbh_lib::formats::matl::Param::Sampler(val) => {
                    samplers.push((
                        param_id,
                        material_id,
                        val.wraps as u32,
                        val.wrapt as u32,
                        val.wrapr as u32,
                        val.min_filter,
                        val.mag_filter,
                        val.unk6,
                        val.unk7,
                        val.unk8,
                        val.unk9,
                        val.unk10,
                        val.unk11,
                        val.unk12,
                        val.lod_bias as f64,
                        val.max_anisotropy,
                    ));
                }
                ssbh_lib::formats::matl::Param::BlendState(val) => {
                    blendstates.push((
                        param_id,
                        material_id,
                        val.unk1,
                        val.unk2,
                        val.blend_factor1,
                        val.unk4,
                        val.unk5,
                        val.blend_factor2,
                        val.unk7,
                        val.unk8,
                        val.unk9,
                        val.unk10,
                        val.unk11,
                        val.unk12,
                    ));
                }
                ssbh_lib::formats::matl::Param::RasterizerState(val) => {
                    rasterizers.push((
                        param_id,
                        material_id,
                        val.fill_mode as u32,
                        val.cull_mode as u32,
                        val.depth_bias as f64,
                        val.unk4 as f64,
                        val.unk5 as f64,
                        val.unk6,
                        val.unk7,
                        val.unk8 as f64,
                    ));
                }
                _ => (),
            }
        }
    }

    execute_many(&transaction, INSERT_BOOLEAN, &booleans)?;
    execute_many(&transaction, INSERT_FLOAT, &floats)?;
    execute_many(&transaction, INSERT_RASTERIZER, &rasterizers)?;
    execute_many(&transaction, INSERT_TEXTURE, &textures)?;
    execute_many(&transaction, INSERT_VECTOR4, &vec4s)?;
    execute_many(&transaction, INSERT_SAMPLER, &samplers)?;
    execute_many(&transaction, INSERT_BLEND_STATE, &blendstates)?;
    Ok(())
}

fn process_mesh(
    transaction: &mut Transaction,
    mesh: &ssbh_lib::formats::mesh::Mesh,
    file_name: &str,
    directory_id: i64,
) -> Result<()> {
    transaction.execute(
        "INSERT INTO Mesh(DirectoryID, FileName) VALUES(?,?)",
        params![directory_id, file_name,],
    )?;
    let mesh_id = transaction.last_insert_rowid();

    for object in &mesh.objects.elements {
        let mesh_name = object.name.get_string();
        let sub_index = object.sub_index;
        transaction.execute(
            "INSERT INTO MeshObject(MeshID, Name, SubIndex) VALUES(?,?,?)",
            params![mesh_id, mesh_name, sub_index],
        )?;
        let mesh_object_id = transaction.last_insert_rowid();

        for attribute in &object.attributes.elements {
            let attribute_name = attribute.attribute_names.elements[0].get_string();
            transaction.execute(
                "INSERT INTO MeshAttribute(MeshObjectID, Name) VALUES(?,?)",
                params![mesh_object_id, attribute_name],
            )?;
        }
    }
    Ok(())
}

fn process_modl(
    transaction: &mut Transaction,
    modl: &ssbh_lib::formats::modl::Modl,
    file_name: &str,
    directory_id: i64,
) -> Result<()> {
    transaction.execute(
        "INSERT INTO Modl(DirectoryID, FileName) VALUES(?,?)",
        params![directory_id, file_name,],
    )?;
    // TODO: Add modl data.
    Ok(())
}

fn process_xmb(
    transaction: &mut Transaction,
    xmb: &xmb_lib::XmbFile,
    file_name: &str,
    directory_id: i64,
) -> Result<()> {
    // TODO: Add xmb entry data.
    transaction.execute(
        "INSERT INTO Xmb(DirectoryID, FileName) VALUES(?,?)",
        params![directory_id, file_name,],
    )?;
    Ok(())
}

fn process_ssbh(
    transaction: &mut Transaction,
    file_name: &str,
    ssbh: &ssbh_lib::Ssbh,
    directory_id: i64,
) -> Result<(), Box<dyn Error>> {
    match &ssbh.data {
        ssbh_lib::SsbhFile::Matl(matl) => {
            process_matl(transaction, &matl, file_name, directory_id)?;
        }
        ssbh_lib::SsbhFile::Modl(modl) => {
            process_modl(transaction, &modl, file_name, directory_id)?;
        }
        ssbh_lib::SsbhFile::Mesh(mesh) => {
            process_mesh(transaction, &mesh, file_name, directory_id)?;
        }
        _ => (),
    }
    Ok(())
}

fn insert_directory_get_id(
    transaction: &Transaction,
    file_path: &Path,
    source_folder: &Path,
    inserted_folders: &mut HashMap<String, i64>,
) -> i64 {
    let folder_path = file_path
        .parent()
        .unwrap()
        .strip_prefix(source_folder)
        .unwrap()
        .to_str()
        .unwrap()
        .to_string();

    match inserted_folders.get(&folder_path) {
        Some(directory_id) => *directory_id,
        None => {
            transaction
                .execute(INSERT_DIRECTORY, params![folder_path])
                .unwrap();
            let row_id = transaction.last_insert_rowid();
            inserted_folders.insert(folder_path, row_id);
            row_id
        }
    }
}

// Convert to Option as a temporary workaround.
// Box<dyn Error> won't work with par_iter.
fn parse_ssbh(path: &Path) -> Option<ssbh_lib::Ssbh> {
    match ssbh_lib::read_ssbh(path) {
        Ok(ssbh) => Some(ssbh),
        Err(_) => None
    }
}

fn parse_xmb(path: &Path) -> Option<xmb_lib::XmbFile> {
    match xmb_lib::read_xmb(path) {
        Ok(xmb) => Some(xmb),
        Err(_) => None
    }
}

fn write_xmb_data(
    xmb_files: &Vec<(&Path, Option<xmb_lib::XmbFile>)>,
    transaction: &mut Transaction,
    source_folder: &Path,
    directory_id_by_path: &mut HashMap<String, i64>,
) -> Result<()> {
    for (file_path, xmb) in xmb_files {
        let directory_id =
            insert_directory_get_id(transaction, file_path, source_folder, directory_id_by_path);

        let file_name = file_path.file_name().unwrap().to_str().unwrap();

        match xmb {
            Some(xmb) => process_xmb(transaction, &xmb, file_name, directory_id)?,
            None => continue,
        }
    }
    Ok(())
}

fn write_ssbh_data(
    ssbh_files: &Vec<(&Path, Option<ssbh_lib::Ssbh>)>,
    transaction: &mut Transaction,
    source_folder: &Path,
    directory_id_by_path: &mut HashMap<String, i64>,
) -> Result<(), Box<dyn Error>> {
    for (file_path, ssbh) in ssbh_files {
        let directory_id = insert_directory_get_id(
            &transaction,
            file_path,
            source_folder,
            directory_id_by_path,
        );

        let file_name = file_path.file_name().unwrap().to_str().unwrap();

        match ssbh {
            Some(ssbh) => process_ssbh(transaction, file_name, &ssbh, directory_id)?,
            None => continue,
        }
    }
    Ok(())
}

fn process_files(source_folder: &Path, connection: &mut Connection) -> Result<(), Box<dyn Error>> {
    // TODO: Additional performance gains?
    // Parse files in parallel to improve performance.

    // TODO: Can these iterators be combined?
    let xmb_duration = Instant::now();
    let xmb_paths: Vec<_> = globwalk::GlobWalkerBuilder::from_patterns(source_folder, &["*.xmb"])
        .build()
        .unwrap()
        .into_iter()
        .filter_map(Result::ok)
        .collect();
    let xmb_files: Vec<(&Path, Option<xmb_lib::XmbFile>)> = xmb_paths
        .par_iter()
        .map(|f| (f.path(), parse_xmb(f.path())))
        .collect();       
    println!("Parse {:?} XMB files: {:?}", xmb_files.len(), xmb_duration.elapsed());

    let ssbh_duration = Instant::now();
    let ssbh_paths: Vec<_> =
        globwalk::GlobWalkerBuilder::from_patterns(source_folder, &["*.{numatb,numdlb,numshb}"])
            .build()
            .unwrap()
            .into_iter()
            .filter_map(Result::ok)
            .collect();
    let ssbh_files: Vec<(&Path, Option<ssbh_lib::Ssbh>)> = ssbh_paths
        .par_iter()
        .map(|f| (f.path(), parse_ssbh(f.path())))
        .collect();
    println!("Parse {:?} SSBH files: {:?}", ssbh_files.len(), ssbh_duration.elapsed());

    let mut directory_id_by_path = HashMap::new();

    // Perform a single transaction to improve performance.
    // This can only be done from a single thread.
    let mut transaction = connection.transaction()?;

    let database_duration = Instant::now();
    write_xmb_data(
        &xmb_files,
        &mut transaction,
        source_folder,
        &mut directory_id_by_path,
    )?;

    write_ssbh_data(
        &ssbh_files,
        &mut transaction,
        source_folder,
        &mut directory_id_by_path,
    )?;
    transaction.commit()?;
    println!("Write to database: {:?}", database_duration.elapsed());

    Ok(())
}

fn create_indexes(connection: &mut Connection) -> Result<()> {
    let transaction = connection.transaction()?;

    // Create indexes to optimize only the more commonly specified parameters.
    transaction.execute_batch(
   "CREATE INDEX BlendState_MaterialID_Idx ON BlendState(MaterialID);
        CREATE INDEX CustomBooleanParam_MaterialID_Idx ON CustomBooleanParam(MaterialID);
        CREATE INDEX CustomFloatParam_MaterialID_Idx ON CustomFloatParam(MaterialID);
        CREATE INDEX CustomVectorParam_MaterialID_Idx ON CustomVectorParam(MaterialID);
        CREATE INDEX RasterizerState_MaterialID_Idx ON RasterizerState(MaterialID);
        CREATE INDEX Sampler_MaterialID_Idx ON Sampler(MaterialID);
        CREATE INDEX Texture_MaterialID_Idx ON Texture(MaterialID);
        CREATE INDEX BlendState_ParamID_Idx ON BlendState(ParamID);
        CREATE INDEX CustomBooleanParam_ParamID_Idx ON CustomBooleanParam(ParamID);
        CREATE INDEX CustomFloatParam_ParamID_Idx ON CustomFloatParam(ParamID);
        CREATE INDEX CustomVectorParam_ParamID_Idx ON CustomVectorParam(ParamID);
        CREATE INDEX RasterizerState_ParamID_Idx ON RasterizerState(ParamID);
        CREATE INDEX Sampler_ParamID_Idx ON Sampler(ParamID);
        CREATE INDEX Texture_ParamID_Idx ON Texture(ParamID);")?;

    transaction.commit()
}

fn initialize_database(connection: &mut Connection) -> Result<()> {
    let mut transaction = connection.transaction()?;

    create_tables(&mut transaction)?;
    insert_custom_params(&mut transaction)?;

    transaction.commit()
}

fn main() {
    let args: Vec<String> = env::args().collect();
    if args.len() != 3 {
        println!("Usage: smush_material_db.exe <source folder> <output SQLite database>");
        return;
    }
    let source_folder = Path::new(&args[1]);
    let database_path = Path::new(&args[2]);

    // Overwrite the file if it already exists.
    if database_path.exists() {
        fs::remove_file(database_path).unwrap();
    }

    let duration = Instant::now();

    // TODO: Move database/SQL code to lib.rs.
    let mut connection = Connection::open(database_path).unwrap();

    initialize_database(&mut connection).unwrap();
    process_files(&source_folder, &mut connection).unwrap();
    create_indexes(&mut connection).unwrap();

    println!("Total: {:?}", duration.elapsed());
}
