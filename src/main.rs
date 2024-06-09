use std::{
    cmp::min_by,
    fs::File,
    io::{BufRead, BufReader},
    time,
};

fn main() {
    let mut sentence: String = String::new();
    std::io::stdin()
        .read_line(&mut sentence)
        .expect("failed to read from stdin");

    let start_time = time::Instant::now();

    let dictionary = load_dictionary();

    for word in sentence.trim().split_whitespace() {
        let correction: String = spellcheck(&word, &dictionary);
        print!("{correction} ");
    }
    println!("");

    let end_time = time::Instant::now();

    let duration = end_time - start_time;
    println!("elapsed: {duration:#?}");
}

fn load_dictionary() -> Vec<String> {
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

// this is an optimized agorithm. do not use for normal purposes, unless your limit is ridiculously large. if the dist(a, b) == limit, it will return none.
fn edit_distance(word1: &str, word2: &str, limit: usize) -> usize {
    if word1 == word2 {
        return 0;
    }
    let n: usize = word1.len();
    let m: usize = word2.len();
    if n * m == 0 {
        return n + m;
    }
    let mut grid: Vec<Vec<usize>> = Vec::new();
    for _ in 0..=n {
        let mut v: Vec<usize> = Vec::new();
        for _ in 0..=m {
            v.push(100);
        }
        grid.push(v);
    }
    let chars1: Vec<char> = word1.chars().collect();
    let chars2: Vec<char> = word2.chars().collect();
    for i in 0..=n {
        let mut smallest_seen = n + m; // we just need a big value, this shouldn't show up again.
        for j in 0..=m {
            // check for edge cases: j == 0 means that we are in the first column, so we will just return i
            // i == 0 means that we are in the first row, so we will just return j
            if i * j == 0 {
                grid[i][j] = i + j;
                smallest_seen = min_by(smallest_seen, i + j, |a: &usize, b: &usize| a.cmp(b));
                continue;
            }

            let deletion_score = grid[i - 1][j] + 1;
            let insertion_score = grid[i][j - 1] + 1;
            let mut swap_score = grid[i - 1][j - 1];
            if chars1[i - 1] != chars2[j - 1] {
                swap_score += 1;
            }

            let mut min_value = deletion_score;
            if insertion_score < min_value {
                min_value = insertion_score;
            }
            if swap_score < min_value {
                min_value = swap_score;
            }

            grid[i][j] = min_value;
            smallest_seen = min_by(min_value, smallest_seen, |a: &usize, b: &usize| a.cmp(b));
        }
        if smallest_seen >= limit {
            return limit;
        }
    }

    grid[n][m]
}

fn search(dictionary: &Vec<String>, word: &String) -> usize {
    let mut l: usize = 0;
    let mut r: usize = dictionary.len() - 1;

    while l <= r {
        let m: usize = (l + r) / 2;
        let dword = &dictionary[m];
        if dword == word {
            return m;
        }
        if dword < word {
            l = m + 1;
        }
        if dword > word {
            r = m - 1;
        }
    }

    l
}

fn spellcheck(word: &str, dictionary: &Vec<String>) -> String {
    let oword = word.to_owned();
    let len = dictionary.len();

    let i = search(dictionary, &oword);

    if dictionary[i] == word {
        return oword;
    }

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
            let dist = edit_distance(word, &candidate, closest_distance);
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
            let dist = edit_distance(word, &candidate, closest_distance);
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
