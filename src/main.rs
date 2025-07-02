use std::time;

fn main() {
    let dictionary = spell::load_dictionary();
    loop {
        let mut sentence: String = String::new();
        std::io::stdin()
            .read_line(&mut sentence)
            .expect("failed to read from stdin");

        let start_time = time::Instant::now();

        let iter = sentence.trim().split_whitespace();
        spell::check_iter(iter, &dictionary);

        let end_time = time::Instant::now();

        let duration = end_time - start_time;
        println!("elapsed: {duration:#?}");
    }
}
