use std::hash::Hash;

use ahash::{HashMap, HashMapExt, HashSet, HashSetExt};

use crate::{
    document::{Document, Node},
    TagType,
};

pub(crate) type States = HashSet<State>;

#[derive(Debug, Copy, Clone, PartialEq, Eq, Hash)]
pub(crate) struct State(usize);

// a unique atomic static counter for state
static STATE_COUNTER: std::sync::atomic::AtomicUsize = std::sync::atomic::AtomicUsize::new(0);

// create a state using a unique atomic static counter
impl State {
    pub(crate) fn new() -> Self {
        State(STATE_COUNTER.fetch_add(1, std::sync::atomic::Ordering::Relaxed))
    }
}

#[derive(Debug, Copy, Clone, PartialEq, Eq, Hash)]
pub(crate) struct FormulaId(usize);

pub(crate) type Nodes = HashSet<Node>;
pub(crate) type Mapping = HashMap<State, Nodes>;

pub(crate) struct Automaton {
    formulas: Vec<Formula>,
    state_lookup: StateLookupFormula,
    start_state: State,
    bottom_states: States,
}

impl Automaton {
    pub(crate) fn new() -> Automaton {
        Automaton {
            formulas: Vec::new(),
            state_lookup: StateLookupFormula::new(),
            start_state: State::new(),
            bottom_states: States::new(),
        }
    }

    pub(crate) fn add(&mut self, state: State, guard: Guard, formula: Formula) -> FormulaId {
        let formula_id = FormulaId(self.formulas.len());
        self.formulas.push(formula);

        let tag_lookup = self.state_lookup.tag_lookup(state);
        if let Some(tag_lookup) = tag_lookup {
            tag_lookup.add(guard, formula_id);
        } else {
            let mut tag_lookup = TagLookupFormula::new();
            tag_lookup.add(guard, formula_id);
            self.state_lookup.add(state, tag_lookup);
        }
        formula_id
    }

    pub(crate) fn add_bottom_state(&mut self, state: State) {
        self.bottom_states.insert(state);
    }

    pub(crate) fn run(&self, document: &Document, node: Node) -> Nodes {
        let mut states = States::new();
        states.insert(self.start_state);
        let mut mapping = self.top_down_run(document, Some(node), states);
        mapping.remove(&self.start_state).unwrap_or_default()
    }

    pub(crate) fn top_down_run(
        &self,
        document: &Document,
        node: Option<Node>,
        states: States,
    ) -> Mapping {
        if let Some(node) = node {
            let trans = self.state_lookup.matching(&states, document.value(node));
            let mut left_states = States::new();
            let mut right_states = States::new();
            for (_q, formula_id) in &trans {
                let formula = &self.formulas[formula_id.0];
                left_states.extend(formula.down_left());
                right_states.extend(formula.down_right());
            }
            let left_mapping = self.top_down_run(document, document.first_child(node), left_states);
            let right_mapping =
                self.top_down_run(document, document.next_sibling(node), right_states);
            let mut mapping = Mapping::new();
            for (q, formula_id) in trans {
                let formula = &self.formulas[formula_id.0];
                let outcome = formula.evaluate(node, &left_mapping, &right_mapping);
                if outcome.b {
                    mapping.entry(q).or_default().extend(outcome.r);
                }
            }
            mapping
        } else {
            let mut mapping = Mapping::new();
            for state in states {
                if self.bottom_states.contains(&state) {
                    mapping.insert(state, Nodes::new());
                }
            }
            mapping
        }
    }

    pub(crate) fn start_state(&self) -> State {
        self.start_state
    }
}

#[derive(Debug)]
pub(crate) enum Formula {
    True,
    False,
    Mark,
    And(And),
    Or(Or),
    Not(Not),
    DownLeft(State),
    DownRight(State),
    Pred(Pred),
}

impl Formula {
    pub(crate) fn and(left: Formula, right: Formula) -> Formula {
        Formula::And(And {
            left: Box::new(left),
            right: Box::new(right),
        })
    }

    pub(crate) fn or(left: Formula, right: Formula) -> Formula {
        Formula::Or(Or {
            left: Box::new(left),
            right: Box::new(right),
        })
    }

    pub(crate) fn not(inner: Formula) -> Formula {
        Formula::Not(Not {
            inner: Box::new(inner),
        })
    }

    pub(crate) fn marking(self: Formula) -> Formula {
        Formula::And(And {
            right: Box::new(Formula::Mark),
            left: Box::new(self),
        })
    }

    fn evaluate(&self, node: Node, left: &Mapping, right: &Mapping) -> FormulaOutcome {
        match self {
            Formula::True => FormulaOutcome {
                b: true,
                r: Nodes::new(),
            },
            Formula::Mark => FormulaOutcome {
                b: true,
                r: {
                    let mut nodes = Nodes::new();
                    nodes.insert(node);
                    nodes
                },
            },
            Formula::And(and) => {
                let left_outcome = and.left.evaluate(node, left, right);
                let right_outcome = and.right.evaluate(node, left, right);
                left_outcome.and(&right_outcome)
            }
            Formula::Or(or) => {
                let left_outcome = or.left.evaluate(node, left, right);
                let right_outcome = or.right.evaluate(node, left, right);
                left_outcome.or(&right_outcome)
            }
            Formula::Not(not) => {
                let inner = not.inner.evaluate(node, left, right);
                inner.not()
            }
            Formula::DownLeft(state) => {
                let nodes = left.get(state).cloned().unwrap_or_default();
                FormulaOutcome { b: true, r: nodes }
                // if let Some(nodes) = nodes {
                //     if nodes.contains(&node) {
                //         return FormulaOutcome {
                //             b: true,
                //             r: nodes.clone(),
                //         };
                //     }
                // }
                // FormulaOutcome {
                //     b: false,
                //     r: Nodes::new(),
                // }
            }
            Formula::DownRight(state) => {
                let nodes = right.get(state).cloned().unwrap_or_default();
                FormulaOutcome { b: true, r: nodes }

                // let nodes = right.get(state);
                // if let Some(nodes) = nodes {
                //     if nodes.contains(&node) {
                //         return FormulaOutcome {
                //             b: true,
                //             r: nodes.clone(),
                //         };
                //     }
                // }
                // FormulaOutcome {
                //     b: false,
                //     r: Nodes::new(),
                // }
            }
            Formula::Pred(pred) => {
                todo!()
            }
            Formula::False => FormulaOutcome {
                b: false,
                r: Nodes::new(),
            },
        }
    }

    // get all states that are in a down1 ast node
    fn down_left(&self) -> States {
        match self {
            Formula::DownLeft(state) => {
                let mut states = States::new();
                states.insert(*state);
                states
            }
            Formula::And(and) => and
                .left
                .down_left()
                .union(&and.right.down_left())
                .cloned()
                .collect(),
            Formula::Or(or) => or
                .left
                .down_left()
                .union(&or.right.down_left())
                .cloned()
                .collect(),
            Formula::Not(not) => not.inner.down_left(),
            _ => States::new(),
        }
    }

    fn down_right(&self) -> States {
        match self {
            Formula::DownRight(state) => {
                let mut states = States::new();
                states.insert(*state);
                states
            }
            Formula::And(and) => and
                .left
                .down_right()
                .union(&and.right.down_right())
                .cloned()
                .collect(),
            Formula::Or(or) => or
                .left
                .down_right()
                .union(&or.right.down_right())
                .cloned()
                .collect(),
            Formula::Not(not) => not.inner.down_right(),
            _ => States::new(),
        }
    }
}

#[derive(Debug)]
pub(crate) struct And {
    pub(crate) left: Box<Formula>,
    pub(crate) right: Box<Formula>,
}

#[derive(Debug)]
pub(crate) struct Or {
    pub(crate) left: Box<Formula>,
    pub(crate) right: Box<Formula>,
}

#[derive(Debug)]
pub(crate) struct Not {
    pub(crate) inner: Box<Formula>,
}

#[derive(Debug)]
pub(crate) struct Predicate;

#[derive(Debug)]
pub(crate) struct Pred {
    pred: Predicate,
}

#[derive(Debug)]
pub(crate) struct FormulaOutcome {
    b: bool,
    r: Nodes,
}

impl FormulaOutcome {
    fn not(&self) -> FormulaOutcome {
        FormulaOutcome {
            b: !self.b,
            r: Nodes::new(),
        }
    }

    fn and(&self, other: &FormulaOutcome) -> FormulaOutcome {
        if self.b && other.b {
            FormulaOutcome {
                b: true,
                r: self.r.union(&other.r).cloned().collect(),
            }
        } else {
            FormulaOutcome {
                b: false,
                r: Nodes::new(),
            }
        }
    }

    fn or(&self, other: &FormulaOutcome) -> FormulaOutcome {
        match (self.b, other.b) {
            (true, true) => FormulaOutcome {
                b: true,
                r: self.r.union(&other.r).cloned().collect(),
            },
            (true, false) => FormulaOutcome {
                b: true,
                r: self.r.clone(),
            },
            (false, true) => FormulaOutcome {
                b: true,
                r: other.r.clone(),
            },
            (false, false) => FormulaOutcome {
                b: false,
                r: Nodes::new(),
            },
        }
    }
}

pub(crate) type StateLookupFormula = StateLookup<FormulaId>;

pub(crate) struct StateLookup<T: Copy> {
    states: HashMap<State, TagLookup<T>>,
}

impl<T: Copy> StateLookup<T> {
    pub(crate) fn new() -> Self {
        Self {
            states: HashMap::new(),
        }
    }

    pub(crate) fn add(&mut self, state: State, tag_lookup: TagLookup<T>) {
        self.states.insert(state, tag_lookup);
    }

    fn tag_lookup(&mut self, state: State) -> Option<&mut TagLookup<T>> {
        self.states.get_mut(&state)
    }

    fn matching(&self, states: &States, tag: &TagType) -> Vec<(State, T)> {
        let mut results = Vec::new();

        for state in states {
            if let Some(tag_lookup) = self.states.get(state) {
                results.extend(
                    tag_lookup
                        .matching(tag)
                        .iter()
                        .map(|payload| (*state, *payload)),
                );
            }
        }
        results
    }
}

pub(crate) type TagLookupFormula = TagLookup<FormulaId>;

pub(crate) struct TagLookup<T: Copy> {
    // Direct mapping for includes
    includes: HashMap<TagType, Vec<T>>,
    // For excludes, we store (excluded_tags, payload) pairs
    excludes: Vec<(HashSet<TagType>, T)>,
}

impl<T: Copy> TagLookup<T> {
    pub(crate) fn new() -> Self {
        Self {
            includes: HashMap::new(),
            excludes: Vec::new(),
        }
    }

    pub(crate) fn add(&mut self, guard: Guard, payload: T) {
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

    fn matching(&self, tag: &TagType) -> Vec<T> {
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
pub(crate) enum Guard {
    Includes(HashSet<TagType>),
    Excludes(HashSet<TagType>),
}

impl Guard {
    pub(crate) fn includes(tags: Vec<TagType>) -> Self {
        Guard::Includes(tags.into_iter().collect())
    }
    pub(crate) fn excludes(tags: Vec<TagType>) -> Self {
        Guard::Excludes(tags.into_iter().collect())
    }

    pub(crate) fn include(tag: TagType) -> Self {
        Guard::Includes([tag].into_iter().collect())
    }

    pub(crate) fn exclude(tag: TagType) -> Self {
        Guard::Excludes([tag].into_iter().collect())
    }

    pub(crate) fn all() -> Self {
        // excluding nothing is including anything
        Guard::Excludes(HashSet::new())
    }
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
        assert_eq!(lookup.matching(&states, &foo_tag), vec![(state1, "value1")]);
        assert_eq!(lookup.matching(&states, &bar_tag), vec![(state2, "value2")]);
        let states = [state1].iter().cloned().collect();
        assert_eq!(lookup.matching(&states, &foo_tag), vec![(state1, "value1")]);
        assert_eq!(
            lookup.matching(&states, &bar_tag),
            Vec::<(State, &str)>::new()
        );
        let states = [state2].iter().cloned().collect();
        assert_eq!(
            lookup.matching(&states, &foo_tag),
            Vec::<(State, &str)>::new()
        );
        assert_eq!(lookup.matching(&states, &bar_tag), vec![(state2, "value2")]);
    }
}
