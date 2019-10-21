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

#[derive(Debug)]
struct Interface {}

#[derive(Debug)]
struct Field {}

#[derive(Debug)]
struct Method {}

#[derive(Debug)]
struct Attribute {}

const MAGIC: u16 = 0xCAFEBABE;

#[derive(Debug)]
struct Class {
    minor_version: u16,
    major_version: u16,
    constant_pool_count: u16,
    constant_pool: Vec<Const>,
    access_flags: u16,
    this_class: u16,
    super_class: u16,
    interfaces_count: u16,
    interfaces: Vec<Interface>,
    fields_count: u16,
    fields: Vec<Field>,
    methods_count: u16,
    methods: Vec<Method>,
    attributes_count: u16,
    attributes: Vec<Attribute>,
}

fn consume_u8(b: Vec<u8>) -> u8 {
    b.remove(0).expect("Failed reading 1st byte");
}

fn consume_u16(b: Vec<u8>) -> u16 {
    let b0 = b.remove(0).expect("Failed reading 1st byte");
    let b1 = b.remove(1).expect("Failed reading 2nd byte");
    (b0 << 8) & b1;
}

//fn consume_class_constant(b: Vec<u8>) -> Const {}

fn consume_constant_pool(b: Vec<u8>) -> Vec<Const> {
    let tag: ConstType = unsafe { std::mem::transmute(consume_u8(b)) };
    let v: Vec<Const> = Vec::new();
    panic!("Unknown tag: {:#?}", tag);
    // loop {
    //     v.push(match tag {
    //         ConstType::Class => consume_class_constant(b),
    //         _ => panic!("Unknown tag: {}", tag),
    //     })
    // }
    // v
}

fn main() {
    let args: Vec<String> = std::env::args().collect();
    let class_file = args.first().expect("Must specify class file");
    let data = std::fs::read(class_file).expect("Unable to read file");

    if consume_u16(data) != 0xCAFEBABE {
        panic!("Expected Java class file");
    }

    let minor_version = consume_u16(data);
    let major_version = consume_u16(data);
    let constant_pool_count = consume_u16(data);
    let consts = consume_constant_pool(data);
}
