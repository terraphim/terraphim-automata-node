#![deny(clippy::all)]

use napi_derive::napi;

use aho_corasick::{AhoCorasick, MatchKind};

#[napi]
pub fn find_matched(patterns: Vec<String>, haystack: String) -> Vec<String> {
  let mut matches = Vec::new();
  let ac = AhoCorasick::builder()
    .match_kind(MatchKind::LeftmostLongest)
    .build(patterns.clone())
    .unwrap();
  let text = &haystack.as_str();
  for mat in ac.find_iter(text) {
    let term = patterns[mat.pattern()].to_string();
    matches.push(term);
  }
  matches
}

#[napi]
pub fn replace_all_stream(patterns: Vec<String>, replace_with: Vec<String>, rdr: String) -> String {
  let mut wtr = vec![];
  let replace_with = &replace_with;
  let ac = AhoCorasick::new(patterns).unwrap();
  ac.try_stream_replace_all(rdr.as_bytes(), &mut wtr, replace_with)
    .unwrap();
  String::from_utf8(wtr)
    .map_err(|non_utf8| String::from_utf8_lossy(non_utf8.as_bytes()).into_owned())
    .unwrap()
}
