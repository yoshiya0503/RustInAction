#[derive(Debug)]
struct File {
    name: String,
    data: Vec<u8>,
}

fn main() {
    let f1 = File{
        name: "f1.txt".to_string(),
        data: Vec::new(),
    };

    let f1_name = &f1.name;
    let f1_data = &f1.data.len();

    println!("{:?}", f1);
    println!("{}, {}", f1_name, f1_data);
}
