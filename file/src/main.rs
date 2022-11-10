use rand::prelude::*;

pub fn one_in(denomitator: u32) -> bool {
    thread_rng().gen_ratio(1, denomitator)
}

pub trait Read {
    fn read(self: &Self, save_to: &mut Vec<u8>) -> Result<usize, String>;
}

impl Read for File {
    fn read(self: &File, save_to: &mut Vec<u8>) -> Result<usize, String> {
        let mut tmp = self.data.clone();
        save_to.reserve(tmp.len());
        save_to.append(&mut tmp);
        Ok(tmp.len())
    }
}

#[derive(Debug)]
pub struct File {
    pub name: String,
    data: Vec<u8>,
}

impl File {
    pub fn new(name: &str) -> File {
        File {
            name: String::from(name),
            data: Vec::new(),
        }
    }

    pub fn new_with_data(name: &str, data: Vec<u8>) -> File {
        let mut f = File::new(name);
        f.data = data.clone();
        f
    }

}

pub fn open(f: File) -> Result<File, String> {
    if one_in(10_000) {
        let msg = String::from("permission denied");
        return Err(msg);
    }
    Ok(f)
}

pub fn close(f: File) -> Result<File, String> {
    if one_in(10_000) {
        let msg = String::from("permission denied");
        return Err(msg);
    }
    Ok(f)
}


fn main() {
    let d: Vec<u8> = vec![114, 117, 115, 116, 33];
    let mut f2 = File::new_with_data("file.txt", d);
    let mut buffer: Vec<u8> = vec![];

    f2 = open(f2).unwrap();
    let f2_length = &f2.read(&mut buffer).unwrap();
    f2 = close(f2).unwrap();

    let text = String::from_utf8_lossy(&buffer);
    println!("{:?}", f2);
    println!("{} is {} byrtes long", f2.name, f2_length);
    println!("{}", text)
}
