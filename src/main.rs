extern crate regex;

use std::io::{self, BufRead};
use std::collections::BTreeMap;

use regex::Regex;

fn main() {
  let is_a_word = Regex::new(r"[a-z']+").unwrap();
  let mut counts: BTreeMap<String, isize> = BTreeMap::new();

  let stdin = io::stdin();

  for line_result in stdin.lock().lines() {
    let line = line_result.unwrap().to_lowercase();
    let words: Vec<String> = is_a_word.find_iter(&line)
      .map(|word| word.as_str().to_string())
      .collect();

    for word in words {
      *counts.entry(word).or_insert(0) += 1;
    }
    print_counts(&counts);
  }
}

fn print_counts(counts: &BTreeMap<String, isize>) {
  for (word, count) in counts.iter() {
    println!("{}: {}", word, count);
  }
}
