use ahash::{HashMap, HashMapExt, HashSet};

use crate::{document::Node, TagType};

struct Automaton {
    tree_labels: usize,
}

type States = HashSet<State>;

#[derive(Debug, Eq, PartialEq, Hash)]
struct State(usize);

type Mapping = HashMap<State, HashSet<Node>>;

// fn top_down_run(automaton: Automaton, node: Option<Node>, states: States) -> Mapping {
//     if let Some(node) = node {
//     } else {
//         Mapping::new()
//     }
// }

struct TagLookup {}

impl TagLookup {
    fn new() -> Self {
        Self {}
    }
}

enum Guard {
    Includes(HashSet<TagType>),
    Excludes(HashSet<TagType>),
}

#[cfg(test)]
mod tests {

    use super::*;

    #[test]
    fn test_tag_lookup() {
        let mut lookup = TagLookup::new();
        let guard = Guard::includes([TagType::Element {
            namespace: "".to_string(),
            local_name: "foo".to_string(),
        }]);
        lookup.add(guard, "value");
        assert_eq!(
            lookup.get(TagType::Element {
                namespace: "".to_string(),
                local_name: "foo".to_string(),
            }),
            Some(&"value")
        );
        assert_eq!(
            lookup.get(TagType::Element {
                namespace: "".to_string(),
                local_name: "bar".to_string(),
            }),
            None
        );
    }
}
