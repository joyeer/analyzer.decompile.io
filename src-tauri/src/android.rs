// Android DEX file structures
use crate::android_analyzer::apk_analyzer::ApkAnalyzer;

#[derive(Debug, Clone)]
pub struct AndroidProjectData {
    pub apk_path: String,
    pub dex_files: Vec<DexFile>,
    pub manifest: Option<AndroidManifest>,
    pub resource_table: Option<ResourceTable>,
}

impl AndroidProjectData {
    pub fn new() -> Self {
        Self {
            apk_path: String::new(),
            dex_files: Vec::new(),
            manifest: None,
            resource_table: None,
        }
    }
    
    pub fn with_apk_path(apk_path: String) -> Self {
        Self {
            apk_path,
            dex_files: Vec::new(),
            manifest: None,
            resource_table: None,
        }
    }
}

#[derive(Debug, Clone)]
pub struct DexFile {
    pub magic: [u8; 8],
    pub checksum: u32,
    pub signature: [u8; 20],
    pub file_size: u32,
    pub header_size: u32,
    pub endian_tag: u32,
    pub link_size: u32,
    pub link_offset: u32,
    pub map_offset: u32,
    pub string_ids_size: u32,
    pub string_ids_offset: u32,
    pub type_ids_size: u32,
    pub type_ids_offset: u32,
    pub proto_ids_size: u32,
    pub proto_ids_offset: u32,
    pub field_ids_size: u32,
    pub field_ids_offset: u32,
    pub method_ids_size: u32,
    pub method_ids_offset: u32,
    pub class_defs_size: u32,
    pub class_defs_offset: u32,
    pub data_size: u32,
    pub data_offset: u32,
    
    // Parsed data
    pub strings: Vec<String>,
    pub types: Vec<TypeDescriptor>,
    pub protos: Vec<ProtoDescriptor>,
    pub fields: Vec<FieldDescriptor>,
    pub methods: Vec<MethodDescriptor>,
    pub classes: Vec<ClassDef>,
}

#[derive(Debug, Clone)]
pub struct AndroidManifest {
    pub package_name: String,
    pub version_code: u32,
    pub version_name: String,
    pub activities: Vec<Activity>,
    pub services: Vec<Service>,
    pub receivers: Vec<Receiver>,
    pub providers: Vec<Provider>,
    pub permissions: Vec<Permission>,
    pub uses_permissions: Vec<String>,
}

#[derive(Debug, Clone)]
pub struct ResourceTable {
    pub packages: Vec<ResourcePackage>,
}

#[derive(Debug, Clone)]
pub struct ResourcePackage {
    pub id: u32,
    pub name: String,
    pub types: Vec<ResourceType>,
}

#[derive(Debug, Clone)]
pub struct ResourceType {
    pub id: u32,
    pub name: String,
    pub entries: Vec<ResourceEntry>,
}

#[derive(Debug, Clone)]
pub struct ResourceEntry {
    pub id: u32,
    pub name: String,
    pub value: ResourceValue,
}

#[derive(Debug, Clone)]
pub enum ResourceValue {
    Null,
    Reference(u32),
    Attribute(u32),
    String(u32),
    Float(f32),
    Dimension(u32),
    Fraction(u32),
    Integer(i32),
    Boolean(bool),
    Color(u32),
    ColorStateList(u32),
    Complex {
        parent: u32,
        values: std::collections::HashMap<u32, ResourceValue>,
    },
    Unknown(u32),
}

// DEX structures
#[derive(Debug, Clone)]
pub struct TypeDescriptor {
    pub descriptor: String,
}

impl TypeDescriptor {
    pub fn from_descriptor(descriptor: &str) -> Result<Self, String> {
        Ok(Self {
            descriptor: descriptor.to_string(),
        })
    }
    
    pub fn package(&self) -> Option<String> {
        if self.descriptor.starts_with('L') && self.descriptor.ends_with(';') {
            let class_name = &self.descriptor[1..self.descriptor.len()-1];
            if let Some(last_slash) = class_name.rfind('/') {
                Some(class_name[..last_slash].replace('/', "."))
            } else {
                None
            }
        } else {
            None
        }
    }
}

#[derive(Debug, Clone)]
pub struct ProtoDescriptor {
    pub shorty: String,
    pub return_type: TypeDescriptor,
    pub parameters: Vec<TypeDescriptor>,
}

#[derive(Debug, Clone)]
pub struct FieldDescriptor {
    pub class_type: TypeDescriptor,
    pub field_type: TypeDescriptor,
    pub name: String,
}

#[derive(Debug, Clone)]
pub struct MethodDescriptor {
    pub class_type: TypeDescriptor,
    pub proto: ProtoDescriptor,
    pub name: String,
}

#[derive(Debug, Clone)]
pub struct ClassDef {
    pub class_type: TypeDescriptor,
    pub access_flags: u32,
    pub super_type: Option<TypeDescriptor>,
    pub interfaces: Vec<TypeDescriptor>,
    pub source_file: Option<String>,
    pub annotations: Vec<Annotation>,
    pub static_fields: Vec<Field>,
    pub instance_fields: Vec<Field>,
    pub direct_methods: Vec<Method>,
    pub virtual_methods: Vec<Method>,
}

#[derive(Debug, Clone)]
pub struct Field {
    pub access_flags: u32,
    pub name: String,
    pub field_type: TypeDescriptor,
    pub value: Option<EncodedValue>,
}

#[derive(Debug, Clone)]
pub struct Method {
    pub access_flags: u32,
    pub name: String,
    pub proto: ProtoDescriptor,
    pub code: Option<CodeItem>,
}

#[derive(Debug, Clone)]
pub struct Annotation {
    pub annotation_type: TypeDescriptor,
    pub elements: Vec<AnnotationElement>,
}

#[derive(Debug, Clone)]
pub struct AnnotationElement {
    pub name: String,
    pub value: EncodedValue,
}

#[derive(Debug, Clone)]
pub enum EncodedValue {
    Byte(i8),
    Short(i16),
    Char(u16),
    Int(i32),
    Long(i64),
    Float(f32),
    Double(f64),
    String(String),
    Type(TypeDescriptor),
    Field(FieldDescriptor),
    Method(MethodDescriptor),
    Enum(FieldDescriptor),
    Array(Vec<EncodedValue>),
    Annotation(Annotation),
    Null,
    Boolean(bool),
}

#[derive(Debug, Clone)]
pub struct ClassData {
    pub static_fields_size: u32,
    pub instance_fields_size: u32,
    pub direct_methods_size: u32,
    pub virtual_methods_size: u32,
}

#[derive(Debug, Clone)]
pub struct EncodedField {
    pub field_idx_diff: u32,
    pub access_flags: u32,
    pub field_idx: u32, // computed
}

#[derive(Debug, Clone)]
pub struct EncodedMethod {
    pub method_idx_diff: u32,
    pub access_flags: u32,
    pub code_offset: u32,
    pub method_idx: u32, // computed
    pub code: Option<CodeItem>,
}

#[derive(Debug, Clone)]
pub struct CodeItem {
    pub registers_size: u16,
    pub ins_size: u16,
    pub outs_size: u16,
    pub tries_size: u16,
    pub debug_info_offset: u32,
    pub insns_size: u32,
    pub insns: Vec<u16>,
    pub tries: Vec<TryItem>,
    pub handlers: Vec<EncodedCatchHandler>,
}

#[derive(Debug, Clone)]
pub struct TryItem {
    pub start_addr: u32,
    pub insn_count: u16,
    pub handler_offset: u16,
}

#[derive(Debug, Clone)]
pub struct EncodedCatchHandler {
    pub size: i32,
    pub handlers: Vec<EncodedTypeAddrPair>,
    pub catch_all_addr: Option<u32>,
}

#[derive(Debug, Clone)]
pub struct EncodedTypeAddrPair {
    pub type_idx: u32,
    pub addr: u32,
}

// Android Manifest structures
#[derive(Debug, Clone)]
pub struct Activity {
    pub name: String,
    pub label: Option<String>,
    pub exported: bool,
    pub intent_filters: Vec<IntentFilter>,
}

#[derive(Debug, Clone)]
pub struct Service {
    pub name: String,
    pub exported: bool,
    pub intent_filters: Vec<IntentFilter>,
}

#[derive(Debug, Clone)]
pub struct Receiver {
    pub name: String,
    pub exported: bool,
    pub intent_filters: Vec<IntentFilter>,
}

#[derive(Debug, Clone)]
pub struct Permission {
    pub name: String,
    pub protection_level: String,
}

#[derive(Debug, Clone)]
pub struct Provider {
    pub name: String,
    pub authorities: Vec<String>,
    pub exported: bool,
    pub grant_uri_permissions: bool,
}

#[derive(Debug, Clone)]
pub struct IntentFilter {
    pub actions: Vec<String>,
    pub categories: Vec<String>,
    pub data: Vec<IntentData>,
}

#[derive(Debug, Clone)]
pub struct IntentData {
    pub scheme: Option<String>,
    pub host: Option<String>,
    pub port: Option<String>,
    pub path: Option<String>,
    pub path_pattern: Option<String>,
    pub path_prefix: Option<String>,
    pub mime_type: Option<String>,
}

// Access flags for classes, methods, fields
#[derive(Debug, Clone)]
pub struct AccessFlags {
    pub flags: u32,
}

impl AccessFlags {
    pub const PUBLIC: u32 = 0x0001;
    pub const PRIVATE: u32 = 0x0002;
    pub const PROTECTED: u32 = 0x0004;
    pub const STATIC: u32 = 0x0008;
    pub const FINAL: u32 = 0x0010;
    pub const SYNCHRONIZED: u32 = 0x0020;
    pub const VOLATILE: u32 = 0x0040;
    pub const BRIDGE: u32 = 0x0040;
    pub const TRANSIENT: u32 = 0x0080;
    pub const VARARGS: u32 = 0x0080;
    pub const NATIVE: u32 = 0x0100;
    pub const INTERFACE: u32 = 0x0200;
    pub const ABSTRACT: u32 = 0x0400;
    pub const STRICT: u32 = 0x0800;
    pub const SYNTHETIC: u32 = 0x1000;
    pub const ANNOTATION: u32 = 0x2000;
    pub const ENUM: u32 = 0x4000;
    pub const CONSTRUCTOR: u32 = 0x10000;
    pub const DECLARED_SYNCHRONIZED: u32 = 0x20000;
    
    pub fn new(flags: u32) -> Self {
        Self { flags }
    }
    
    pub fn is_public(&self) -> bool {
        (self.flags & Self::PUBLIC) != 0
    }
    
    pub fn is_private(&self) -> bool {
        (self.flags & Self::PRIVATE) != 0
    }
    
    pub fn is_protected(&self) -> bool {
        (self.flags & Self::PROTECTED) != 0
    }
    
    pub fn is_static(&self) -> bool {
        (self.flags & Self::STATIC) != 0
    }
    
    pub fn is_final(&self) -> bool {
        (self.flags & Self::FINAL) != 0
    }
    
    pub fn is_abstract(&self) -> bool {
        (self.flags & Self::ABSTRACT) != 0
    }
    
    pub fn is_interface(&self) -> bool {
        (self.flags & Self::INTERFACE) != 0
    }
    
    pub fn to_string(&self) -> String {
        let mut result = Vec::new();
        
        if self.is_public() { result.push("public"); }
        if self.is_private() { result.push("private"); }
        if self.is_protected() { result.push("protected"); }
        if self.is_static() { result.push("static"); }
        if self.is_final() { result.push("final"); }
        if self.is_abstract() { result.push("abstract"); }
        if self.is_interface() { result.push("interface"); }
        
        result.join(" ")
    }
}

/// Analyze an APK file
#[tauri::command]
pub fn android_analyze_apk(apk_path: String) -> Result<String, String> {
    let mut analyzer = ApkAnalyzer::new(apk_path);
    match analyzer.analyze() {
        Ok(data) => Ok(format!("APK analyzed successfully! Found {} DEX files, manifest: {}, resource table: {}", 
                              data.dex_files.len(),
                              data.manifest.is_some(),
                              data.resource_table.is_some())),
        Err(e) => Err(e.to_string())
    }
}