use rayon::prelude::*;
use rusqlite::Transaction;
use rusqlite::{params, Connection, Result, NO_PARAMS};
use std::collections::HashMap;
use std::path::Path;
use std::time::Instant;

mod records;
use records::*;

const CUSTOM_PARAM_NAMES: [&str; 366] = [
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

const CREATE_DIRECTORY_TABLE: &str = r#"CREATE TABLE "Directory" (
	"ID"	INTEGER NOT NULL UNIQUE,
	"Path"	TEXT NOT NULL,
	PRIMARY KEY("ID" AUTOINCREMENT)
)"#;

const CREATE_XMB_TABLE: &str = r#"CREATE TABLE "Xmb" (
	"ID"	INTEGER NOT NULL UNIQUE,
	"FileName"	TEXT NOT NULL,
	"DirectoryID"	INTEGER NOT NULL,
	PRIMARY KEY("ID" AUTOINCREMENT),
	FOREIGN KEY("DirectoryID") REFERENCES "Directory"("ID")
)"#;

const CREATE_XMB_ENTRY_TABLE: &str = r#"CREATE TABLE "XmbEntry" (
    "ID"	INTEGER NOT NULL UNIQUE,
    "XmbID" INTEGER NOT NULL,
	"Name"	TEXT NOT NULL,
	PRIMARY KEY("ID" AUTOINCREMENT),
	FOREIGN KEY("XmbID") REFERENCES "Xmb"("ID")
)"#;

const CREATE_XMB_ATTRIBUTE_TABLE: &str = r#"CREATE TABLE "XmbAttribute" (
    "ID"	INTEGER NOT NULL UNIQUE,
    "XmbEntryID" INTEGER NOT NULL,
	"Name"	TEXT NOT NULL,
	"Value"	TEXT NOT NULL,
	PRIMARY KEY("ID" AUTOINCREMENT),
	FOREIGN KEY("XmbEntryID") REFERENCES "XmbEntry"("ID")
)"#;

const CREATE_MODL_TABLE: &str = r#"CREATE TABLE "Modl" (
	"ID"	INTEGER NOT NULL UNIQUE,
    "FileName"	TEXT NOT NULL,
    "ModelFileName" TEXT NOT NULL,
    "SkeletonFileName" TEXT NOT NULL,
    "MaterialFileName" TEXT NOT NULL,
	"DirectoryID"	INTEGER NOT NULL,
	PRIMARY KEY("ID" AUTOINCREMENT),
	FOREIGN KEY("DirectoryID") REFERENCES "Directory"("ID")
)"#;

const CREATE_MESH_TABLE: &str = r#"CREATE TABLE "Mesh" (
	"ID"	INTEGER NOT NULL UNIQUE,
	"FileName"	TEXT NOT NULL,
	"DirectoryID"	INTEGER NOT NULL,
	PRIMARY KEY("ID" AUTOINCREMENT),
	FOREIGN KEY("DirectoryID") REFERENCES "Directory"("ID")
)"#;

const CREATE_MESH_OBJECT_TABLE: &str = r#"CREATE TABLE "MeshObject" (
    "ID"	INTEGER NOT NULL UNIQUE,
    "MeshID" INTEGER NOT NULL,
	"Name"	TEXT NOT NULL,
	"SubIndex"	INTEGER NOT NULL,
	PRIMARY KEY("ID" AUTOINCREMENT),
	FOREIGN KEY("MeshID") REFERENCES "Mesh"("ID")
)"#;

const CREATE_MESH_ATTRIBUTE_TABLE: &str = r#"CREATE TABLE "MeshAttribute" (
    "ID"	INTEGER NOT NULL UNIQUE,
    "MeshObjectID" INTEGER NOT NULL,
	"Name"	TEXT NOT NULL,
	PRIMARY KEY("ID" AUTOINCREMENT),
	FOREIGN KEY("MeshObjectID") REFERENCES "MeshObject"("ID")
)"#;

const CREATE_MATL_TABLE: &str = r#"CREATE TABLE "Matl" (
	"ID"	INTEGER NOT NULL UNIQUE,
	"FileName"	TEXT NOT NULL,
	"DirectoryID"	INTEGER NOT NULL,
	PRIMARY KEY("ID" AUTOINCREMENT),
	FOREIGN KEY("DirectoryID") REFERENCES "Directory"("ID")
)"#;

const CREATE_MATERIAL_TABLE: &str = r#"CREATE TABLE "Material" (
	"ID"	INTEGER NOT NULL UNIQUE,
	"MatlID"	INTEGER NOT NULL,
	"MaterialLabel"	TEXT NOT NULL,
	"ShaderLabel"	TEXT NOT NULL,
	PRIMARY KEY("ID"),
	FOREIGN KEY("MatlID") REFERENCES "Matl"("ID")
)"#;

const CREATE_VECTOR_TABLE: &str = r#"CREATE TABLE "CustomVectorParam" (
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

const CREATE_PARAM_TABLE: &str = r#"CREATE TABLE "CustomParam" (
	"ID"	INTEGER NOT NULL UNIQUE,
	"Name"	TEXT NOT NULL,
	PRIMARY KEY("ID" AUTOINCREMENT)
)"#;

const CREATE_FLOAT_TABLE: &str = r#"CREATE TABLE "CustomFloatParam" (
	"ID"	INTEGER NOT NULL UNIQUE,
	"ParamID"	INTEGER,
	"MaterialID"	INTEGER NOT NULL,
    "Value"	INTEGER NOT NULL,
    FOREIGN KEY("MaterialID")REFERENCES "Material"("ID"),
	FOREIGN KEY("ParamID") REFERENCES "CustomParam"("ID"),
	PRIMARY KEY("ID" AUTOINCREMENT)
)"#;

const CREATE_BOOLEAN_TABLE: &str = r#"CREATE TABLE "CustomBooleanParam" (
	"ID"	INTEGER NOT NULL UNIQUE,
	"ParamID"	INTEGER NOT NULL,
	"MaterialID"	INTEGER NOT NULL,
    "Value"	INTEGER NOT NULL,
    FOREIGN KEY("MaterialID")REFERENCES "Material"("ID"),
	PRIMARY KEY("ID" AUTOINCREMENT),
	FOREIGN KEY("ParamID") REFERENCES "CustomParam"("ID")
)"#;

const CREATE_TEXTURE_TABLE: &str = r#"CREATE TABLE "Texture" (
	"ID"	INTEGER NOT NULL UNIQUE,
	"ParamID"	INTEGER NOT NULL,
	"MaterialID"	INTEGER NOT NULL,
    "Value"	TEXT,
    FOREIGN KEY("MaterialID")REFERENCES "Material"("ID"),
	FOREIGN KEY("ParamID") REFERENCES "CustomParam"("ID"),
	PRIMARY KEY("ID" AUTOINCREMENT)
)"#;

const CREATE_BLENDSTATE_TABLE: &str = r#"CREATE TABLE "BlendState" (
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

const CREATE_RASTERIZERSTATE_TABLE: &str = r#"CREATE TABLE "RasterizerState" (
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

const CREATE_SAMPLER_TABLE: &str = r#"CREATE TABLE "Sampler" (
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

fn create_tables(transaction: &mut Transaction) -> Result<()> {
    transaction.execute(CREATE_PARAM_TABLE, NO_PARAMS)?;
    transaction.execute(CREATE_DIRECTORY_TABLE, NO_PARAMS)?;
    transaction.execute(CREATE_MODL_TABLE, NO_PARAMS)?;
    transaction.execute(CREATE_MESH_TABLE, NO_PARAMS)?;
    transaction.execute(CREATE_MESH_OBJECT_TABLE, NO_PARAMS)?;
    transaction.execute(CREATE_MESH_ATTRIBUTE_TABLE, NO_PARAMS)?;
    transaction.execute(CREATE_MATL_TABLE, NO_PARAMS)?;
    transaction.execute(CREATE_XMB_TABLE, NO_PARAMS)?;
    transaction.execute(CREATE_XMB_ENTRY_TABLE, NO_PARAMS)?;
    transaction.execute(CREATE_XMB_ATTRIBUTE_TABLE, NO_PARAMS)?;
    transaction.execute(CREATE_MATERIAL_TABLE, NO_PARAMS)?;
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
    let mut statement =
        transaction.prepare_cached("INSERT INTO CustomParam(ID,Name) VALUES(?,?)")?;

    for i in 0..CUSTOM_PARAM_NAMES.len() {
        statement.execute(params![i as u32, CUSTOM_PARAM_NAMES[i]])?;
    }

    Ok(())
}

fn process_matl(
    matl: &ssbh_lib::formats::matl::Matl,
    matl_id: &mut i64,
    material_id: &mut i64,
    directory_id: i64,
    file_name: String,
) -> Vec<Box<dyn Insert>> {
    let mut records: Vec<Box<dyn Insert>> = Vec::new();
    records.push(Box::new(MatlRecord {
        directory_id,
        file_name,
    }));
    *matl_id += 1;

    for entry in &matl.entries.elements {
        let material_label = entry.material_label.get_string().unwrap();
        let shader_label = entry.shader_label.get_string().unwrap();
        records.push(Box::new(MaterialRecord {
            matl_id: *matl_id,
            material_label: material_label.to_string(),
            shader_label: shader_label.to_string(),
        }));
        *material_id += 1;

        for attribute in &entry.attributes.elements {
            let param_id = attribute.param_id as u32;

            match &attribute.param.data {
                ssbh_lib::formats::matl::Param::Boolean(val) => {
                    records.push(Box::new(BoolRecord {
                        param_id,
                        material_id: *material_id,
                        value: *val > 0,
                    }));
                }
                ssbh_lib::formats::matl::Param::Float(val) => {
                    records.push(Box::new(FloatRecord {
                        param_id,
                        material_id: *material_id,
                        value: *val as f64,
                    }));
                }
                ssbh_lib::formats::matl::Param::Vector4(val) => {
                    records.push(Box::new(Vector4Record {
                        param_id,
                        material_id: *material_id,
                        x: val.x as f64,
                        y: val.y as f64,
                        z: val.z as f64,
                        w: val.w as f64,
                    }));
                }
                ssbh_lib::formats::matl::Param::MatlString(val) => {
                    let text = val.get_string().unwrap().to_string();
                    records.push(Box::new(TextureRecord {
                        param_id,
                        material_id: *material_id,
                        text,
                    }));
                }
                ssbh_lib::formats::matl::Param::Sampler(val) => {
                    records.push(Box::new(SamplerRecord {
                        param_id,
                        material_id: *material_id,
                        wraps: val.wraps as u32,
                        wrapt: val.wrapt as u32,
                        wrapr: val.wrapr as u32,
                        min_filter: val.min_filter,
                        mag_filter: val.mag_filter,
                        unk6: val.unk6,
                        unk7: val.unk7,
                        unk8: val.unk8,
                        unk9: val.unk9,
                        unk10: val.unk10,
                        unk11: val.unk11,
                        unk12: val.unk12,
                        lod_bias: val.lod_bias as f64,
                        max_anisotropy: val.max_anisotropy,
                    }));
                }
                ssbh_lib::formats::matl::Param::BlendState(val) => {
                    records.push(Box::new(BlendStateRecord {
                        param_id,
                        material_id: *material_id,
                        unk1: val.unk1,
                        unk2: val.unk2,
                        blend_factor1: val.blend_factor1,
                        unk4: val.unk4,
                        unk5: val.unk5,
                        blend_factor2: val.blend_factor2,
                        unk7: val.unk7,
                        unk8: val.unk8,
                        unk9: val.unk9,
                        unk10: val.unk10,
                        unk11: val.unk11,
                        unk12: val.unk12,
                    }));
                }
                ssbh_lib::formats::matl::Param::RasterizerState(val) => {
                    records.push(Box::new(RasterizerRecord {
                        param_id,
                        material_id: *material_id,
                        fill_mode: val.fill_mode as u32,
                        cull_mode: val.cull_mode as u32,
                        depth_bias: val.depth_bias as f64,
                        unk4: val.unk4 as f64,
                        unk5: val.unk5 as f64,
                        unk6: val.unk6,
                        unk7: val.unk7,
                        unk8: val.unk8 as f64,
                    }));
                }
                _ => (),
            }
        }
    }

    records
}

fn process_mesh(
    mesh: &ssbh_lib::formats::mesh::Mesh,
    file_name: &str,
    mesh_id: &mut i64,
    mesh_object_id: &mut i64,
    mesh_attribute_id: &mut i64,
    directory_id: i64,
) -> Vec<Box<dyn Insert>> {
    let mut records: Vec<Box<dyn Insert>> = Vec::new();

    // TODO: Simpler way to ensure the id gets incremented?
    // Combine increment with create method?
    records.push(Box::new(MeshRecord {
        directory_id,
        file_name: file_name.to_string(),
    }));
    *mesh_id += 1;

    for object in &mesh.objects.elements {
        let mesh_name = object.name.get_string().unwrap().to_string();
        let sub_index = object.sub_index;
        records.push(Box::new(MeshObjectRecord {
            mesh_id: *mesh_id,
            mesh_name,
            sub_index,
        }));

        *mesh_object_id += 1;

        for attribute in &object.attributes.elements {
            let attribute_name = attribute.attribute_names.elements[0]
                .get_string()
                .unwrap()
                .to_string();
            records.push(Box::new(MeshAttributeRecord {
                mesh_object_id: *mesh_object_id,
                attribute_name,
            }));
            *mesh_attribute_id += 1;
        }
    }

    records
}

fn process_modl(
    modl: &ssbh_lib::formats::modl::Modl,
    file_name: &str,
    directory_id: i64,
) -> ModlRecord {
    // There could be multiple material filenames but assume just one.
    // Most modl files only reference a single material.
    ModlRecord {
        directory_id,
        file_name: file_name.to_string(),
        model_file_name: modl.model_file_name.get_string().unwrap().to_string(),
        material_file_name: modl.material_file_names.elements[0].get_string().unwrap().to_string(),
        skeleton_file_name: modl.skeleton_file_name.get_string().unwrap().to_string()
    }
}

fn process_xmb(file_name: &str, xmb: &xmb_lib::XmbFile, directory_id: i64,
    current_xmb_id: &mut i64, current_xmb_entry_id: &mut i64, current_xmb_attribute_id: &mut i64) 
-> Vec<Box<dyn Insert>> {
    let mut records: Vec<Box<dyn Insert>> = Vec::new();
    // TODO: Find a way to prevent incrementing the wrong ID.
    // Static get function in records module to prevent outside mutability?
    // Each ::new() function would increment the static ID.
    // Assume there is only a single database with autoincrement primary keys.
    records.push(Box::new(XmbRecord {
        directory_id,
        file_name: file_name.to_string()
    }));
    *current_xmb_id += 1;

    for entry in &xmb.entries {
        records.push(Box::new(XmbEntryRecord {xmb_id: *current_xmb_id, name: entry.name.clone()}));
        *current_xmb_entry_id += 1;

        for attribute in &entry.attributes {
            records.push(Box::new(XmbAttributeRecord {xmb_entry_id: *current_xmb_entry_id, name: attribute.0.clone(), value: attribute.1.clone()}));
            *current_xmb_attribute_id += 1;
        }
    }

    records
}

fn process_ssbh(
    file_name: &str,
    ssbh: &ssbh_lib::Ssbh,
    directory_id: i64,
    matl_id: &mut i64,
    material_id: &mut i64,
    mesh_id: &mut i64,
    mesh_object_id: &mut i64,
    mesh_attribute_id: &mut i64,
) -> Vec<Box<dyn Insert>> {
    match &ssbh.data {
        ssbh_lib::SsbhFile::Matl(matl) => process_matl(
            &matl,
            matl_id,
            material_id,
            directory_id,
            file_name.to_string(),
        ),
        ssbh_lib::SsbhFile::Modl(modl) => {
            // TODO: Finish this method.
            let record = process_modl(&modl, file_name, directory_id);
            vec![Box::new(record)]
        }
        ssbh_lib::SsbhFile::Mesh(mesh) => process_mesh(
            &mesh,
            file_name,
            mesh_id,
            mesh_object_id,
            mesh_attribute_id,
            directory_id,
        ),
        _ => (Vec::<Box<dyn Insert>>::new()),
    }
}

/// Get the row and the inserted record if the path has not been added yet.
fn insert_directory_get_id(
    file_path: &Path,
    source_folder: &Path,
    row_id: i64,
    directory_id_by_path: &mut HashMap<String, i64>,
) -> (i64, Option<DirectoryRecord>) {
    // Only store the in game directory structure.
    // ex: "C:\Users\User\root\...\model.numatb" -> "root\...\model.numatb"
    let folder_path = file_path
        .parent()
        .unwrap()
        .strip_prefix(source_folder)
        .unwrap()
        .to_str()
        .unwrap()
        .to_string();

    match directory_id_by_path.get(&folder_path) {
        Some(directory_id) => (*directory_id, None),
        None => {
            let new_row_id = row_id + 1;
            directory_id_by_path.insert(folder_path.clone(), new_row_id);
            (new_row_id, Some(DirectoryRecord { path: folder_path }))
        }
    }
}

// Convert to Option as a temporary workaround.
// Box<dyn Error> won't work with par_iter.
// TODO: Cleaner handling of errors.
fn parse_ssbh(path: &Path) -> Option<ssbh_lib::Ssbh> {
    match ssbh_lib::read_ssbh(path) {
        Ok(ssbh) => Some(ssbh),
        Err(_) => None,
    }
}

fn parse_xmb(path: &Path) -> Option<xmb_lib::XmbFile> {
    match xmb_lib::read_xmb(path) {
        Ok(xmb) => Some(xmb),
        Err(_) => None,
    }
}

fn get_database_data(
    parsed_files: &Vec<(String, ParsedFile)>,
    source_folder: &Path,
) -> Vec<Box<dyn Insert>> {
    // Simulate an autoincrementing primary key.
    // The first insert will update the value to 1.
    let mut current_matl_id: i64 = 0;
    let mut current_material_id: i64 = 0;
    let mut current_mesh_id: i64 = 0;
    let mut current_mesh_object_id: i64 = 0;
    let mut current_mesh_attribute_id: i64 = 0;
    let mut current_xmb_id: i64 = 0;
    let mut current_xmb_entry_id: i64 = 0;
    let mut current_xmb_attribute_id: i64 = 0;
    let mut current_directory_id: i64 = 0;

    let mut directory_id_by_path = HashMap::new();

    let mut records: Vec<Box<dyn Insert>> = Vec::new();

    for (file_path, parsed_file) in parsed_files {
        // TODO: Move directory processing elsewhere?
        let file_path = Path::new(file_path);
        let (directory_id, directory_record) = insert_directory_get_id(
            file_path,
            source_folder,
            current_directory_id,
            &mut directory_id_by_path,
        );
        current_directory_id = directory_id;

        // Check for directory changes.
        match directory_record {
            Some(record) => records.push(Box::new(record)),
            None => {}
        }

        let file_name = file_path.file_name().unwrap().to_str().unwrap();

        match parsed_file {
            ParsedFile::Ssbh(ssbh) => match ssbh {
                Some(ssbh) => {
                    let mut ssbh_records = process_ssbh(
                        file_name,
                        &ssbh,
                        directory_id,
                        &mut current_matl_id,
                        &mut current_material_id,
                        &mut current_mesh_id,
                        &mut current_mesh_object_id,
                        &mut current_mesh_attribute_id,
                    );
                    records.append(&mut ssbh_records);
                }

                None => continue,
            },
            ParsedFile::Xmb(xmb) => match xmb {
                Some(xmb) => {
                    let mut xmb_records = process_xmb(file_name, &xmb, directory_id, 
                        &mut current_xmb_id, &mut current_xmb_entry_id, &mut current_xmb_attribute_id);
                    records.append(&mut xmb_records);
                }

                None => continue,
            },
        }
    }

    records
}

enum ParsedFile {
    Ssbh(Option<ssbh_lib::Ssbh>),
    Xmb(Option<xmb_lib::XmbFile>),
}

fn process_files(source_folder: &Path, connection: &mut Connection) -> Result<()> {
    // TODO: Additional performance gains?
    let parse_duration = Instant::now();

    // TODO: Combine parsing with record creation.
    // TODO: Don't store all the files in memory.
    let paths = globwalk::GlobWalkerBuilder::from_patterns(
        source_folder,
        &["*.{numatb,numdlb,numshb,xmb}"],
    )
    .build()
    .unwrap()
    .into_iter()
    .filter_map(Result::ok)
    .par_bridge();

    // Parse files in parallel to improve performance.
    // Assume anything other than XMB is one of the SSBH formats.
    let parsed_files: Vec<(String, ParsedFile)> = paths
        .map(|d| {
            let path_string = d.path().to_str().unwrap().to_string();
            match d.path().extension().unwrap().to_str().unwrap() {
                "xmb" => (path_string, ParsedFile::Xmb(parse_xmb(d.path()))),
                _ => (path_string, ParsedFile::Ssbh(parse_ssbh(d.path()))),
            }
        })
        .collect();

    println!(
        "Parse {:?} files: {:?}",
        parsed_files.len(),
        parse_duration.elapsed()
    );

    let create_records = Instant::now();
    let records = get_database_data(&parsed_files, source_folder);
    println!("Create data: {:?}", create_records.elapsed());

    // Perform a single transaction to improve performance.
    // This can only be done from a single thread.
    let database_duration = Instant::now();
    let mut transaction = connection.transaction()?;

    for record in &records {
        record.insert(&mut transaction)?;
    }

    transaction.commit()?;
    println!("Write {} records to database: {:?}", records.len(), database_duration.elapsed());

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
        CREATE INDEX Texture_ParamID_Idx ON Texture(ParamID);",
    )?;

    transaction.commit()
}

fn initialize_database(connection: &mut Connection) -> Result<()> {
    let mut transaction = connection.transaction()?;

    create_tables(&mut transaction)?;
    insert_custom_params(&mut transaction)?;

    transaction.commit()
}

pub fn create_database(source_folder: &Path, database_path: &Path) -> Result<()> {
    let mut connection = Connection::open(database_path)?;

    initialize_database(&mut connection)?;
    process_files(&source_folder, &mut connection)?;
    create_indexes(&mut connection)?;
    
    Ok(())
}
