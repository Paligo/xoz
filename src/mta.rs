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

struct TagLookup<T> {}

impl<T: Clone> TagLookup<T> {
    fn new() -> Self {
        Self {}
    }

    fn add(&mut self, guard: Guard, payload: T) {
        todo!()
    }

    fn matching(&self, tag: TagType) -> Vec<T> {
        todo!()
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
        let guard = Guard::Includes(
            [TagType::Element {
                namespace: "".to_string(),
                local_name: "foo".to_string(),
            }]
            .into_iter()
            .collect(),
        );
        lookup.add(guard, "value");
        assert_eq!(
            lookup.matching(TagType::Element {
                namespace: "".to_string(),
                local_name: "foo".to_string(),
            }),
            vec![&"value"]
        );
        assert_eq!(
            lookup.matching(TagType::Element {
                namespace: "".to_string(),
                local_name: "bar".to_string(),
            }),
            vec![]
        );
    }
}
