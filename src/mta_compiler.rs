use crate::{
    mta::{Automaton, Formula, FormulaId, Guard, State},
    TagType,
};

enum Axis {
    Descendant,
    Child,
    Self_,
    Attribute,
    FollowingSibling,
}

enum NodeTest {
    TagName {
        // none is match everything
        // empty url is match default namespace
        namespace: Option<String>,
        // none is match all local names
        local_name: Option<String>,
    },
    Text,
    Node,
}

struct LocationStep {
    axis: Axis,
    node_test: NodeTest,
    predicate: Option<Pred>,
}

enum Pred {
    And(Box<Pred>, Box<Pred>),
    Or(Box<Pred>, Box<Pred>),
    Not(Box<Pred>),
    Core(Core),
    // need extension for predicate functions and equality for text nodes
}

struct LocationPath {
    steps: Vec<LocationStep>,
}

enum Core {
    Relative(LocationPath),
    Absolute(LocationPath),
}

impl Core {
    fn translate(&self, automaton: &mut Automaton, state: State, in_main: bool) {
        match self {
            Core::Absolute(location_path) => {
                if in_main && location_path.steps.is_empty() {
                    automaton.add(state, Guard::include(TagType::Document), Formula::Mark);
                } else {
                    let downleft_state = State::new();
                    automaton.add(
                        state,
                        Guard::include(TagType::Document),
                        Formula::DownLeft(downleft_state),
                    );
                    location_path.translate(automaton, downleft_state, in_main);
                }
            }
            _ => unimplemented!(),
        }
    }
}

impl LocationPath {
    fn translate(&self, automaton: &mut Automaton, state: State, in_main: bool) {
        debug_assert!(!self.steps.is_empty());
        let last = self.steps.len() - 1;
        let mut next_state = state;
        for (i, step) in self.steps.iter().enumerate() {
            if i != last {
                next_state = step.translate(automaton, next_state, false);
            } else {
                // translate last step. If we're in main, mark it
                step.translate(automaton, next_state, in_main);
            }
        }
    }
}

impl LocationStep {
    fn translate(&self, automaton: &mut Automaton, state: State, mark: bool) -> State {
        match self.axis {
            Axis::Child => {
                unimplemented!();
                // let downleft_state = State::new();
                // automaton.add(state, guard, Formula::DownLeft(downleft_state));
                // automaton.add(state, )
            }
            Axis::Descendant => {
                let next_state = State::new();
                let formula = Formula::and(
                    Formula::DownLeft(state),
                    Formula::and(Formula::DownLeft(next_state), Formula::DownRight(state)),
                );
                let formula = if mark {
                    Formula::and(Formula::Mark, formula)
                } else {
                    formula
                };
                automaton.add(state, self.guard(), formula);
                automaton.add(
                    state,
                    Guard::all(),
                    Formula::and(Formula::DownLeft(state), Formula::DownRight(state)),
                );
                next_state
            }
            _ => unimplemented!(),
        }
    }

    fn guard(&self) -> Guard {
        match &self.node_test {
            NodeTest::TagName {
                namespace,
                local_name,
            } => {
                // TODO: namespace and wildcard handling
                // we construct the matching tag type
                let tag_type = TagType::Element {
                    namespace: "".to_string(),
                    local_name: local_name
                        .as_ref()
                        .expect("local name is not wildcard")
                        .to_string(),
                };
                Guard::include(tag_type)
            }
            _ => unimplemented!(),
        }
    }
}

#[cfg(test)]
mod tests {
    use ahash::HashSetExt;

    use crate::{
        mta::{Formula, Nodes, State, States},
        parse_document,
    };

    use super::*;

    #[test]
    fn test_manual_translation() {
        let d =
            parse_document(r#"<doc><listitem><p><keyword><emph/></keyword></p></listitem></doc>"#)
                .unwrap();
        let root = d.root();
        let doc = d.document_element();
        let listitem = d.first_child(doc).unwrap();
        let p = d.first_child(listitem).unwrap();
        let keyword = d.first_child(p).unwrap();
        let emph = d.first_child(keyword).unwrap();

        let mut automaton = Automaton::new();
        let q0 = automaton.start_state();
        let q1 = State::new();
        let q2 = State::new();
        let q3 = State::new();

        automaton.add(q0, Guard::include(TagType::Document), Formula::DownLeft(q1));
        // down left q1 and down left q2 and down right q1
        let formula = Formula::and(
            Formula::and(Formula::DownLeft(q1), Formula::DownLeft(q2)),
            Formula::DownRight(q1),
        );
        automaton.add(
            q1,
            Guard::include(TagType::Element {
                namespace: "".to_string(),
                local_name: "listitem".to_string(),
            }),
            formula,
        );
        automaton.add(
            q1,
            Guard::all(),
            Formula::and(Formula::DownRight(q1), Formula::DownLeft(q1)),
        );
        // mark and down left q2, and down left q3 and down right q2
        let formula = Formula::and(
            Formula::and(Formula::Mark, Formula::DownLeft(q2)),
            Formula::and(Formula::DownLeft(q3), Formula::DownRight(q2)),
        );
        automaton.add(
            q2,
            Guard::include(TagType::Element {
                namespace: "".to_string(),
                local_name: "keyword".to_string(),
            }),
            formula,
        );
        automaton.add(
            q2,
            Guard::all(),
            Formula::and(Formula::DownRight(q2), Formula::DownLeft(q2)),
        );
        automaton.add(
            q3,
            Guard::include(TagType::Element {
                namespace: "".to_string(),
                local_name: "emph".to_string(),
            }),
            Formula::True,
        );
        automaton.add(q3, Guard::all(), Formula::DownRight(q3));

        automaton.add_bottom_state(q1);
        automaton.add_bottom_state(q2);

        let marked = automaton.run(&d, root);

        assert_eq!(marked, vec![keyword].into_iter().collect::<Nodes>());
    }

    #[test]
    fn test_manual_translation_without_emph() {
        let d =
            parse_document(r#"<doc><listitem><p><keyword/></p><p><keyword/></p></listitem></doc>"#)
                .unwrap();
        let root = d.root();
        let doc = d.document_element();
        let listitem = d.first_child(doc).unwrap();
        let p = d.first_child(listitem).unwrap();
        let keyword = d.first_child(p).unwrap();
        let p2 = d.next_sibling(p).unwrap();
        let keyword2 = d.first_child(p2).unwrap();

        let mut automaton = Automaton::new();
        let q0 = automaton.start_state();
        let q1 = State::new();
        let q2 = State::new();

        automaton.add(q0, Guard::include(TagType::Document), Formula::DownLeft(q1));
        // down left q1 and down left q2 and down right q1
        let formula = Formula::and(
            Formula::and(Formula::DownLeft(q1), Formula::DownLeft(q2)),
            Formula::DownRight(q1),
        );
        automaton.add(
            q1,
            Guard::include(TagType::Element {
                namespace: "".to_string(),
                local_name: "listitem".to_string(),
            }),
            formula,
        );
        automaton.add(
            q1,
            Guard::all(),
            Formula::and(Formula::DownRight(q1), Formula::DownLeft(q1)),
        );
        let formula = Formula::and(
            Formula::and(Formula::Mark, Formula::DownLeft(q2)),
            Formula::DownRight(q2),
        );
        automaton.add(
            q2,
            Guard::include(TagType::Element {
                namespace: "".to_string(),
                local_name: "keyword".to_string(),
            }),
            formula,
        );
        automaton.add(
            q2,
            Guard::all(),
            Formula::and(Formula::DownRight(q2), Formula::DownLeft(q2)),
        );

        automaton.add_bottom_state(q1);
        automaton.add_bottom_state(q2);

        let marked = automaton.run(&d, root);

        assert_eq!(
            marked,
            vec![keyword, keyword2].into_iter().collect::<Nodes>()
        );
    }

    #[test]
    fn test_root() {
        let doc = parse_document(r#"<doc><a/><b/></doc>"#).unwrap();
        let root = doc.root();
        let path = Core::Absolute(LocationPath { steps: vec![] });
        let mut automaton = Automaton::new();
        let start_state = automaton.start_state();
        path.translate(&mut automaton, start_state, true);

        let marked = automaton.run(&doc, root);

        assert_eq!(marked, vec![root].into_iter().collect::<Nodes>());
    }

    #[test]
    fn test_descendants() {
        let d = parse_document(r#"<doc><listitem><p><keyword/></p></listitem></doc>"#).unwrap();
        let root = d.root();
        let doc = d.document_element();
        let listitem = d.first_child(doc).unwrap();
        let p = d.first_child(listitem).unwrap();
        let keyword = d.first_child(p).unwrap();

        let path = Core::Absolute(LocationPath {
            steps: vec![
                LocationStep {
                    axis: Axis::Descendant,
                    node_test: NodeTest::TagName {
                        namespace: Some("".to_string()),
                        local_name: Some("listitem".to_string()),
                    },
                    predicate: None,
                },
                LocationStep {
                    axis: Axis::Descendant,
                    node_test: NodeTest::TagName {
                        namespace: Some("".to_string()),
                        local_name: Some("keyword".to_string()),
                    },
                    predicate: None,
                },
            ],
        });

        let mut automaton = Automaton::new();
        let start_state = automaton.start_state();
        path.translate(&mut automaton, start_state, true);

        let marked = automaton.run(&d, root);

        assert_eq!(marked, vec![keyword].into_iter().collect::<Nodes>());
    }
}
