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

struct TagLookup<'a, T: ?Sized> {
    // Direct mapping for includes
    includes: HashMap<TagType, Vec<&'a T>>,
    // For excludes, we store (excluded_tags, payload) pairs
    excludes: Vec<(HashSet<TagType>, &'a T)>,
}

impl<'a, T: ?Sized> TagLookup<'a, T> {
    fn new() -> Self {
        Self {
            includes: HashMap::new(),
            excludes: Vec::new(),
        }
    }

    fn add(&mut self, guard: Guard, payload: &'a T) {
        match guard {
            Guard::Includes(tags) => {
                // For includes, add the payload to each tag's vector
                for tag in tags {
                    self.includes.entry(tag).or_default().push(payload);
                }
            }
            Guard::Excludes(tags) => {
                // For excludes, store the whole set with its payload
                self.excludes.push((tags, payload));
            }
        }
    }

    fn matching(&self, tag: &TagType) -> Vec<&'a T> {
        let mut results = Vec::new();

        // Add all direct matches from includes
        if let Some(payloads) = self.includes.get(tag) {
            results.extend(payloads.iter().cloned());
        }

        // Add matches from excludes where tag is in the excluded set
        results.extend(
            self.excludes
                .iter()
                .filter(|(tags, _)| tags.contains(tag))
                .map(|(_, payload)| payload),
        );

        results
    }
}

#[derive(Debug, Eq, PartialEq, Clone)]
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

        // Test includes
        let guard = Guard::Includes(
            [TagType::Element {
                namespace: "".to_string(),
                local_name: "foo".to_string(),
            }]
            .into_iter()
            .collect(),
        );
        lookup.add(guard.clone(), "value1");

        // Add another payload for the same tag
        lookup.add(guard, "value2");

        let foo_tag = TagType::Element {
            namespace: "".to_string(),
            local_name: "foo".to_string(),
        };

        let bar_tag = TagType::Element {
            namespace: "".to_string(),
            local_name: "bar".to_string(),
        };

        assert_eq!(lookup.matching(&foo_tag), vec!["value1", "value2"]);

        assert_eq!(lookup.matching(&bar_tag), Vec::<&str>::new());
    }
}
