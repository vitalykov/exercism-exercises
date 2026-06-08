use grapheme::Graphemes;

pub fn reverse(input: &str) -> String {
  let graphemes = Graphemes::from_usvs(input);
  let reversed = graphemes.iter().rev();
  return reversed.map(|g| g.to_string())
         .collect::<String>();
}
