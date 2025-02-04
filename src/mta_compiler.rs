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
        let mut automaton = Automaton::new();
        
        match self {
            Core::Relative(path) | Core::Absolute(path) => {
                if path.steps.is_empty() {
                    let state = State::new();
                    automaton.add(state, Guard::Excludes(HashSet::new()), Formula::Mark);
                    automaton.add_bottom_state(state);
                    return automaton;
                }

                let mut current_state = State::new();
                
                // Process all steps except the last one
                for step in path.steps.iter().take(path.steps.len() - 1) {
                    let next_state = State::new();
                    
                    match &step.axis {
                        Axis::Child => {
                            let guard = match &step.node_test {
                                NodeTest::TagName { namespace, local_name } => {
                                    let mut tags = HashSet::new();
                                    tags.insert(TagType::Element {
                                        namespace: namespace.clone().unwrap_or_default(),
                                        local_name: local_name.clone().unwrap_or_default(),
                                    });
                                    Guard::Includes(tags)
                                }
                                NodeTest::Text => {
                                    let mut tags = HashSet::new();
                                    tags.insert(TagType::Text);
                                    Guard::Includes(tags)
                                }
                                NodeTest::Node => Guard::Excludes(HashSet::new()),
                            };
                            automaton.add(current_state, guard, Formula::DownLeft(next_state));
                        }
                        Axis::Descendant => {
                            let guard = match &step.node_test {
                                NodeTest::TagName { namespace, local_name } => {
                                    let mut tags = HashSet::new();
                                    tags.insert(TagType::Element {
                                        namespace: namespace.clone().unwrap_or_default(),
                                        local_name: local_name.clone().unwrap_or_default(),
                                    });
                                    Guard::Includes(tags)
                                }
                                NodeTest::Text => {
                                    let mut tags = HashSet::new();
                                    tags.insert(TagType::Text);
                                    Guard::Includes(tags)
                                }
                                NodeTest::Node => Guard::Excludes(HashSet::new()),
                            };
                            // Recursive case - continue looking for descendants
                            automaton.add(current_state, Guard::Excludes(HashSet::new()), 
                                Formula::DownLeft(current_state));
                            // Base case - move to next state
                            automaton.add(current_state, guard, Formula::DownLeft(next_state));
                        }
                        Axis::Self_ => {
                            let guard = match &step.node_test {
                                NodeTest::TagName { namespace, local_name } => {
                                    let mut tags = HashSet::new();
                                    tags.insert(TagType::Element {
                                        namespace: namespace.clone().unwrap_or_default(),
                                        local_name: local_name.clone().unwrap_or_default(),
                                    });
                                    Guard::Includes(tags)
                                }
                                NodeTest::Text => {
                                    let mut tags = HashSet::new();
                                    tags.insert(TagType::Text);
                                    Guard::Includes(tags)
                                }
                                NodeTest::Node => Guard::Excludes(HashSet::new()),
                            };
                            automaton.add(current_state, guard, Formula::DownLeft(next_state));
                        }
                        Axis::Attribute => {
                            if let NodeTest::TagName { namespace, local_name } = &step.node_test {
                                let mut tags = HashSet::new();
                                tags.insert(TagType::Attribute {
                                    namespace: namespace.clone().unwrap_or_default(),
                                    local_name: local_name.clone().unwrap_or_default(),
                                });
                                automaton.add(current_state, Guard::Includes(tags), 
                                    Formula::DownLeft(next_state));
                            }
                        }
                        Axis::FollowingSibling => {
                            let guard = match &step.node_test {
                                NodeTest::TagName { namespace, local_name } => {
                                    let mut tags = HashSet::new();
                                    tags.insert(TagType::Element {
                                        namespace: namespace.clone().unwrap_or_default(),
                                        local_name: local_name.clone().unwrap_or_default(),
                                    });
                                    Guard::Includes(tags)
                                }
                                NodeTest::Text => {
                                    let mut tags = HashSet::new();
                                    tags.insert(TagType::Text);
                                    Guard::Includes(tags)
                                }
                                NodeTest::Node => Guard::Excludes(HashSet::new()),
                            };
                            automaton.add(current_state, guard, Formula::DownRight(next_state));
                        }
                    }
                    
                    if let Some(pred) = &step.predicate {
                        // TODO: Handle predicates
                    }
                    
                    current_state = next_state;
                }
                
                // Handle the last step - similar to above but with Mark
                if let Some(last_step) = path.steps.last() {
                    let final_state = State::new();
                    
                    match &last_step.axis {
                        Axis::Child => {
                            let guard = match &last_step.node_test {
                                NodeTest::TagName { namespace, local_name } => {
                                    let mut tags = HashSet::new();
                                    tags.insert(TagType::Element {
                                        namespace: namespace.clone().unwrap_or_default(),
                                        local_name: local_name.clone().unwrap_or_default(),
                                    });
                                    Guard::Includes(tags)
                                }
                                NodeTest::Text => {
                                    let mut tags = HashSet::new();
                                    tags.insert(TagType::Text);
                                    Guard::Includes(tags)
                                }
                                NodeTest::Node => Guard::Excludes(HashSet::new()),
                            };
                            automaton.add(current_state, guard, 
                                Formula::And(And {
                                    left: Box::new(Formula::DownLeft(final_state)),
                                    right: Box::new(Formula::Mark),
                                }));
                        }
                        Axis::Descendant => {
                            let guard = match &last_step.node_test {
                                NodeTest::TagName { namespace, local_name } => {
                                    let mut tags = HashSet::new();
                                    tags.insert(TagType::Element {
                                        namespace: namespace.clone().unwrap_or_default(),
                                        local_name: local_name.clone().unwrap_or_default(),
                                    });
                                    Guard::Includes(tags)
                                }
                                NodeTest::Text => {
                                    let mut tags = HashSet::new();
                                    tags.insert(TagType::Text);
                                    Guard::Includes(tags)
                                }
                                NodeTest::Node => Guard::Excludes(HashSet::new()),
                            };
                            automaton.add(current_state, Guard::Excludes(HashSet::new()), 
                                Formula::DownLeft(current_state));
                            automaton.add(current_state, guard,
                                Formula::And(And {
                                    left: Box::new(Formula::DownLeft(final_state)),
                                    right: Box::new(Formula::Mark),
                                }));
                        }
                        Axis::Self_ => {
                            let guard = match &last_step.node_test {
                                NodeTest::TagName { namespace, local_name } => {
                                    let mut tags = HashSet::new();
                                    tags.insert(TagType::Element {
                                        namespace: namespace.clone().unwrap_or_default(),
                                        local_name: local_name.clone().unwrap_or_default(),
                                    });
                                    Guard::Includes(tags)
                                }
                                NodeTest::Text => {
                                    let mut tags = HashSet::new();
                                    tags.insert(TagType::Text);
                                    Guard::Includes(tags)
                                }
                                NodeTest::Node => Guard::Excludes(HashSet::new()),
                            };
                            automaton.add(current_state, guard,
                                Formula::And(And {
                                    left: Box::new(Formula::DownLeft(final_state)),
                                    right: Box::new(Formula::Mark),
                                }));
                        }
                        Axis::Attribute => {
                            if let NodeTest::TagName { namespace, local_name } = &last_step.node_test {
                                let mut tags = HashSet::new();
                                tags.insert(TagType::Attribute {
                                    namespace: namespace.clone().unwrap_or_default(),
                                    local_name: local_name.clone().unwrap_or_default(),
                                });
                                automaton.add(current_state, Guard::Includes(tags),
                                    Formula::And(And {
                                        left: Box::new(Formula::DownLeft(final_state)),
                                        right: Box::new(Formula::Mark),
                                    }));
                            }
                        }
                        Axis::FollowingSibling => {
                            let guard = match &last_step.node_test {
                                NodeTest::TagName { namespace, local_name } => {
                                    let mut tags = HashSet::new();
                                    tags.insert(TagType::Element {
                                        namespace: namespace.clone().unwrap_or_default(),
                                        local_name: local_name.clone().unwrap_or_default(),
                                    });
                                    Guard::Includes(tags)
                                }
                                NodeTest::Text => {
                                    let mut tags = HashSet::new();
                                    tags.insert(TagType::Text);
                                    Guard::Includes(tags)
                                }
                                NodeTest::Node => Guard::Excludes(HashSet::new()),
                            };
                            automaton.add(current_state, guard,
                                Formula::And(And {
                                    left: Box::new(Formula::DownRight(final_state)),
                                    right: Box::new(Formula::Mark),
                                }));
                        }
                    }
                    
                    if let Some(pred) = &last_step.predicate {
                        // TODO: Handle predicates
                    }
                    
                    automaton.add_bottom_state(final_state);
                }
            }
        }
        
        automaton
    }
}
