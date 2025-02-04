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
    fn translate(&self) -> Automaton {}
}
