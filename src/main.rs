mod class;

fn main() {
    let args: Vec<String> = std::env::args().collect();
    let class_file = args.get(1).expect("Must specify class file");
    let data = &std::fs::read(class_file).expect("Unable to read file");

    let c = class::parse(data);
    c.print();
}
