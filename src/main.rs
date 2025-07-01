use std::time;

fn main() {
    let dictionary = spell::load_dictionary();
    println!("loaded dictionary, start typing:");

    loop {
        let mut sentence: String = String::new();
        std::io::stdin()
            .read_line(&mut sentence)
            .expect("failed to read from stdin");

        let start_time = time::Instant::now();

        for word in sentence.trim().split_whitespace() {
            let correction: String = spell::spellcheck(&word, &dictionary);
            print!("{correction} ");
        }
        println!(""); // final \n

        let end_time = time::Instant::now();

        let duration = end_time - start_time;
        println!("elapsed: {duration:#?}");
    }
}


