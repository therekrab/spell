use std::{
    cmp,
    fs::File,
    io::{BufRead, BufReader},
};

fn edit_distance(word1: &str, word2: &str, limit: usize) -> Option<usize> {
    None
}

pub fn spellcheck(word: &str, dictionary: &Vec<String>) -> String {
    let oword = word.to_string();
    let len = dictionary.len();

    let res = dictionary.binary_search(&oword);

    if res.is_ok() {
        return oword;
    }

    let i = res.unwrap_err();

    let mut l = i;
    let mut r = i;

    let mut closest_word = oword;
    let mut closest_distance = word.len();

    while l > 0 || r < len {
        if l > 0 {
            // decrement, then check l
            // this means that once l is 0, we will already have checked dict[0], so we can stop without worrying about overflow.
            l -= 1;
            let candidate = dictionary[l].to_owned();
            let dist_option = edit_distance(word, &candidate, closest_distance);
            if dist_option.is_none() {
                continue;
            }
            let dist = dist_option.unwrap();
            if dist == 1 {
                return candidate;
            }
            if dist < closest_distance {
                closest_distance = dist;
                closest_word = candidate;
            }
        }

        if r < len {
            // check r, then increment
            let candidate = dictionary[r].to_owned();
            let dist_option = edit_distance(word, &candidate, closest_distance);
            if dist_option.is_none() {
                continue;
            }
            let dist = dist_option.unwrap();
            if dist == 1 {
                return candidate;
            }
            if dist < closest_distance {
                closest_distance = dist;
                closest_word = candidate;
            }
            r += 1;
        }
    }

    closest_word
}

pub fn load_dictionary() -> Vec<String> {
    let mut dictionary: Vec<String> = Vec::new();

    let f: File = match File::open("words.txt") {
        Ok(file) => file,
        Err(e) => panic!("Failed to open dictionary: {e:?}"),
    };

    let mut reader: BufReader<File> = BufReader::new(f);
    loop {
        let mut line: String = String::new();
        let res = reader.read_line(&mut line);
        if res.unwrap_or(0) == 0 {
            // we are done for one  reason or another.
            break;
        }
        let l = line.trim().to_owned();
        dictionary.push(l);
    }
    dictionary
}
