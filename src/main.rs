use common_macros::hash_map;

fn main() {
  let classes = hash_map! {
    'C' => "(p|b|t|d|k|g|m|n|f|v|s|z|c|w|j|l)",
    'V' => "(i|u|e|o|a)",
    'S' => "(s|c)",
  };

  let patterns = [
    (false, "VSC", "Invalid syllable structure"),
    (false, "C{3}", "3 or more consonants sequentially"),
    (true, "^ S? ( C l? V n? )+ $", "General invalid structure"),
  ];

  let tests = [
    ("tanta", true),
    ("panta", true),
    ("panka", true),
    ("pania", false),
    ("spato", true),
    ("spato", true),
    ("splatlo", false),
    ("splanto", false),
    ("splasto", false),
    ("splantlo", false),
  ];

  phonotactics::run_tests(&tests, &patterns, &classes);
}
