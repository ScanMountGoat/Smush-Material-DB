use rayon::prelude::*;
use rusqlite::Transaction;
use rusqlite::{params, Connection, Result, NO_PARAMS};
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

const CREATE_XMB_TABLE: &str = r#"CREATE TABLE "Xmb" (
	"Id"	INTEGER NOT NULL UNIQUE,
	"FileName"	TEXT NOT NULL,
	"Directory"	TEXT NOT NULL,
	PRIMARY KEY("Id")
)"#;

const CREATE_XMB_ENTRY_TABLE: &str = r#"CREATE TABLE "XmbEntry" (
    "Id"	INTEGER NOT NULL UNIQUE,
    "XmbId" INTEGER NOT NULL,
    "Name"	TEXT NOT NULL,
    FOREIGN KEY("XmbId") REFERENCES "Xmb"("Id")
	PRIMARY KEY("Id")
)"#;

const CREATE_XMB_ATTRIBUTE_TABLE: &str = r#"CREATE TABLE "XmbAttribute" (
    "Id"	INTEGER NOT NULL UNIQUE,
    "XmbEntryId" INTEGER NOT NULL,
	"Name"	TEXT NOT NULL,
    "Value"	TEXT NOT NULL,
    FOREIGN KEY("XmbEntryId") REFERENCES "XmbEntry"("Id")
	PRIMARY KEY("Id")
)"#;

const CREATE_MODL_TABLE: &str = r#"CREATE TABLE "Modl" (
	"Id"	INTEGER NOT NULL UNIQUE,
    "FileName"	TEXT NOT NULL,
    "ModelFileName" TEXT NOT NULL,
    "SkeletonFileName" TEXT NOT NULL,
    "MaterialFileName" TEXT NOT NULL,
	"Directory"	TEXT NOT NULL,
	PRIMARY KEY("Id")
)"#;

const CREATE_MESH_TABLE: &str = r#"CREATE TABLE "Mesh" (
	"Id"	INTEGER NOT NULL UNIQUE,
	"FileName"	TEXT NOT NULL,
	"Directory"	TEXT NOT NULL,
	PRIMARY KEY("Id")
)"#;

const CREATE_MESH_OBJECT_TABLE: &str = r#"CREATE TABLE "MeshObject" (
    "Id"	INTEGER NOT NULL UNIQUE,
    "MeshId" INTEGER NOT NULL,
	"Name"	TEXT NOT NULL,
    "SubIndex"	INTEGER NOT NULL,
    FOREIGN KEY("MeshId") REFERENCES "Mesh"("Id")
	PRIMARY KEY("Id")
)"#;

const CREATE_MESH_ATTRIBUTE_TABLE: &str = r#"CREATE TABLE "MeshAttribute" (
    "Id"	INTEGER NOT NULL UNIQUE,
    "MeshObjectId" INTEGER NOT NULL,
    "Name"	TEXT NOT NULL,
    FOREIGN KEY("MeshObjectId") REFERENCES "MeshObject"("Id")
	PRIMARY KEY("Id")
)"#;

const CREATE_MATL_TABLE: &str = r#"CREATE TABLE "Matl" (
	"Id"	INTEGER NOT NULL UNIQUE,
	"FileName"	TEXT NOT NULL,
	"Directory"	TEXT NOT NULL,
	PRIMARY KEY("Id")
)"#;

const CREATE_MATERIAL_TABLE: &str = r#"CREATE TABLE "Material" (
	"Id"	INTEGER NOT NULL UNIQUE,
	"MatlId"	INTEGER NOT NULL,
	"MaterialLabel"	TEXT NOT NULL,
    "ShaderLabel"	TEXT NOT NULL,
    FOREIGN KEY("MatlId") REFERENCES "Matl"("Id")
	PRIMARY KEY("Id")
)"#;

const CREATE_VECTOR_TABLE: &str = r#"CREATE TABLE "CustomVectorParam" (
	"Id"	INTEGER NOT NULL UNIQUE,
	"ParamId"	INTEGER NOT NULL,
	"MaterialId"	INTEGER NOT NULL,
	"X"	REAL NOT NULL,
	"Y"	REAL NOT NULL,
	"Z"	REAL NOT NULL,
    "W"	REAL NOT NULL,
    FOREIGN KEY("MaterialId") REFERENCES "Material"("Id"),
	FOREIGN KEY("ParamId") REFERENCES "CustomParam"("Id")
	PRIMARY KEY("Id")
)"#;

const CREATE_PARAM_TABLE: &str = r#"CREATE TABLE "CustomParam" (
	"Id"	INTEGER NOT NULL UNIQUE,
	"Name"	TEXT NOT NULL,
	PRIMARY KEY("Id")
)"#;

const CREATE_FLOAT_TABLE: &str = r#"CREATE TABLE "CustomFloatParam" (
	"Id"	INTEGER NOT NULL UNIQUE,
	"ParamId"	INTEGER,
	"MaterialId"	INTEGER NOT NULL,
    "Value"	INTEGER NOT NULL,
    FOREIGN KEY("MaterialId") REFERENCES "Material"("Id"),
	FOREIGN KEY("ParamId") REFERENCES "CustomParam"("Id")
	PRIMARY KEY("Id")
)"#;

const CREATE_BOOLEAN_TABLE: &str = r#"CREATE TABLE "CustomBooleanParam" (
	"Id"	INTEGER NOT NULL UNIQUE,
	"ParamId"	INTEGER NOT NULL,
	"MaterialId"	INTEGER NOT NULL,
    "Value"	INTEGER NOT NULL,
    PRIMARY KEY("Id"),
    FOREIGN KEY("MaterialId") REFERENCES "Material"("Id"),
	FOREIGN KEY("ParamId") REFERENCES "CustomParam"("Id")
)"#;

const CREATE_TEXTURE_TABLE: &str = r#"CREATE TABLE "Texture" (
	"Id"	INTEGER NOT NULL UNIQUE,
	"ParamId"	INTEGER NOT NULL,
	"MaterialId"	INTEGER NOT NULL,
    "Value"	TEXT,
    FOREIGN KEY("MaterialId") REFERENCES "Material"("Id"),
	FOREIGN KEY("ParamId") REFERENCES "CustomParam"("Id"),
	PRIMARY KEY("Id")
)"#;

const CREATE_BLENDSTATE_TABLE: &str = r#"CREATE TABLE "BlendState" (
	"Id"	INTEGER NOT NULL UNIQUE,
    "ParamId"	INTEGER NOT NULL,
	"MaterialId"	INTEGER NOT NULL,
	"SourceColor"	INTEGER NOT NULL,
	"Unk2"	INTEGER NOT NULL,
	"DestinationColor"	INTEGER NOT NULL,
	"Unk4"	INTEGER NOT NULL,
	"Unk5"	INTEGER NOT NULL,
	"Unk6"	INTEGER NOT NULL,
	"Unk7"	INTEGER NOT NULL,
	"Unk8"	INTEGER NOT NULL,
	"Unk9"	INTEGER NOT NULL,
	"Unk10"	INTEGER NOT NULL,
    FOREIGN KEY("MaterialId") REFERENCES "Material"("Id"),
	FOREIGN KEY("ParamId") REFERENCES "CustomParam"("Id"),
	PRIMARY KEY("Id")
)"#;

const CREATE_RASTERIZERSTATE_TABLE: &str = r#"CREATE TABLE "RasterizerState" (
	"Id"	INTEGER NOT NULL UNIQUE,
    "ParamId"	INTEGER NOT NULL,
	"MaterialId"	INTEGER NOT NULL,
	"FillMode"	INTEGER NOT NULL,
	"CullMode"	INTEGER NOT NULL,
	"DepthBias"	REAL NOT NULL,
	"Unk4"	REAL NOT NULL,
	"Unk5"	REAL NOT NULL,
	"Unk6"	INTEGER NOT NULL,
    FOREIGN KEY("MaterialId") REFERENCES "Material"("Id"),
	FOREIGN KEY("ParamId") REFERENCES "CustomParam"("Id"),
	PRIMARY KEY("Id")
)"#;

const CREATE_SAMPLER_TABLE: &str = r#"CREATE TABLE "Sampler" (
	"Id"	INTEGER NOT NULL UNIQUE,
	"ParamId"	INTEGER NOT NULL,
	"MaterialId"	INTEGER NOT NULL,
	"Wraps"	INTEGER NOT NULL,
	"Wrapt"	INTEGER NOT NULL,
	"Wrapr"	INTEGER NOT NULL,
	"MinFilter"	INTEGER NOT NULL,
	"MagFilter"	INTEGER NOT NULL,
	"TextureFilteringType"	INTEGER NOT NULL,
	"BorderColorR"	Real NOT NULL,
	"BorderColorG"	Real NOT NULL,
	"BorderColorB"	Real NOT NULL,
	"BorderColorA"	Real NOT NULL,
	"Unk11"	INTEGER NOT NULL,
	"Unk12"	INTEGER NOT NULL,
	"LodBias"	REAL NOT NULL,
    "MaxAnisotropy"	INTEGER NOT NULL,
    PRIMARY KEY("Id"),
    FOREIGN KEY("MaterialId") REFERENCES "Material"("Id"),
	FOREIGN KEY("ParamId") REFERENCES "CustomParam"("Id")
)"#;

fn create_tables(transaction: &mut Transaction) -> Result<()> {
    transaction.execute(CREATE_PARAM_TABLE, NO_PARAMS)?;
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
        transaction.prepare_cached("INSERT INTO CustomParam(Id,Name) VALUES(?,?)")?;

    for i in 0..CUSTOM_PARAM_NAMES.len() {
        statement.execute(params![i as u32, CUSTOM_PARAM_NAMES[i]])?;
    }

    Ok(())
}

fn process_matl(
    matl: &ssbh_lib::formats::matl::Matl,
    directory_id: String,
    file_name: String,
) -> Vec<Box<dyn SqlInsert>> {
    let mut records: Vec<Box<dyn SqlInsert>> = Vec::new();

    let (matl_id, matl_record) = MatlRecord::create_record(directory_id, file_name);
    records.push(Box::new(matl_record));

    for entry in &matl.entries.elements {
        let material_label = entry.material_label.get_string().unwrap();
        let shader_label = entry.shader_label.get_string().unwrap();

        let (material_id, material_record) = MaterialRecord::create_record(
            matl_id,
            material_label.to_string(),
            shader_label.to_string(),
        );
        records.push(Box::new(material_record));

        for attribute in &entry.attributes.elements {
            let param_id = attribute.param_id as u32;

            match &(*attribute.param.data) {
                Some(data) => {
                    match data {
                        ssbh_lib::formats::matl::Param::Boolean(val) => {
                            records.push(Box::new(
                                BoolRecord::create_record(param_id, material_id, *val > 0).1,
                            ));
                        }
                        ssbh_lib::formats::matl::Param::Float(val) => {
                            records.push(Box::new(
                                FloatRecord::create_record(param_id, material_id, *val as f64).1,
                            ));
                        }
                        ssbh_lib::formats::matl::Param::Vector4(val) => {
                            records.push(Box::new(
                                Vector4Record::create_record(
                                    param_id,
                                    material_id,
                                    val.x as f64,
                                    val.y as f64,
                                    val.z as f64,
                                    val.w as f64,
                                )
                                .1,
                            ));
                        }
                        ssbh_lib::formats::matl::Param::MatlString(val) => {
                            let text = val.get_string().unwrap().to_string();
                            records.push(Box::new(
                                TextureRecord::create_record(param_id, material_id, text).1,
                            ));
                        }
                        ssbh_lib::formats::matl::Param::Sampler(val) => {
                            records.push(Box::new(
                                SamplerRecord::create_record(
                                    param_id,
                                    material_id,
                                    val.wraps as u32,
                                    val.wrapt as u32,
                                    val.wrapr as u32,
                                    val.min_filter as u32,
                                    val.mag_filter as u32,
                                    val.texture_filtering_type as u32,
                                    val.border_color.r,
                                    val.border_color.g,
                                    val.border_color.b,
                                    val.border_color.a,
                                    val.unk11,
                                    val.unk12,
                                    val.lod_bias as f64,
                                    val.max_anisotropy,
                                )
                                .1,
                            ));
                        }
                        ssbh_lib::formats::matl::Param::BlendState(val) => {
                            records.push(Box::new(
                                BlendStateRecord::create_record(
                                    param_id,
                                    material_id,
                                    val.source_color as u32,
                                    val.unk2,
                                    val.destination_color as u32,
                                    val.unk4,
                                    val.unk5,
                                    val.unk6,
                                    val.unk7,
                                    val.unk8,
                                    val.unk9,
                                    val.unk10
                                )
                                .1,
                            ));
                        }
                        ssbh_lib::formats::matl::Param::RasterizerState(val) => {
                            records.push(Box::new(
                                RasterizerRecord::create_record(
                                    param_id,
                                    material_id,
                                    val.fill_mode as u32,
                                    val.cull_mode as u32,
                                    val.depth_bias as f64,
                                    val.unk4 as f64,
                                    val.unk5 as f64,
                                    val.unk6,
                                )
                                .1,
                            ));
                        }
                        _ => (),
                    }
                }
                None => ()
            }
            
        }
    }

    records
}

fn process_mesh(
    mesh: &ssbh_lib::formats::mesh::Mesh,
    file_name: &str,
    directory_id: String,
) -> Vec<Box<dyn SqlInsert>> {
    let mut records: Vec<Box<dyn SqlInsert>> = Vec::new();

    let (mesh_id, mesh_record) = MeshRecord::create_record(directory_id, file_name.to_string());
    records.push(Box::new(mesh_record));

    for object in &mesh.objects.elements {
        let mesh_name = object.name.get_string().unwrap().to_string();
        let sub_index = object.sub_index;

        let (mesh_object_id, mesh_object_record) =
            MeshObjectRecord::create_record(mesh_id, mesh_name, sub_index as i64);
        records.push(Box::new(mesh_object_record));

        // Only version 1.10 has attribute names. 
        // There are a small number of 1.8 meshes, so ignore them for now.
        match &object.attributes {
            ssbh_lib::formats::mesh::MeshAttributes::AttributesV10(v) => {
                for attribute in &v.elements {
                    let attribute_name = attribute.attribute_names.elements[0]
                        .get_string()
                        .unwrap()
                        .to_string();
                    records.push(Box::new(
                        MeshAttributeRecord::create_record(mesh_object_id, attribute_name).1,
                    ));
                }
            }
            _ => ()
        }

    }

    records
}

fn process_modl(
    modl: &ssbh_lib::formats::modl::Modl,
    file_name: &str,
    directory_id: String,
) -> ModlRecord {
    // There could be multiple material filenames but assume just one.
    // Most modl files only reference a single material.
    ModlRecord::create_record(
        directory_id,
        file_name.to_string(),
        modl.model_file_name.get_string().unwrap().to_string(),
        modl.skeleton_file_name.get_string().unwrap().to_string(),
        modl.material_file_names.elements[0]
            .get_string()
            .unwrap()
            .to_string(),
    )
    .1
}

fn process_xmb(
    file_name: &str,
    xmb: &xmb_lib::XmbFile,
    directory: String,
) -> Vec<Box<dyn SqlInsert>> {
    let mut records: Vec<Box<dyn SqlInsert>> = Vec::new();

    let (xmb_id, xmb_record) = XmbRecord::create_record(directory, file_name.to_string());
    records.push(Box::new(xmb_record));

    for entry in &xmb.entries {
        let (xmb_entry_id, entry_record) =
            XmbEntryRecord::create_record(xmb_id, entry.name.clone());
        records.push(Box::new(entry_record));

        for attribute in &entry.attributes {
            records.push(Box::new(
                XmbAttributeRecord::create_record(
                    xmb_entry_id,
                    attribute.0.clone(),
                    attribute.1.clone(),
                )
                .1,
            ));
        }
    }

    records
}

fn process_ssbh(
    file_name: &str,
    ssbh: &ssbh_lib::Ssbh,
    directory: String,
) -> Vec<Box<dyn SqlInsert>> {
    match &ssbh.data {
        ssbh_lib::SsbhFile::Matl(matl) => process_matl(&matl, directory, file_name.to_string()),
        ssbh_lib::SsbhFile::Modl(modl) => {
            let record = process_modl(&modl, file_name, directory);
            vec![Box::new(record)]
        }
        ssbh_lib::SsbhFile::Mesh(mesh) => process_mesh(&mesh, file_name, directory),
        _ => (Vec::<Box<dyn SqlInsert>>::new()),
    }
}

/// Get the row and the inserted record if the path has not been added yet.
fn get_directory(file_path: &Path, source_folder: &Path) -> String {
    // Only store the in game directory structure.
    // ex: "C:\Users\User\root\...\model.numatb" -> "root\...\model.numatb"
    file_path
        .parent()
        .unwrap()
        .strip_prefix(source_folder)
        .unwrap()
        .to_str()
        .unwrap()
        .to_string()
}

// Convert to Option as a temporary workaround.
// Box<dyn Error> won't work with par_iter.
// TODO: Cleaner handling of errors.
fn parse_ssbh(path: &Path) -> Option<ssbh_lib::Ssbh> {
    match ssbh_lib::Ssbh::from_file(path) {
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

fn get_records(file_path: &Path, source_folder: &Path) -> Vec<Box<dyn SqlInsert>> {
    let file_name = file_path.file_name().unwrap().to_str().unwrap();
    let extension = file_path.extension().unwrap().to_str().unwrap();

    let mut records: Vec<Box<dyn SqlInsert>> = Vec::new();

    let directory = get_directory(file_path, source_folder);

    // Assume files that are not XMB files are SSBH.
    match extension {
        "xmb" => match parse_xmb(file_path) {
            Some(xmb) => {
                let mut xmb_records = process_xmb(file_name, &xmb, directory);
                records.append(&mut xmb_records);
            }

            None => {}
        },
        _ => match parse_ssbh(file_path) {
            Some(ssbh) => {
                let mut ssbh_records = process_ssbh(file_name, &ssbh, directory);
                records.append(&mut ssbh_records);
            }

            None => {}
        },
    }

    records
}

fn process_files(source_folder: &Path, connection: &mut Connection) -> Result<()> {
    let parse_duration = Instant::now();

    let paths_iter = globwalk::GlobWalkerBuilder::from_patterns(
        source_folder,
        &["*.{numatb,numdlb,numshb,xmb}"],
    )
    .build()
    .unwrap()
    .into_iter()
    .filter_map(Result::ok)
    .par_bridge();

    let records: Vec<Box<dyn SqlInsert>> = paths_iter
        .map(|p| get_records(p.path(), source_folder))
        .flatten()
        .collect();

    println!(
        "Create {:?} records: {:?}",
        records.len(),
        parse_duration.elapsed()
    );

    // Perform a single transaction to improve performance.
    // This can only be done from a single thread.
    let database_duration = Instant::now();
    let mut transaction = connection.transaction()?;

    for record in &records {
        match record.insert(&mut transaction) {
            Ok(_) => (),
            Err(e) => {
                println!("Error inserting {:?}: {:?}", record, e);
                break;
            }
        }
    }

    transaction.commit()?;
    println!(
        "Write {} records to database: {:?}",
        records.len(),
        database_duration.elapsed()
    );

    Ok(())
}

fn create_indexes(connection: &mut Connection) -> Result<()> {
    let transaction = connection.transaction()?;

    // Create indexes to optimize only the more commonly specified parameters.
    transaction.execute_batch(
        "CREATE INDEX BlendState_MaterialId_Idx ON BlendState(MaterialId);
        CREATE INDEX CustomBooleanParam_MaterialId_Idx ON CustomBooleanParam(MaterialId);
        CREATE INDEX CustomFloatParam_MaterialId_Idx ON CustomFloatParam(MaterialId);
        CREATE INDEX CustomVectorParam_MaterialId_Idx ON CustomVectorParam(MaterialId);
        CREATE INDEX RasterizerState_MaterialId_Idx ON RasterizerState(MaterialId);
        CREATE INDEX Sampler_MaterialId_Idx ON Sampler(MaterialId);
        CREATE INDEX Texture_MaterialId_Idx ON Texture(MaterialId);
        CREATE INDEX BlendState_ParamId_Idx ON BlendState(ParamId);
        CREATE INDEX CustomBooleanParam_ParamId_Idx ON CustomBooleanParam(ParamId);
        CREATE INDEX CustomFloatParam_ParamId_Idx ON CustomFloatParam(ParamId);
        CREATE INDEX CustomVectorParam_ParamId_Idx ON CustomVectorParam(ParamId);
        CREATE INDEX RasterizerState_ParamId_Idx ON RasterizerState(ParamId);
        CREATE INDEX Sampler_ParamId_Idx ON Sampler(ParamId);
        CREATE INDEX Texture_ParamId_Idx ON Texture(ParamId);",
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

    // Reduce the amount of space used by the file on disk.
    connection.execute_batch("VACUUM;")?;

    Ok(())
}
