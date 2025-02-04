use crate::mta::Automaton;

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
    fn translate(&self) -> Automaton {
        match self {
            Core::Relative(path) | Core::Absolute(path) => {
                let mut states = HashSet::new();
                let mut automaton = Automaton::new(states.clone());
                let mut current_state = State(0);
                
                for step in &path.steps {
                    let next_state = State(current_state.0 + 1);
                    
                    match &step.axis {
                        Axis::Child => {
                            match &step.node_test {
                                NodeTest::TagName { namespace, local_name } => {
                                    let mut tags = HashSet::new();
                                    // Match specific tag
                                    tags.insert(TagType::Element {
                                        namespace: namespace.clone().unwrap_or_default(),
                                        local_name: local_name.clone().unwrap_or_default(),
                                    });
                                    let guard = Guard::Includes(tags);
                                    automaton.add(
                                        current_state,
                                        guard,
                                        Formula::And(And {
                                            left: Box::new(Formula::DownLeft(next_state)),
                                            right: Box::new(Formula::Mark),
                                        }),
                                    );
                                }
                                NodeTest::Text => {
                                    let mut tags = HashSet::new();
                                    tags.insert(TagType::Text);
                                    let guard = Guard::Includes(tags);
                                    automaton.add(
                                        current_state,
                                        guard,
                                        Formula::And(And {
                                            left: Box::new(Formula::DownLeft(next_state)),
                                            right: Box::new(Formula::Mark),
                                        }),
                                    );
                                }
                                NodeTest::Node => {
                                    // Match any node
                                    let guard = Guard::Excludes(HashSet::new());
                                    automaton.add(
                                        current_state,
                                        guard,
                                        Formula::And(And {
                                            left: Box::new(Formula::DownLeft(next_state)),
                                            right: Box::new(Formula::Mark),
                                        }),
                                    );
                                }
                            }
                        }
                        Axis::Descendant => {
                            match &step.node_test {
                                NodeTest::TagName { namespace, local_name } => {
                                    let mut tags = HashSet::new();
                                    tags.insert(TagType::Element {
                                        namespace: namespace.clone().unwrap_or_default(),
                                        local_name: local_name.clone().unwrap_or_default(),
                                    });
                                    let guard = Guard::Includes(tags);
                                    // Recursive case - continue looking for descendants
                                    automaton.add(
                                        current_state,
                                        Guard::Excludes(HashSet::new()),
                                        Formula::DownLeft(current_state),
                                    );
                                    // Base case - match the node
                                    automaton.add(
                                        current_state,
                                        guard,
                                        Formula::And(And {
                                            left: Box::new(Formula::DownLeft(next_state)),
                                            right: Box::new(Formula::Mark),
                                        }),
                                    );
                                }
                                NodeTest::Text => {
                                    let mut tags = HashSet::new();
                                    tags.insert(TagType::Text);
                                    let guard = Guard::Includes(tags);
                                    // Recursive case
                                    automaton.add(
                                        current_state,
                                        Guard::Excludes(HashSet::new()),
                                        Formula::DownLeft(current_state),
                                    );
                                    // Base case
                                    automaton.add(
                                        current_state,
                                        guard,
                                        Formula::And(And {
                                            left: Box::new(Formula::DownLeft(next_state)),
                                            right: Box::new(Formula::Mark),
                                        }),
                                    );
                                }
                                NodeTest::Node => {
                                    // Recursive case
                                    automaton.add(
                                        current_state,
                                        Guard::Excludes(HashSet::new()),
                                        Formula::DownLeft(current_state),
                                    );
                                    // Base case
                                    automaton.add(
                                        current_state,
                                        Guard::Excludes(HashSet::new()),
                                        Formula::And(And {
                                            left: Box::new(Formula::DownLeft(next_state)),
                                            right: Box::new(Formula::Mark),
                                        }),
                                    );
                                }
                            }
                        }
                        Axis::Self_ => {
                            match &step.node_test {
                                NodeTest::TagName { namespace, local_name } => {
                                    let mut tags = HashSet::new();
                                    tags.insert(TagType::Element {
                                        namespace: namespace.clone().unwrap_or_default(),
                                        local_name: local_name.clone().unwrap_or_default(),
                                    });
                                    let guard = Guard::Includes(tags);
                                    automaton.add(
                                        current_state,
                                        guard,
                                        Formula::And(And {
                                            left: Box::new(Formula::DownLeft(next_state)),
                                            right: Box::new(Formula::Mark),
                                        }),
                                    );
                                }
                                NodeTest::Text => {
                                    let mut tags = HashSet::new();
                                    tags.insert(TagType::Text);
                                    let guard = Guard::Includes(tags);
                                    automaton.add(
                                        current_state,
                                        guard,
                                        Formula::And(And {
                                            left: Box::new(Formula::DownLeft(next_state)),
                                            right: Box::new(Formula::Mark),
                                        }),
                                    );
                                }
                                NodeTest::Node => {
                                    automaton.add(
                                        current_state,
                                        Guard::Excludes(HashSet::new()),
                                        Formula::And(And {
                                            left: Box::new(Formula::DownLeft(next_state)),
                                            right: Box::new(Formula::Mark),
                                        }),
                                    );
                                }
                            }
                        }
                        Axis::Attribute => {
                            if let NodeTest::TagName { namespace, local_name } = &step.node_test {
                                let mut tags = HashSet::new();
                                tags.insert(TagType::Attribute {
                                    namespace: namespace.clone().unwrap_or_default(),
                                    local_name: local_name.clone().unwrap_or_default(),
                                });
                                let guard = Guard::Includes(tags);
                                automaton.add(
                                    current_state,
                                    guard,
                                    Formula::And(And {
                                        left: Box::new(Formula::DownLeft(next_state)),
                                        right: Box::new(Formula::Mark),
                                    }),
                                );
                            }
                        }
                        Axis::FollowingSibling => {
                            match &step.node_test {
                                NodeTest::TagName { namespace, local_name } => {
                                    let mut tags = HashSet::new();
                                    tags.insert(TagType::Element {
                                        namespace: namespace.clone().unwrap_or_default(),
                                        local_name: local_name.clone().unwrap_or_default(),
                                    });
                                    let guard = Guard::Includes(tags);
                                    automaton.add(
                                        current_state,
                                        guard,
                                        Formula::And(And {
                                            left: Box::new(Formula::DownRight(next_state)),
                                            right: Box::new(Formula::Mark),
                                        }),
                                    );
                                }
                                NodeTest::Text => {
                                    let mut tags = HashSet::new();
                                    tags.insert(TagType::Text);
                                    let guard = Guard::Includes(tags);
                                    automaton.add(
                                        current_state,
                                        guard,
                                        Formula::And(And {
                                            left: Box::new(Formula::DownRight(next_state)),
                                            right: Box::new(Formula::Mark),
                                        }),
                                    );
                                }
                                NodeTest::Node => {
                                    automaton.add(
                                        current_state,
                                        Guard::Excludes(HashSet::new()),
                                        Formula::And(And {
                                            left: Box::new(Formula::DownRight(next_state)),
                                            right: Box::new(Formula::Mark),
                                        }),
                                    );
                                }
                            }
                        }
                    }
                    
                    if step.predicate.is_some() {
                        // TODO: Handle predicates
                    }
                    
                    current_state = next_state;
                }
                
                // Mark the final state as accepting
                states.insert(current_state);
                automaton.bottom_states = states;
                
                automaton
            }
        }
    }
}
