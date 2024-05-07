// related to issue https://github.com/media-io/yaserde/issues/11
use hifa_yaserde::*;

#[derive(YaDeserialize, Default, Debug, PartialEq)]
#[yaserde(root = "DOMSymbolItem")]
struct Level {
  #[yaserde(attribute)]
  last_modified: u64,
  #[yaserde(attribute, rename = "name")]
  named: String,
  timeline: Timeline,
}

#[derive(YaDeserialize, Default, Debug, PartialEq)]
struct Timeline {
  #[yaserde(rename = "DOMTimeline")]
  timeline: DOMTimeline,
}

#[derive(YaDeserialize, Default, Debug, PartialEq)]
struct DOMTimeline {
  #[yaserde(attribute, rename = "name")]
  named: String,
  #[yaserde(attribute)]
  current_frame: u64,
  #[yaserde(attribute)]
  guides: u64,
  layers: Layers,
}

#[derive(YaDeserialize, Default, Debug, PartialEq)]
struct Layers {
  #[yaserde(rename = "DOMLayer")]
  dom_layer: Vec<DOMLayer>,
}

#[derive(YaDeserialize, Default, Debug, PartialEq)]
struct DOMLayer {
  #[yaserde(attribute, rename = "name")]
  named: String,
  #[yaserde(attribute)]
  name2: String,
}

#[test]
fn parsing_ln_dom() {
  use hifa_yaserde::de::from_str;
  use std::fs;

  let filename = "tests/data/ln-dom.xml";

  let content = fs::read_to_string(filename).expect("something went wrong reading the file");

  let loaded: Level = from_str(&content).unwrap();
  println!("{:?}", loaded);

  let reference = Level {
    last_modified: 1414141442,
    named: "dagger".to_string(),
    timeline: Timeline {
      timeline: DOMTimeline {
        named: "dagger timeline name".to_string(),
        current_frame: 7,
        guides: 11,
        layers: Layers {
          dom_layer: vec![
            DOMLayer {
              named: "Layer 2".to_string(),
              name2: "Lalayer 2".to_string(),
            },
            DOMLayer {
              named: "Layer 1".to_string(),
              name2: "Lalayer 1".to_string(),
            },
          ],
        },
      },
    },
  };

  assert_eq!(loaded, reference);
}
