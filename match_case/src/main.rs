fn main() {
    let needle = 54;
    let haystack = [1, 1, 2, 5, 14, 54, 132];
    for item in &haystack {
        let result = match item {
            14|5 => "hit",
            &needle => "out",
            _ => "no_hit",
        };
        if result == "hit" {
            println!("{:?}", item);
        };

        if *item == needle {
            println!("{:?}", item);
        };

    };
    let c = add_with_lifetime(&10, &20);
    println!("c = {}", c);
    println!("{}", add(5, 1));
    println!("{}", add(5.2, 1.9));
}

fn add_with_lifetime<'a, 'b>(i: &'a i32, j: &'b i32) -> i32 {
    return *i + *j
}

fn add<T: std::ops::Add<Output = T>>(i: T, j: T) -> T {
    return i + j;
}
