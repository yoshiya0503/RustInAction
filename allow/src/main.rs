#![allow(unused_variables)]

type File = String;

fn open(f: &mut File) -> bool { println!("call open"); true }

fn close(f: &mut File) -> bool { println!("call close"); true }

#[allow(dead_code)]
fn read(f: &mut File, save_to: &mut Vec<u8>) -> ! {
    unimplemented!()
}

fn main() {
    let mut f1 = File::from("t1.txt");
    open(&mut f1);
    // read(f1, vec![])
    close(&mut f1);
}
