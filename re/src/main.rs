use regex::Regex;

fn main() {
    let re = Regex::new("picture").unwrap();

    let quote = "\
        This is a Message.
        Every face, every shop, bedroom window, public-house, and
        dark square is picture feverishly";
    for (_, line) in quote.lines().enumerate() {
        let contains_substring = re.find(line);
        match contains_substring {
            Some(_) => println!("{}", line),
            None => (),
        }
    };
}
