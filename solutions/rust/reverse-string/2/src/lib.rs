#[cfg(feature = "grapheme")]
use unicode_segmentation::UnicodeSegmentation;

pub fn reverse(input: &str) -> String {
  let graphemes = input.graphemes(true).collect::<Vec<&str>>();
  let reversed = graphemes.iter().rev();
  return reversed.map(|g| g.to_string())
         .collect::<String>();
}
