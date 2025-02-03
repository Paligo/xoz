use ahash::{HashMap, HashMapExt, HashSet};

use crate::{document::Node, TagType};

struct Automaton {
    tree_labels: usize,
}

type States = HashSet<State>;

#[derive(Debug, Copy, Clone, PartialEq, Eq, Hash)]
struct State(usize);

type Mapping = HashMap<State, HashSet<Node>>;

// fn top_down_run(automaton: Automaton, node: Option<Node>, states: States) -> Mapping {
//     if let Some(node) = node {
//     } else {
//         Mapping::new()
//     }
// }

struct StateLookup<'a, T: ?Sized> {
    states: HashMap<State, TagLookup<'a, T>>,
}

impl<'a, T: ?Sized> StateLookup<'a, T> {
    fn new() -> Self {
        Self {
            states: HashMap::new(),
        }
    }

    fn add(&mut self, state: State, tag_lookup: TagLookup<'a, T>) {
        self.states.insert(state, tag_lookup);
    }

    fn matching(&self, states: &States, tag: &TagType) -> Vec<&'a T> {
        states
            .iter()
            .flat_map(|state| self.states.get(state).map(|lookup| lookup.matching(tag)))
            .flatten()
            .collect()
    }
}

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

        // Add matches from excludes where tag is NOT in the excluded set
        results.extend(
            self.excludes
                .iter()
                .filter(|(tags, _)| !tags.contains(tag))
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
    fn test_tag_lookup_includes() {
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

    #[test]
    fn test_tag_lookup_excludes() {
        let mut lookup = TagLookup::new();

        let foo_tag = TagType::Element {
            namespace: "".to_string(),
            local_name: "foo".to_string(),
        };

        let bar_tag = TagType::Element {
            namespace: "".to_string(),
            local_name: "bar".to_string(),
        };

        // Test excludes
        let exclude_guard =
            Guard::Excludes([foo_tag.clone(), bar_tag.clone()].into_iter().collect());
        lookup.add(exclude_guard, "excluded");

        // Excluded tags should not match
        assert_eq!(lookup.matching(&foo_tag), Vec::<&str>::new());
        assert_eq!(lookup.matching(&bar_tag), Vec::<&str>::new());

        // Non-excluded tag should match
        let baz_tag = TagType::Element {
            namespace: "".to_string(),
            local_name: "baz".to_string(),
        };
        assert_eq!(lookup.matching(&baz_tag), vec!["excluded"]);

        // Test combination of includes and excludes
        let include_guard = Guard::Includes([foo_tag.clone()].into_iter().collect());
        lookup.add(include_guard, "included");

        // foo is excluded but also included
        assert_eq!(lookup.matching(&foo_tag), vec!["included"]);
        // bar is just excluded
        assert_eq!(lookup.matching(&bar_tag), Vec::<&str>::new());
        // baz matches the exclude guard
        assert_eq!(lookup.matching(&baz_tag), vec!["excluded"]);
    }

    #[test]
    fn test_state_lookup() {
        let mut lookup = StateLookup::new();
        let state1 = State(1);
        let state2 = State(2);
        let mut tag_lookup1 = TagLookup::new();
        let mut tag_lookup2 = TagLookup::new();
        let foo_tag = TagType::Element {
            namespace: "".to_string(),
            local_name: "foo".to_string(),
        };
        let bar_tag = TagType::Element {
            namespace: "".to_string(),
            local_name: "bar".to_string(),
        };
        tag_lookup1.add(
            Guard::Includes([foo_tag.clone()].into_iter().collect()),
            "value1",
        );
        tag_lookup2.add(
            Guard::Includes([bar_tag.clone()].into_iter().collect()),
            "value2",
        );
        lookup.add(state1, tag_lookup1);
        lookup.add(state2, tag_lookup2);

        let states = [state1, state2].iter().cloned().collect();
        assert_eq!(lookup.matching(&states, &foo_tag), vec!["value1"]);
        assert_eq!(lookup.matching(&states, &bar_tag), vec!["value2"]);
        let states = [state1].iter().cloned().collect();
        assert_eq!(lookup.matching(&states, &foo_tag), vec!["value1"]);
        assert_eq!(lookup.matching(&states, &bar_tag), Vec::<&str>::new());
        let states = [state2].iter().cloned().collect();
        assert_eq!(lookup.matching(&states, &foo_tag), Vec::<&str>::new());
        assert_eq!(lookup.matching(&states, &bar_tag), vec!["value2"]);
    }
}
