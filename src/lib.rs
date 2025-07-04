use std::{
    env,
    fs::File,
    io::{BufRead, BufReader},
};

const MAX_DIFFERENCE: usize = 4;

pub enum Correction<'a> {
    Correct,
    Recommendation(&'a str),
    Unknown,
}

fn edit_distance(word1: &str, word2: &str, limit: Option<usize>) -> Option<usize> {
    if word1 == word2 {
        return Some(0);
    }

    let (shorter, longer);

    let (n, m);

    let word1len = word1.len();
    let word2len = word2.len();

    if word1len < word2len {
        (shorter, longer) = (word1, word2);
        (n, m) = (word1len, word2len);
    } else {
        (shorter, longer) = (word2, word1);
        (n, m) = (word2len, word1len);
    }

    // First limit test - if the difference in letters alone is greater than the limit, then we can
    // end now.
    if limit.is_some_and(|l| m - n > l) {
        return None;
    }

    // Store the characters in each word preemptively
    let chars1: Vec<char> = shorter.chars().collect();
    let chars2: Vec<char> = longer.chars().collect();

    /*
     * Possible optimization: instead of nested arrays, use math and one single big array
     * Possible optimization: don't instantiate with zeroes except for the first row.
     */
    let mut grid: Vec<Vec<usize>> = Vec::with_capacity(m + 1);
    // Push first row
    grid.push((0..=n).collect());
    for i in 1..=m {
        let mut row: Vec<usize> = Vec::with_capacity(n + 1);
        let mut min_value = i;
        // The calculation for each first item is a little different
        row.push(grid[i - 1][0] + 1);
        for j in 1..=n {
            let mut best = row[j - 1].min(grid[i - 1][j]) + 1;
            let swap = grid[i - 1][j - 1];
            if swap < best {
                // We have to check cost now
                let cost = if chars1[j - 1] == chars2[i - 1] { 0 } else { 1 };
                best = swap + cost;
            }
            // This is the best of the two options
            row.push(best);
            min_value = min_value.min(best);
        }
        if limit.is_some_and(|l| min_value > l) {
            return None;
        }
        grid.push(row);
    }
    Some(grid[m][n])
}

pub fn spellcheck<'a>(word: &str, dictionary: &'a [String]) -> Correction<'a> {
    let location = dictionary.binary_search(&word.to_string());
    // If the word is in the dictionary, just return it.
    if location.is_ok() {
        return Correction::Correct; // no change
    }
    let search_start = location.unwrap_err(); // Where we *could* go
    let mut l = search_start;
    let mut r = search_start + 1;
    let mut min_value = Some(MAX_DIFFERENCE);
    let mut index = search_start;
    let mut running = true;
    let mut l_ok;
    let mut r_ok;
    let mut l_word;
    let mut r_word;
    let mut found = false;

    while running {
        r_ok = r < dictionary.len();
        l_ok = l > 0;
        running = r_ok || l_ok;

        if r_ok {
            r_word = &dictionary[r];
            if let Some(dist) = edit_distance(word, r_word, min_value) {
                if min_value.is_none() || dist < min_value.unwrap() {
                    min_value = Some(dist);
                    index = r;
                    found = true;
                }
            }
            r += 1;
        }

        if l_ok {
            l_word = &dictionary[l];
            if let Some(dist) = edit_distance(word, l_word, min_value) {
                if min_value.is_none() || dist < min_value.unwrap() {
                    min_value = Some(dist);
                    index = l;
                    found = true;
                }
            }
            l -= 1;
        }

        if let Some(1) = min_value {
            return Correction::Recommendation(&dictionary[index]);
        }
    }
    if found {
        Correction::Recommendation(&dictionary[index])
    } else {
        Correction::Unknown
    }
}

pub fn load_dictionary() -> Vec<String> {
    let mut dictionary: Vec<String> = Vec::new();

    let f: File =
        match File::open(env::var("DICTIONARY").unwrap_or(String::from("/usr/share/dict/words"))) {
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
        let l = line.trim().to_lowercase();
        if l.is_ascii() {
            dictionary.push(l);
        }
    }
    dictionary.sort_unstable();
    dictionary
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn no_change() {
        let distance = edit_distance("movie", "movie", None);
        assert_eq!(distance, Some(0), "the same word is itself");
    }

    #[test]
    fn one_change() {
        let distance = edit_distance("dog", "dogs", None);
        assert_eq!(distance, Some(1), "`dog` and `dogs` are one letter away");
    }

    #[test]
    fn two_changes() {
        let distance = edit_distance("movie", "love", None);
        assert_eq!(distance, Some(2), "`movie` and `love` are two changes away");
    }
}
