use std::env;

fn main() {
    let args: Vec<String> = env::args().collect();
    let language = if args.len() > 1 {
        &args[1]
    } else {
        "Rust"
    };
    println!("{language} 是全宇宙最好的语言");
}
