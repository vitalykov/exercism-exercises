use std::collections::{HashSet, HashMap};

fn eq_ignore_case(a: &str, b: &str) -> bool {
    let mut a_it = a.chars().flat_map(|c| c.to_lowercase());
    let mut b_it = b.chars().flat_map(|c| c.to_lowercase());
    let zip_it = a_it.by_ref().zip(b_it.by_ref());
    for (ch_a, ch_b) in zip_it {
        if ch_a != ch_b {
            return false;
        }
    }
    if a_it.next().is_some() || b_it.next().is_some() {
        return false;
    }
    true
}

pub fn anagrams_for<'a>(word: &str, possible_anagrams: &'a [&str]) -> HashSet<&'a str> {
    let mut letters = HashMap::<char, i32>::new();
    for ch in word.chars().flat_map(|c| c.to_lowercase()) {
        letters.entry(ch)
            .and_modify(|freq| *freq += 1)
            .or_insert(1);
    }
    let mut anagrams = HashSet::<&str>::new();
    'outer: for w in possible_anagrams {
        if eq_ignore_case(w, word) {
            continue;
        }
        let mut letters_copy = letters.clone();
        for ch in w.chars().flat_map(|c| c.to_lowercase()) {
            if let Some(freq) = letters_copy.get_mut(&ch) {
                *freq -= 1;
                if *freq == 0 {
                    letters_copy.remove(&ch);
                }
            } else {
                continue 'outer;
            }
        }
        if letters_copy.is_empty() {
            anagrams.insert(w);
        }
    }
    anagrams
}
