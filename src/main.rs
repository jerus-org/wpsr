use slb::Words;

const SOURCE_DIR: &str = "words";
const SOURCE_FILE: &str = "mit_words.txt";

fn main() {
    println!("Hello, world!");

    env_logger::init();

    let src = format!("{SOURCE_DIR}/{SOURCE_FILE}");

    let _words = Words::new(&src);

}
