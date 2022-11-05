fn greet_world() {
    println!("Hello, world");
    let southern_germany = "xxxx";
    let japanese = "こんにちは";
    let list = [southern_germany, japanese];
    for region in list.iter() {
        println!("{}", &region);
    }
}

fn main() {
    println!("Hello, world!");
    greet_world()
}
