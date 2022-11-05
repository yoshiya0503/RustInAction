fn main() {
    let mut slice = ["this", "is", "test", "of", "loop"];
    let _a = for item in slice {
        println!("{}", item);
    };

    /*
    for item in &mut slice {
        item = "abc";
        println!("{}", item);
    }
    */

    let a = (0..10).zip(0..10);
    println!("{:?}", a);

    for (x, y) in (0..10).zip(0..10) {
        println!("<{} {} > {}", x, y, x + y)
    }

    let description = if is_even(124) {
        "even"
    } else {
        "odd"
    };
    println!("{}", description);
}

fn is_even(n: i32) -> bool {
    n % 2 == 0
}
