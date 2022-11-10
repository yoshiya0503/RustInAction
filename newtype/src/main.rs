struct Hostname(String);

fn connect(hostname: Hostname) {
    println!("connected to {}", hostname.0);
}
fn main() {
    let ordinary_string = String::from("locahost");
    let host = Hostname(ordinary_string.clone());

    connect(ordinary_string);
    //connect(host);
}
