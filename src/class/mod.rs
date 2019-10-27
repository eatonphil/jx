fn consume_u8(b: &Vec<u8>, cursor: usize) -> (u8, usize) {
    (b[cursor], cursor + 1)
}

fn consume_u16(b: &Vec<u8>, cursor: usize) -> (u16, usize) {
    let (left, cursor) = consume_u8(b, cursor);
    let (right, cursor) = consume_u8(b, cursor);
    (((left as u16) << 8) | (right as u16), cursor)
}

fn consume_u32(b: &Vec<u8>, cursor: usize) -> (u32, usize) {
    let (left, cursor) = consume_u16(b, cursor);
    let (right, cursor) = consume_u16(b, cursor);
    (((left as u32) << 16) | (right as u32), cursor)
}

fn consume_u64(b: &Vec<u8>, cursor: usize) -> (u64, usize) {
    let (left, cursor) = consume_u32(b, cursor);
    let (right, cursor) = consume_u32(b, cursor);
    (((left as u64) << 32) | (right as u64), cursor)
}

#[derive(Debug)]
enum AccessFlag {
    Public = 0x0001,
    Final = 0x0010,
    Super = 0x0020,
    Interface = 0x0200,
    Abstract = 0x0400,
    Synthetic = 0x1000,
    Annotation = 0x2000,
    Enum = 0x4000,
}

#[derive(Debug)]
enum Const {
    Class {
        name_index: u16,
    },
    Fieldref {
        class_index: u16,
        nameandtype_index: u16,
    },
    Methodref {
        class_index: u16,
        nameandtype_index: u16,
    },
    InterfaceMethodref {
        class_index: u16,
        nameandtype_index: u16,
    },
    String {
        string_index: u16,
    },
    Integer(u32),
    Float(f32),
    Long(u64),
    Double(f64),
    NameAndType {
        name_index: u16,
        descriptor_index: u16,
    },
    Utf8(String),
    MethodHandle {
        reference_kind: u8,
        reference_index: u16,
    },
    MethodType {
        descriptor_index: u16,
    },
    InvokeDynamic {
        bootstrap_method_attr_index: u16,
        name_and_type_index: u16,
    },
}

impl Const {
    pub fn print(&self) {
        match self {
            Const::Class { .. } => (),
            Const::Fieldref { .. } => (),
            Const::Methodref { .. } => (),
            Const::InterfaceMethodref { .. } => (),
            Const::String { .. } => print!(""),
            Const::Integer(i) => print!("Integer: {}\n", i),
            Const::Float(f) => print!("Float: {}\n", f),
            Const::Long(l) => print!("Long: {}\n", l),
            Const::Double(d) => print!("Double: {}\n", d),
            Const::NameAndType { .. } => (),
            Const::Utf8(s) => print!("Utf8: {}\n", s),
            _ => panic!("Unknown tag: {:#?}", self),
        }
    }
}

#[derive(Debug)]
enum ConstType {
    Class = 7,
    Fieldref = 9,
    Methodref = 10,
    InterfaceMethodref = 11,
    String = 8,
    Integer = 3,
    Float = 4,
    Long = 5,
    Double = 6,
    NameAndType = 12,
    Utf8 = 1,
    MethodHandle = 15,
    MethodType = 16,
    InvokeDynamic = 18,
}

fn consume_class_constant(b: &Vec<u8>, cursor: usize) -> (Const, usize) {
    let (name_index, cursor) = consume_u16(b, cursor);
    let c = Const::Class { name_index };
    (c, cursor)
}

fn consume_fieldref_constant(b: &Vec<u8>, cursor: usize) -> (Const, usize) {
    let (class_index, cursor) = consume_u16(b, cursor);
    let (nameandtype_index, cursor) = consume_u16(b, cursor);
    let c = Const::Fieldref {
        class_index,
        nameandtype_index,
    };
    (c, cursor)
}

fn consume_methodref_constant(b: &Vec<u8>, cursor: usize) -> (Const, usize) {
    let (class_index, cursor) = consume_u16(b, cursor);
    let (nameandtype_index, cursor) = consume_u16(b, cursor);
    let c = Const::Methodref {
        class_index,
        nameandtype_index,
    };
    (c, cursor)
}

fn consume_interfacemethodref_constant(b: &Vec<u8>, cursor: usize) -> (Const, usize) {
    let (class_index, cursor) = consume_u16(b, cursor);
    let (nameandtype_index, cursor) = consume_u16(b, cursor);
    let c = Const::InterfaceMethodref {
        class_index,
        nameandtype_index,
    };
    (c, cursor)
}

fn consume_string_constant(b: &Vec<u8>, cursor: usize) -> (Const, usize) {
    let (string_index, cursor) = consume_u16(b, cursor);
    (Const::String { string_index }, cursor)
}

fn consume_integer_constant(b: &Vec<u8>, cursor: usize) -> (Const, usize) {
    let (bytes, cursor) = consume_u32(b, cursor);
    (Const::Integer(bytes), cursor)
}

fn consume_float_constant(b: &Vec<u8>, cursor: usize) -> (Const, usize) {
    let (bytes, cursor) = consume_u32(b, cursor);
    (Const::Integer(bytes), cursor)
}

fn consume_long_constant(b: &Vec<u8>, cursor: usize) -> (Const, usize) {
    let (bytes, cursor) = consume_u64(b, cursor);
    (Const::Long(bytes), cursor)
}

fn consume_double_constant(b: &Vec<u8>, cursor: usize) -> (Const, usize) {
    let (bytes, cursor) = consume_u64(b, cursor);
    (Const::Double(bytes as f64), cursor)
}

fn consume_nameandtype_constant(b: &Vec<u8>, cursor: usize) -> (Const, usize) {
    let (name_index, cursor) = consume_u16(b, cursor);
    let (descriptor_index, cursor) = consume_u16(b, cursor);
    (
        Const::NameAndType {
            name_index,
            descriptor_index,
        },
        cursor,
    )
}

fn consume_utf8_constant(b: &Vec<u8>, cursor: usize) -> (Const, usize) {
    let (length, cursor) = consume_u16(b, cursor);
    let offset = cursor + (length as usize);
    let c = Const::Utf8(std::str::from_utf8(&b[cursor..offset]).unwrap().to_string());
    (c, offset)
}

impl ConstType {
    fn consume_constant(&self, b: &Vec<u8>, cursor: usize) -> (Const, usize) {
        match self {
            ConstType::Class => consume_class_constant(b, cursor),
            ConstType::Fieldref => consume_fieldref_constant(b, cursor),
            ConstType::Methodref => consume_methodref_constant(b, cursor),
            ConstType::InterfaceMethodref => consume_interfacemethodref_constant(b, cursor),
            ConstType::String => consume_string_constant(b, cursor),
            ConstType::Integer => consume_integer_constant(b, cursor),
            ConstType::Float => consume_float_constant(b, cursor),
            ConstType::Long => consume_long_constant(b, cursor),
            ConstType::Double => consume_double_constant(b, cursor),
            ConstType::NameAndType => consume_nameandtype_constant(b, cursor),
            ConstType::Utf8 => consume_utf8_constant(b, cursor),
            _ => panic!("Unknown tag: {:#?}", self),
        }
    }
}

#[derive(Debug)]
struct Interface {
    index: u16,
}

#[derive(Debug)]
struct Field {
    access_flags: u16,
    name_index: u16,
    descriptor_index: u16,
    attributes: Vec<Attribute>,
}

impl Field {
    fn print(&self) {}
}

#[derive(Debug)]
struct Method {
    access_flags: u16,
    name_index: u16,
    descriptor_index: u16,
    attributes: Vec<Attribute>,
}

impl Method {
    fn print(&self) {}
}

#[derive(Debug)]
struct Attribute {
    name_index: u16,
    info: Vec<u8>,
}

const MAGIC: u32 = 0xCAFEBABE;

fn consume_constant_pool(b: &Vec<u8>, n: usize, cursor: usize) -> (Vec<Const>, usize) {
    let mut v: Vec<Const> = Vec::new();
    let mut c = cursor;
    while v.len() < n - 1 {
        let tag: ConstType = unsafe { std::mem::transmute(b[c]) };
        let res = tag.consume_constant(b, c + 1);
        v.push(res.0);
        c = res.1;
    }
    (v, c)
}

fn consume_interfaces(b: &Vec<u8>, n: usize, cursor: usize) -> (Vec<Interface>, usize) {
    let mut v: Vec<Interface> = Vec::new();
    while v.len() < n {
        let (index, _) = consume_u16(b, cursor);
        v.push(Interface { index });
    }
    return (v, cursor + n * 2);
}

fn consume_fields(b: &Vec<u8>, n: usize, cursor: usize) -> (Vec<Field>, usize) {
    let mut v: Vec<Field> = Vec::new();
    let mut c = cursor;
    while v.len() < n {
        let (access_flags, cursor) = consume_u16(b, c);
        let (name_index, cursor) = consume_u16(b, cursor);
        let (descriptor_index, cursor) = consume_u16(b, cursor);
        let (attributes_count, cursor) = consume_u16(b, cursor);
        let (attributes, cursor) = consume_attributes(b, attributes_count as usize, cursor);

        v.push(Field {
            access_flags,
            name_index,
            descriptor_index,
            attributes,
        });

        c = cursor;
    }
    return (v, c);
}

fn consume_methods(b: &Vec<u8>, n: usize, cursor: usize) -> (Vec<Method>, usize) {
    let mut v: Vec<Method> = Vec::new();
    let mut c = cursor;
    while v.len() < n {
        let (access_flags, cursor) = consume_u16(b, c);
        let (name_index, cursor) = consume_u16(b, cursor);
        let (descriptor_index, cursor) = consume_u16(b, cursor);
        let (attributes_count, cursor) = consume_u16(b, cursor);
        let (attributes, cursor) = consume_attributes(b, attributes_count as usize, cursor);

        v.push(Method {
            access_flags,
            name_index,
            descriptor_index,
            attributes,
        });

        c = cursor;
    }
    return (v, c);
}

fn consume_attributes(b: &Vec<u8>, n: usize, cursor: usize) -> (Vec<Attribute>, usize) {
    let mut v: Vec<Attribute> = Vec::new();

    let mut c = cursor;
    while v.len() < n {
        let (name_index, cursor) = consume_u16(b, c);
        let (length, cursor) = consume_u32(b, cursor);
        v.push(Attribute {
            name_index: name_index,
            info: b[cursor..cursor + (length as usize)]
                .iter()
                .cloned()
                .collect(),
        });
        c = cursor + (length as usize);
    }

    return (v, c);
}

#[derive(Debug)]
pub struct Class {
    minor_version: u16,
    major_version: u16,
    constant_pool: Vec<Const>,
    access_flags: u16,
    this_class: u16,
    super_class: u16,
    interfaces: Vec<Interface>,
    fields: Vec<Field>,
    methods: Vec<Method>,
    attributes: Vec<Attribute>,
}

impl Class {
    pub fn print(&self) {
        print!("Major: {}\n", self.major_version);
        print!("Minor: {}\n", self.minor_version);
        print!("Access Flags: {}\n", self.access_flags);
        self.constant_pool[self.this_class as usize].print();
        self.constant_pool[self.super_class as usize].print();

        for f in self.fields.iter() {
            f.print();
        }

        for m in self.methods.iter() {
            m.print();
        }
    }
}

pub fn parse(data: &Vec<u8>) -> Class {
    let (magic, cursor) = consume_u32(data, 0);
    if magic != MAGIC {
        panic!("Expected Java class file, got {:#?}", magic);
    }

    let (minor_version, cursor) = consume_u16(data, cursor);
    let (major_version, cursor) = consume_u16(data, cursor);
    let (constant_pool_count, cursor) = consume_u16(data, cursor);
    let (constants, cursor) = consume_constant_pool(data, constant_pool_count as usize, cursor);
    let (access_flags, cursor) = consume_u16(data, cursor);
    let (this_class, cursor) = consume_u16(data, cursor);
    let (super_class, cursor) = consume_u16(data, cursor);
    let (interfaces_count, cursor) = consume_u16(data, cursor);
    let (interfaces, cursor) = consume_interfaces(data, interfaces_count as usize, cursor);
    let (fields_count, cursor) = consume_u16(data, cursor);
    let (fields, cursor) = consume_fields(data, fields_count as usize, cursor);
    let (methods_count, cursor) = consume_u16(data, cursor);
    let (methods, cursor) = consume_methods(data, methods_count as usize, cursor);
    let (attributes_count, cursor) = consume_u16(data, cursor);
    let (attributes, _) = consume_attributes(data, attributes_count as usize, cursor);

    Class {
        minor_version: minor_version,
        major_version: major_version,
        constant_pool: constants,
        access_flags: access_flags,
        this_class: this_class,
        super_class: super_class,
        interfaces: interfaces,
        fields: fields,
        methods: methods,
        attributes: attributes,
    }
}
