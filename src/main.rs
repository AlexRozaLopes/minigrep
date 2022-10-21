use std;

fn main() {
    let args: Vec<String> = std::env::args().collect();
    let query = &args[1];
    let filename = &args[2];
    println!("Searching for {}", query);
    println!("In file {}", filename);

    let contents = std::fs::read_to_string(filename)
        .expect("should be able to read the file");
    print!("With text:\n{contents}");
}
