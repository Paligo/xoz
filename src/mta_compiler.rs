use crate::{
    mta::{Automaton, Guard},
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
    fn translate(&self, automaton: &mut Automaton) {
        match self {
            Core::Absolute(location_path) => {
                location_path.translate(automaton);
            }
            _ => unimplemented!(),
        }
    }
}

impl LocationPath {
    fn translate(&self, automaton: &mut Automaton) {
        for step in &self.steps {
            step.translate(automaton);
        }
    }
}

impl LocationStep {
    fn translate(&self, automaton: &mut Automaton) {
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
                let guard = Guard::Includes([tag_type].into_iter().collect());
                todo!()
            }
            _ => unimplemented!(),
        }
    }
}

#[cfg(test)]
mod tests {
    use ahash::HashSetExt;

    use crate::{
        mta::{self, Formula, Nodes, State, States},
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

        println!("Expected as marked: {:?}", keyword);

        let mut automaton = Automaton::new();
        let q0 = State::new();
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

        let mut states = States::new();
        states.insert(q0);
        let mut marked = Nodes::new();
        let mapping = automaton.top_down_run(&d, Some(root), states, &mut marked);

        assert_eq!(marked, vec![keyword].into_iter().collect::<Nodes>());
    }
    // #[test]
    // fn test_single_step_path() {
    //     let doc = parse_document(r#"<doc><a/></b></doc>"#).unwrap();
    //     let root = doc.root();
    //     let doc_el = doc.document_element();
    //     let a = doc.first_child(doc_el).unwrap();
    //     let b = doc.next_sibling(a).unwrap();

    //     let path = Core::Absolute(LocationPath {
    //         steps: vec![LocationStep {
    //             axis: Axis::Child,
    //             node_test: NodeTest::TagName {
    //                 namespace: None,
    //                 local_name: Some("doc".to_string()),
    //             },
    //             predicate: None,
    //         }],
    //     });

    //     let mut automaton = Automaton::new();

    //     path.translate(&mut automaton);

    //     let mut states = States::new();
    //     states.insert(automaton.start_state());
    //     let mapping = automaton.top_down_run(&doc, Some(root), states);
    //     dbg!(mapping);
    //     assert!(false);
    // }
}
