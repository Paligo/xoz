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

struct TagLookup<T> {
    // Direct mapping for includes
    includes: HashMap<TagType, T>,
    // For excludes, we map each excluded tag to payloads that exclude it
    excludes: HashMap<TagType, Vec<(HashSet<TagType>, T)>>,
}

impl<T: Clone> TagLookup<T> {
    fn new() -> Self {
        Self {
            includes: HashMap::new(),
            excludes: HashMap::new(),
        }
    }

    fn add(&mut self, guard: Guard, payload: T) {
        match guard {
            Guard::Includes(tags) => {
                // For includes, we add the payload for each included tag
                for tag in tags {
                    self.includes.insert(tag, payload.clone());
                }
            }
            Guard::Excludes(tags) => {
                // For excludes, we add the payload to each excluded tag's list
                for tag in &tags {
                    self.excludes
                        .entry(tag.clone())
                        .or_default()
                        .push((tags.clone(), payload.clone()));
                }
            }
        }
    }

    fn get(&self, tag: TagType) -> Option<&T> {
        // First check direct includes
        if let Some(payload) = self.includes.get(&tag) {
            return Some(payload);
        }

        // Then check excludes - if this tag is in excludes map,
        // we need to find a payload that excludes it
        if let Some(excluded_payloads) = self.excludes.get(&tag) {
            // Return the first payload where tag is in the excluded set
            return excluded_payloads
                .iter()
                .find(|(tags, _)| tags.contains(&tag))
                .map(|(_, payload)| payload);
        }

        None
    }
}

enum Guard {
    Includes(HashSet<TagType>),
    Excludes(HashSet<TagType>),
}

impl Guard {
    fn includes<I>(tags: I) -> Self 
    where
        I: IntoIterator<Item = TagType>,
    {
        Guard::Includes(tags.into_iter().collect())
    }
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

    #[test]
    fn test_tag_lookup_excludes() {
        let mut lookup = TagLookup::new();
        let guard = Guard::Excludes([
            TagType::Element {
                namespace: "".to_string(),
                local_name: "foo".to_string(),
            },
            TagType::Element {
                namespace: "".to_string(),
                local_name: "bar".to_string(),
            },
        ].into_iter().collect());
        
        lookup.add(guard, "excluded");

        assert_eq!(
            lookup.get(TagType::Element {
                namespace: "".to_string(),
                local_name: "foo".to_string(),
            }),
            Some(&"excluded")
        );
        assert_eq!(
            lookup.get(TagType::Element {
                namespace: "".to_string(),
                local_name: "bar".to_string(),
            }),
            Some(&"excluded")
        );
        assert_eq!(
            lookup.get(TagType::Element {
                namespace: "".to_string(),
                local_name: "baz".to_string(),
            }),
            None
        );
    }
}
