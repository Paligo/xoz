use quick_xml::Error as QuickXMLError;

use crate::document::{Document, DocumentId, Node as DocumentNode};
use crate::parser::parse_document_with_id;

use crate::{NodeName, NodeType, ProcessingInstruction};

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub struct Node {
    document_id: DocumentId,
    document_node: DocumentNode,
}

/// The Xoz structure is a pool of multiple documents in your application.
///
/// All operations on nodes are done through this structure. Behavior of
/// nodes from different Xoz structures is undefined.
///
/// You can add documents to the pool but otherwise the documents are immutable.
pub struct Xoz {
    documents: Vec<Document>,
}

impl Document {
    fn new_node(&self, document_node: DocumentNode) -> Node {
        Node {
            document_id: self.id,
            document_node,
        }
    }
}

impl Default for Xoz {
    fn default() -> Self {
        Self::new()
    }
}

impl Xoz {
    pub fn new() -> Self {
        Xoz {
            documents: Vec::new(),
        }
    }

    fn new_document_id(&self) -> DocumentId {
        DocumentId::new(self.documents.len())
    }

    fn document(&self, id: DocumentId) -> &Document {
        &self.documents[id.index()]
    }

    fn wrap(&self, node: Node, f: impl Fn(&Document, DocumentNode) -> DocumentNode) -> Node {
        let document = self.document(node.document_id);
        document.new_node(f(document, node.document_node))
    }

    fn wrap_option(
        &self,
        node: Node,
        f: impl Fn(&Document, DocumentNode) -> Option<DocumentNode>,
    ) -> Option<Node> {
        let document = self.document(node.document_id);
        f(document, node.document_node).map(|n| document.new_node(n))
    }

    // parse
    pub fn parse_str(&mut self, xml: &str) -> Result<Node, QuickXMLError> {
        let document = parse_document_with_id(self.new_document_id(), xml)?;
        let root = document.root();
        let root = document.new_node(root);
        self.documents.push(document);
        Ok(root)
    }

    // nav
    
    /// Obtain the document element.
    ///
    /// ```rust
    /// use xoz::Xoz;
    /// let mut xoz = Xoz::new();
    /// let root = xoz.parse_str("<p>Example</p>").unwrap();
    ///
    /// let doc_el = xoz.document_element(root);
    ///
    /// assert!(xoz.is_element(doc_el));
    /// assert_eq!(xoz.parent(doc_el), Some(root));
    /// ```
    pub fn document_element(&self, root: Node) -> Node {
        let document = self.document(root.document_id);
        document.new_node(document.document_element())
    }

    /// Get parent node.
    ///
    /// Returns [`None`] if this is the document node or if the node is
    /// unattached to a document.
    ///
    /// Attribute and namespace nodes have a parent, even though they aren't
    /// children of the element they are in.
    ///
    /// ```rust
    /// use xoz::Xoz;
    /// let mut xoz = Xoz::new();
    /// let root = xoz.parse_str("<p>Example</p>").unwrap();
    /// let p = xoz.document_element(root);
    /// let text = xoz.first_child(p).unwrap();
    ///
    /// assert_eq!(xoz.parent(text), Some(p));
    /// assert_eq!(xoz.parent(p), Some(root));
    /// assert_eq!(xoz.parent(root), None);
    /// ```
    pub fn parent(&self, node: Node) -> Option<Node> {
        self.wrap_option(node, |doc, n| doc.parent(n))
    }

    /// Get first child.
    ///
    /// Returns [`None`] if there are no children.
    ///
    /// ```rust
    /// use xoz::Xoz;
    /// let mut xoz = Xoz::new();
    /// let root = xoz.parse_str("<p>Example</p>").unwrap();
    /// let p = xoz.document_element(root);
    /// let text = xoz.first_child(p).unwrap();
    /// assert_eq!(xoz.first_child(root), Some(p));
    /// assert_eq!(xoz.first_child(p), Some(text));
    /// assert_eq!(xoz.first_child(text), None);
    /// ```
    pub fn first_child(&self, node: Node) -> Option<Node> {
        self.wrap_option(node, |doc, n| doc.first_child(n))
    }

    /// Get last child.
    ///
    /// Returns [`None`] if there are no children.
    pub fn last_child(&self, node: Node) -> Option<Node> {
        self.wrap_option(node, |doc, n| doc.last_child(n))
    }

    /// Get next sibling.
    ///
    /// Returns [`None`] if there is no next sibling.
    ///
    /// For normal child nodes, gives the next child.
    ///
    /// For namespace and attribute nodes, gives the next namespace or
    /// attribute in definition order.
    ///
    /// ```rust
    /// use xoz::Xoz;
    /// let mut xoz = Xoz::new();
    /// let root = xoz.parse_str("<p><a/><b/></p>").unwrap();
    /// let p = xoz.document_element(root);
    /// let a = xoz.first_child(p).unwrap();
    /// let b = xoz.next_sibling(a).unwrap();
    /// assert_eq!(xoz.next_sibling(b), None);
    /// ```
    pub fn next_sibling(&self, node: Node) -> Option<Node> {
        self.wrap_option(node, |doc, n| doc.next_sibling(n))
    }

    /// Get previous sibling.
    ///
    /// Returns [`None`] if there is no previous sibling.
    pub fn previous_sibling(&self, node: Node) -> Option<Node> {
        self.wrap_option(node, |doc, n| doc.previous_sibling(n))
    }

    /// If ancestor is an ancestor of descendant, return true.
    /// The ancestor node is not considered a descendant of itself.
    pub fn is_ancestor(&self, ancestor: Node, descendant: Node) -> bool {
        let ancestor_document_id = ancestor.document_id;
        let descendant_document_id = descendant.document_id;
        if ancestor_document_id != descendant_document_id {
            return false;
        }
        let document = self.document(ancestor_document_id);
        document.is_ancestor(ancestor.document_node, descendant.document_node)
    }

    /// If ancestor is an ancestor of descendant, return true.
    /// A node is considered a descendant of itself.
    pub fn is_ancestor_or_self(&self, ancestor: Node, descendant: Node) -> bool {
        let ancestor_document_id = ancestor.document_id;
        let descendant_document_id = descendant.document_id;
        if ancestor_document_id != descendant_document_id {
            return false;
        }
        let document = self.document(ancestor_document_id);
        document.is_ancestor_or_self(ancestor.document_node, descendant.document_node)
    }

    /// Obtain top node, given node anywhere in a tree
    ///
    /// In an XML document this is the document element.
    pub fn top_element(&self, node: Node) -> Node {
        self.wrap(node, |doc, n| doc.top_element(n))
    }

    /// Return true if node is directly under the document node.
    ///
    /// This means it's either the document element or a comment or processing
    /// instruction.
    ///
    /// ```rust
    /// use xoz::Xoz;
    /// let mut xoz = Xoz::new();
    /// let root = xoz.parse_str("<!--foo--><p>Example</p><?bar?>").unwrap();
    ///
    /// let comment = xoz.first_child(root).unwrap();
    /// let p = xoz.next_sibling(comment).unwrap();
    /// let pi = xoz.next_sibling(p).unwrap();
    /// let text = xoz.first_child(p).unwrap();
    ///
    /// assert!(xoz.is_directly_under_document(comment));
    /// assert!(xoz.is_directly_under_document(pi));
    /// assert!(xoz.is_directly_under_document(p));
    /// assert!(!xoz.is_directly_under_document(text));
    /// ```
    pub fn is_directly_under_document(&self, node: Node) -> bool {
        let document = self.document(node.document_id);
        document.is_directly_under_document(node.document_node)
    }

    /// Returns true if the node is the document element
    ///
    /// ```rust
    /// use xoz::Xoz;
    /// let mut xoz = Xoz::new();
    /// let root = xoz.parse_str("<!--foo--><p>Example<em>Em</em></p>").unwrap();
    /// let comment = xoz.first_child(root).unwrap();
    /// let p = xoz.next_sibling(comment).unwrap();
    /// let text = xoz.first_child(p).unwrap();
    /// let em = xoz.next_sibling(text).unwrap();
    /// assert!(!xoz.is_document_element(comment));
    /// assert!(xoz.is_document_element(p));
    /// assert!(!xoz.is_document_element(text));
    /// assert!(!xoz.is_document_element(em));
    /// ```
    pub fn is_document_element(&self, node: Node) -> bool {
        let document = self.document(node.document_id);
        document.is_document_element(node.document_node)
    }

    /// Get index of child.
    ///
    /// Returns [`None`] if the node is not a child of this node.
    ///
    /// Namespace and attribute nodes aren't considered children.
    ///
    /// ```rust
    /// use xoz::Xoz;
    /// let mut xoz = Xoz::new();
    /// let root = xoz.parse_str("<p><a/><b/></p>").unwrap();
    /// let p = xoz.document_element(root);
    /// let a = xoz.first_child(p).unwrap();
    /// let b = xoz.next_sibling(a).unwrap();
    /// assert_eq!(xoz.child_index(p, a), Some(0));
    /// assert_eq!(xoz.child_index(p, b), Some(1));
    /// assert_eq!(xoz.child_index(a, b), None);
    /// ```
    pub fn child_index(&self, parent: Node, node: Node) -> Option<usize> {
        let parent_document_id = parent.document_id;
        let node_document_id = node.document_id;
        if parent_document_id != node_document_id {
            return None;
        }
        let document = self.document(node_document_id);
        document.child_index(parent.document_node, node.document_node)
    }

    /// Descendant of node type
    ///
    /// Look for the first descendant of node in document order that has NodeType.
    pub fn typed_descendant(&self, node: Node, node_type: NodeType) -> Option<Node> {
        let document = self.document(node.document_id);
        document
            .typed_descendant(node.document_node, node_type)
            .map(|n| document.new_node(n))
    }

    /// Following node of node type.
    ///
    /// Look for the first following node (after node) in document order that
    /// has node type.
    pub fn typed_foll(&self, node: Node, node_type: NodeType) -> Option<Node> {
        let document = self.document(node.document_id);
        document
            .typed_foll(node.document_node, node_type)
            .map(|n| document.new_node(n))
    }

    // info

    /// Preorder number of node
    ///
    /// This can be used to sort nodes by preorder.
    ///
    /// Note that since attributes and namespaces are also nodes in the tree,
    /// as well as the nodes that hold them, the preorder may have gaps.
    pub fn preorder(&self, node: Node) -> usize {
        let document = self.document(node.document_id);
        document.preorder(node.document_node)
    }

    pub fn sort_key(&self, node: Node) -> (usize, usize) {
        let document = self.document(node.document_id);
        (document.id.index(), document.preorder(node.document_node))
    }

    /// Given a node, give back the [`NodeName`] of this node.
    ///
    /// For elements and attribute that is their name, for processing
    /// instructions this is a name based on the target attribute.
    ///
    /// For anything else, it's `None`.
    ///
    /// ```rust
    /// use xoz::{Xoz, NodeName};
    /// let mut xoz = Xoz::new();
    /// let root = xoz.parse_str(r#"<ex:doc xmlns:ex="http://example.com" ex:b="B"><a/></ex:doc>"#).unwrap();
    /// let doc_el = xoz.document_element(root);
    /// let a_el = xoz.first_child(doc_el).unwrap();
    ///
    /// let doc_name = xoz.node_name(doc_el).unwrap();
    /// assert_eq!(doc_name.local_name(), b"doc");
    /// assert_eq!(doc_name.namespace(), b"http://example.com");
    ///
    /// let a_name = xoz.node_name(a_el).unwrap();
    /// assert_eq!(a_name.local_name(), b"a");
    /// assert_eq!(a_name.namespace(), b"");
    ///
    /// // it also works on attribute nodes
    /// let b_attribute = xoz.attribute_node(doc_el, NodeName::new("http://example.com", "b")).unwrap();
    /// let b_name = xoz.node_name(b_attribute).unwrap();
    /// assert_eq!(b_name.local_name(), b"b");
    /// assert_eq!(b_name.namespace(), b"http://example.com");
    /// ```
    pub fn node_name(&self, node: Node) -> Option<&NodeName> {
        let document = self.document(node.document_id);
        document.node_name(node.document_node)
    }

    /// Get the [`NodeType`] for a node.
    pub fn node_type(&self, node: Node) -> &NodeType {
        let document = self.document(node.document_id);
        document.node_type(node.document_node)
    }

    /// Check whether this node is a document node.
    pub fn is_document(&self, node: Node) -> bool {
        let document = self.document(node.document_id);
        document.is_document(node.document_node)
    }

    /// Check whether this node is an element node.
    pub fn is_element(&self, node: Node) -> bool {
        let document = self.document(node.document_id);
        document.is_element(node.document_node)
    }

    /// Check whether this node is a text node.
    pub fn is_text(&self, node: Node) -> bool {
        let document = self.document(node.document_id);
        document.is_text(node.document_node)
    }

    /// Check whether this node is a comment node.
    pub fn is_comment(&self, node: Node) -> bool {
        let document = self.document(node.document_id);
        document.is_comment(node.document_node)
    }

    /// Check whether this node is a processing instruction node.
    pub fn is_processing_instruction(&self, node: Node) -> bool {
        let document = self.document(node.document_id);
        document.is_processing_instruction(node.document_node)
    }

    /// Check whether this node is an attribute node.
    pub fn is_attribute(&self, node: Node) -> bool {
        let document = self.document(node.document_id);
        document.is_attribute(node.document_node)
    }

    /// Check whether this node is a namespace node.
    pub fn is_namespace(&self, node: Node) -> bool {
        let document = self.document(node.document_id);
        document.is_namespace(node.document_node)
    }

    /// Count how many nodes of a certain type are in the subtree of this node.
    pub fn subtree_count(&self, node: Node, node_type: NodeType) -> usize {
        let document = self.document(node.document_id);
        document.subtree_count(node.document_node, node_type)
    }

    /// Count how many nodes there are in a subtree of this node.
    pub fn subtree_size(&self, node: Node) -> usize {
        let document = self.document(node.document_id);
        document.subtree_size(node.document_node)
    }

    // attr
    
    /// Get the attribute node with the given name.
    ///
    /// If this is not an element node, or there is no attribute with the given name,
    /// it returns `None`.
    ///
    /// Note that [`Xoz::attribute_value`] can be used to access the attribute
    /// value directly.
    ///
    /// ```rust
    /// use xoz::Xoz;
    /// let mut xoz = Xoz::new();
    /// let root = xoz.parse_str(r#"<p a="1" b="2"/>"#).unwrap();
    /// let p = xoz.document_element(root);
    /// let a = xoz.attribute_node(p, "a").unwrap();
    /// let value = xoz.string_value(a);
    /// assert_eq!(value, "1");
    /// ```
    pub fn attribute_node<'a>(&self, node: Node, name: impl Into<NodeName<'a>>) -> Option<Node> {
        let document = self.document(node.document_id);
        document
            .attribute_node(node.document_node, name)
            .map(|n| document.new_node(n))
    }

    /// Get a node which contains the attributes children of this node.
    ///
    /// This node has tag type `TagType::Attributes`.
    ///
    /// If this is not an element node or there are no attributes, it returns `None`.
    pub fn attributes_child(&self, node: Node) -> Option<Node> {
        let document = self.document(node.document_id);
        document
            .attributes_child(node.document_node)
            .map(|n| document.new_node(n))
    }

    /// Get the value of the attribute with the given name.
    ///
    /// If this is not an element node, or there is no attribute with the given name,
    /// it returns `None`.
    ///
    /// ```rust
    /// use xoz::Xoz;
    /// let mut xoz = Xoz::new();
    /// let root = xoz.parse_str(r#"<p a="1" b="2"/>"#).unwrap();
    /// let p = xoz.document_element(root);
    /// let value = xoz.attribute_value(p, "a").unwrap();
    /// assert_eq!(value, "1");
    /// ```
    pub fn attribute_value<'a>(&self, node: Node, name: impl Into<NodeName<'a>>) -> Option<&str> {
        let document = self.document(node.document_id);
        document.attribute_value(node.document_node, name)
    }

    /// Get an iterator over the name and value of all attributes of this node.
    ///
    /// If this is not an element node, it returns an empty iterator.
    pub fn attribute_entries<'a>(
        &'a self,
        node: Node,
    ) -> impl Iterator<Item = (&'a NodeName<'a>, &'a str)> + 'a {
        let document = self.document(node.document_id);
        document.attribute_entries(node.document_node)
    }

    // str
    
    /// Text node string.
    ///
    /// If the node is not a text node, this returns `None`.
    pub fn text_str(&self, node: Node) -> Option<&str> {
        let document = self.document(node.document_id);
        document.text_str(node.document_node)
    }

    /// Comment node string.
    ///
    /// If the node is not a comment node, this returns `None`.
    pub fn comment_str(&self, node: Node) -> Option<&str> {
        let document = self.document(node.document_id);
        document.comment_str(node.document_node)
    }

    /// Processing instruction node string.
    ///
    /// This includes both target and content information.
    ///
    /// If the node is not a processing instruction node, this returns `None`.
    pub fn processing_instruction_str(&self, node: Node) -> Option<&str> {
        let document = self.document(node.document_id);
        document.processing_instruction_str(node.document_node)
    }

    /// Get [`ProcessingInstruction`] if this is a processing instruction node.
    pub fn processing_instruction(&self, node: Node) -> Option<ProcessingInstruction> {
        let document = self.document(node.document_id);
        document.processing_instruction(node.document_node)
    }

    /// Given a node, give back a string representation.
    ///
    /// For the root node and element nodes this gives back all text node
    /// descendant content, concatenated.
    ///
    /// For text nodes, it gives back the text.
    ///
    /// For comments, it gives back the comment text.
    ///
    /// For processing instructions, it gives back their content (data).
    ///
    /// For attribute nodes, it gives back the attribute value.
    ///
    /// For namespace nodes, it gives back the namespace URI.
    ///
    /// This is defined by the `string-value` property in
    /// <https://www.w3.org/TR/xpath-datamodel-31>
    pub fn string_value(&self, node: Node) -> String {
        let document = self.document(node.document_id);
        document.string_value(node.document_node)
    }

    /// Get the string content of a node.
    pub fn node_str(&self, node: Node) -> Option<&str> {
        let document = self.document(node.document_id);
        document.node_str(node.document_node)
    }

    // ns
    
    /// Get a node which contains the namespace declarations ("xmlns") children of
    /// of this node.
    ///
    /// This node has tag type `TagType::Namespaces`.
    ///
    /// If this is not an element node, or there are no namespace declarations,
    /// it returns `None`.
    pub fn namespaces_child(&self, node: Node) -> Option<Node> {
        let document = self.document(node.document_id);
        document
            .namespaces_child(node.document_node)
            .map(|n| document.new_node(n))
    }

    /// Get an iterator over the namespace declarations of this node.
    ///
    /// This iterates over prefix, uri tuples.
    pub fn namespace_entries(&self, node: Node) -> impl Iterator<Item = (&[u8], &[u8])> + '_ {
        let document = self.document(node.document_id);
        document.namespace_entries(node.document_node)
    }

    /// Given a namespace URI, return the prefix for this node
    ///
    /// This walks up the tree to find the first namespace declaration
    /// that has the given URI. If an element declares multiple prefixes for the
    /// same URI then an empty prefix is preferred over non-empty prefix.
    ///
    /// The `xml` prefix always exists. The prefix for the empty namespace is
    /// always empty.
    pub fn prefix_for_namespace(&self, node: Node, uri: &[u8]) -> Option<&[u8]> {
        let document = self.document(node.document_id);
        document.prefix_for_namespace(node.document_node, uri)
    }

    /// Prefix for a node
    ///
    /// Only element and attributes can have prefixes.
    pub fn node_prefix(&self, node: Node) -> Option<&[u8]> {
        let document = self.document(node.document_id);
        document.node_prefix(node.document_node)
    }

    /// Full name for a node.
    ///
    /// This is the prefix and local name concatenated with a colon, if a prefix
    /// exists.
    ///
    /// If the node is not an element or attribute node, this returns `None`.
    pub fn node_full_name(&self, node: Node) -> Option<String> {
        let document = self.document(node.document_id);
        document.node_full_name(node.document_node)
    }

    // iter
    
    /// Iterator over the child nodes of this node.
    ///
    /// Note that the special Namespaces and Attributes nodes are not
    /// included in the iterator, but the children of these nodes can be accessed
    /// using this way.
    ///
    /// ```rust
    /// use xoz::Xoz;
    /// let mut xoz = Xoz::new();
    /// let root = xoz.parse_str("<p><a/><b/></p>").unwrap();
    /// let p = xoz.document_element(root);
    /// let a = xoz.first_child(p).unwrap();
    /// let b = xoz.next_sibling(a).unwrap();
    /// let children = xoz.children(p).collect::<Vec<_>>();
    ///
    /// assert_eq!(children, vec![a, b]);
    /// ```
    pub fn children(&self, node: Node) -> impl DoubleEndedIterator<Item = Node> + '_ {
        let document = self.document(node.document_id);
        document
            .children(node.document_node)
            .map(move |n| document.new_node(n))
    }

    /// Iterator representing the XPath child axis.
    ///
    /// This is the same as [`Xoz::children`].
    pub fn axis_child(&self, node: Node) -> impl DoubleEndedIterator<Item = Node> + '_ {
        self.children(node)
    }

    /// Iterator over the following siblings of this node, not including this one.
    ///
    /// In case of namespace or attribute nodes, includes the following sibling
    /// namespace or attribute nodes.
    ///
    /// ```rust
    /// use xoz::Xoz;
    /// let mut xoz = Xoz::new();
    /// let root = xoz.parse_str("<p><a/><b/><c/></p>").unwrap();
    /// let p = xoz.document_element(root);
    /// let a = xoz.first_child(p).unwrap();
    /// let b = xoz.next_sibling(a).unwrap();
    /// let c = xoz.next_sibling(b).unwrap();
    /// let siblings = xoz.following_siblings(a).collect::<Vec<_>>();
    /// assert_eq!(siblings, vec![b, c]);
    /// let siblings = xoz.following_siblings(b).collect::<Vec<_>>();
    /// assert_eq!(siblings, vec![c]);
    /// ```
    pub fn following_siblings(&self, node: Node) -> impl Iterator<Item = Node> + '_ {
        let document = self.document(node.document_id);
        document
            .following_siblings(node.document_node)
            .map(move |n| document.new_node(n))
    }

    /// Iterator representing the XPath following-sibling axis.
    ///
    /// This is the same as [`Xoz::following_siblings`].
    pub fn axis_following_sibling(&self, node: Node) -> impl Iterator<Item = Node> + '_ {
        self.following_siblings(node)
    }

    /// Iterator over the preceding siblings of this node.
    pub fn preceding_siblings(&self, node: Node) -> impl Iterator<Item = Node> + '_ {
        let document = self.document(node.document_id);
        document
            .preceding_siblings(node.document_node)
            .map(move |n| document.new_node(n))
    }

    /// Iterator representing the XPath preceding-sibling axis.
    ///
    /// This is the same as [`Xoz::preceding_siblings`] but in reverse order.
    pub fn axis_preceding_sibling(&self, node: Node) -> impl Iterator<Item = Node> + '_ {
        let document = self.document(node.document_id);
        document
            .axis_preceding_sibling(node.document_node)
            .map(move |n| document.new_node(n))
    }

    /// Iterator over ancestor nodes, including this one.
    ///
    /// Namespace and attribute node have ancestors, even though
    /// they aren't the child of the element they are in.
    ///
    /// ```rust
    /// use xoz::Xoz;
    /// let mut xoz = Xoz::new();
    /// let root = xoz.parse_str("<a><b><c/></b></a>").unwrap();
    ///
    /// let a = xoz.document_element(root);
    /// let b = xoz.first_child(a).unwrap();
    /// let c = xoz.first_child(b).unwrap();
    ///
    /// let ancestors = xoz.ancestors_or_self(c).collect::<Vec<_>>();
    /// assert_eq!(ancestors, vec![c, b, a, root]);
    /// ```
    pub fn ancestors_or_self(&self, node: Node) -> impl Iterator<Item = Node> + '_ {
        let document = self.document(node.document_id);
        document
            .ancestors_or_self(node.document_node)
            .map(move |n| document.new_node(n))
    }

    /// Iterator representing the XPath ancestor-or-self axis.
    ///
    /// Note that this starts at the root node, and then descends to the
    /// provided node, unlike [`Xoz::ancestors_or_self`].
    pub fn axis_ancestor_or_self(&self, node: Node) -> impl Iterator<Item = Node> + '_ {
        let document = self.document(node.document_id);
        document
            .axis_ancestor_or_self(node.document_node)
            .map(move |n| document.new_node(n))
    }

    /// Iterator over ancestor nodes, not including this one.
    ///
    /// ```rust
    /// use xoz::Xoz;
    /// let mut xoz = Xoz::new();
    /// let root = xoz.parse_str("<a><b><c/></b></a>").unwrap();
    /// let a = xoz.document_element(root);
    /// let b = xoz.first_child(a).unwrap();
    /// let c = xoz.first_child(b).unwrap();
    /// let ancestors = xoz.ancestors(c).collect::<Vec<_>>();
    /// assert_eq!(ancestors, vec![b, a, root]);
    /// ```
    pub fn ancestors(&self, node: Node) -> impl Iterator<Item = Node> + '_ {
        let document = self.document(node.document_id);
        document
            .ancestors(node.document_node)
            .map(move |n| document.new_node(n))
    }

    /// Iterator representing the XPath ancestor axis.
    ///
    /// Note that this starts at the root node, and then descends to the
    /// provided node, unlike [`Xoz::ancestors`].
    pub fn axis_ancestor(&self, node: Node) -> impl Iterator<Item = Node> + '_ {
        let document = self.document(node.document_id);
        document
            .axis_ancestor(node.document_node)
            .map(move |n| document.new_node(n))
    }

    /// Iterator over of the descendants of this node,
    /// not including this one. In document order (pre-order depth-first).
    ///
    /// Namespace and attribute nodes aren't included as descendants.
    ///
    /// ```rust
    /// use xoz::Xoz;
    /// let mut xoz = Xoz::new();
    /// let root = xoz.parse_str("<a><b><c/></b></a>").unwrap();
    /// let a = xoz.document_element(root);
    /// let b = xoz.first_child(a).unwrap();
    /// let c = xoz.first_child(b).unwrap();
    /// let descendants = xoz.descendants(a).collect::<Vec<_>>();
    /// assert_eq!(descendants, vec![b, c]);
    /// ```
    pub fn descendants(&self, node: Node) -> impl Iterator<Item = Node> + '_ {
        let document = self.document(node.document_id);
        document
            .descendants(node.document_node)
            .map(move |n| document.new_node(n))
    }

    /// Iterator representing the XPath descendant axis.
    ///
    /// This is the same as [`Xoz::descendants`].
    pub fn axis_descendant(&self, node: Node) -> impl Iterator<Item = Node> + '_ {
        self.descendants(node)
    }

    /// Iterator over of the descendants of this node, including this one.
    /// In document order (pre-order depth-first).
    ///
    /// Namespace and attribute nodes aren't included as descendants.
    ///
    /// ```rust
    /// use xoz::Xoz;
    /// let mut xoz = Xoz::new();
    /// let root = xoz.parse_str("<a><b><c/></b></a>").unwrap();
    /// let a = xoz.document_element(root);
    /// let b = xoz.first_child(a).unwrap();
    /// let c = xoz.first_child(b).unwrap();
    /// let descendants = xoz.descendants_or_self(a).collect::<Vec<_>>();
    /// assert_eq!(descendants, vec![a, b, c]);
    /// ```
    pub fn descendants_or_self(&self, node: Node) -> impl Iterator<Item = Node> + '_ {
        let document = self.document(node.document_id);
        document
            .descendants_or_self(node.document_node)
            .map(move |n| document.new_node(n))
    }

    /// Iterator representing the XPath descendant-or-self axis.
    ///
    /// This is the same as [`Xoz::descendants_or_self`].
    pub fn axis_descendant_or_self(&self, node: Node) -> impl Iterator<Item = Node> + '_ {
        self.descendants_or_self(node)
    }

    /// Iterator over the attribute nodes of this node.
    ///
    /// ```rust
    /// use xoz::Xoz;
    /// let mut xoz = Xoz::new();
    /// let root = xoz.parse_str(r#"<p a="1" b="2"/>"#).unwrap();
    /// let p = xoz.document_element(root);
    /// let attributes = xoz.attributes(p).collect::<Vec<_>>();
    /// assert_eq!(attributes.len(), 2);
    /// ```
    pub fn attributes(&self, node: Node) -> impl Iterator<Item = Node> + '_ {
        let document = self.document(node.document_id);
        document
            .attributes(node.document_node)
            .map(move |n| document.new_node(n))
    }

    /// Iterator representing the XPath attribute axis.
    ///
    /// This is the same as [`Xoz::attributes`].
    pub fn axis_attributes(&self, node: Node) -> impl Iterator<Item = Node> + '_ {
        self.attributes(node)
    }

    /// Iterator representing the XPath parent axis
    pub fn axis_parent(&self, node: Node) -> impl Iterator<Item = Node> + '_ {
        self.parent(node).into_iter()
    }

    /// Iterator representing the XPath self axis
    pub fn axis_self(&self, node: Node) -> impl Iterator<Item = Node> + '_ {
        std::iter::once(node)
    }

    /// Following nodes in document order
    ///
    /// These are nodes that come after given node in document order,
    /// without that node itself, its ancestors, or its descendants.
    ///
    /// Does not include namespace or attribute nodes.
    ///
    /// ```rust
    /// use xoz::Xoz;
    /// let mut xoz = Xoz::new();
    /// let root = xoz.parse_str("<p><a/><b><c/><d/><e/></b><f><g/><h/></f></p>").unwrap();
    /// let p = xoz.document_element(root);
    /// let a = xoz.first_child(p).unwrap();
    /// let b = xoz.next_sibling(a).unwrap();
    /// let c = xoz.first_child(b).unwrap();
    /// let d = xoz.next_sibling(c).unwrap();
    /// let e = xoz.next_sibling(d).unwrap();
    /// let f = xoz.next_sibling(b).unwrap();
    /// let g = xoz.first_child(f).unwrap();
    /// let h = xoz.next_sibling(g).unwrap();
    /// let siblings = xoz.following(c).collect::<Vec<_>>();
    /// assert_eq!(siblings, vec![d, e, f, g, h]);
    /// ```
    pub fn following(&self, node: Node) -> impl Iterator<Item = Node> + '_ {
        let document = self.document(node.document_id);
        document
            .following(node.document_node)
            .map(move |n| document.new_node(n))
    }

    /// Iterator representing the XPath following axis.
    ///
    /// This is the same as [`Xoz::following`].
    pub fn axis_following(&self, node: Node) -> impl Iterator<Item = Node> + '_ {
        self.following(node)
    }

    /// Iterator representing the XPath preceding axis.
    ///
    /// These are nodes that come before given node in document order.
    pub fn axis_preceding(&self, node: Node) -> impl Iterator<Item = Node> + '_ {
        let document = self.document(node.document_id);
        document
            .axis_preceding(node.document_node)
            .map(move |n| document.new_node(n))
    }

    /// Iterate over descendants of a certain node type.
    ///
    /// This more efficient than filtering the descendants iterator, as it
    /// only traverses the nodes that are of the given type, jumping over
    /// irrelevant ones.
    pub fn typed_descendants(
        &self,
        node: Node,
        node_type: NodeType,
    ) -> impl Iterator<Item = Node> + '_ {
        let document = self.document(node.document_id);
        document
            .typed_descendants(node.document_node, node_type)
            .map(move |n| document.new_node(n))
    }

    /// Iterate over descendants of a certain node type, including self if it matches.
    pub fn typed_descendants_or_self(
        &self,
        node: Node,
        node_type: NodeType,
    ) -> impl Iterator<Item = Node> + '_ {
        let document = self.document(node.document_id);
        document
            .typed_descendants_or_self(node.document_node, node_type)
            .map(move |n| document.new_node(n))
    }

    /// Iterate over following nodes of a certain node type.
    pub fn typed_following(
        &self,
        node: Node,
        node_type: NodeType,
    ) -> impl Iterator<Item = Node> + '_ {
        let document = self.document(node.document_id);
        document
            .typed_following(node.document_node, node_type)
            .map(move |n| document.new_node(n))
    }

    /// Iterate over the nodes in the tree.
    ///
    /// This goes in document order. Attributes and namespace nodes are not included.
    ///
    /// The iterator yields a tuple of the node type, the tag state (open, close, empty),
    /// and the node itself. Only document and element node have an open and close state.
    pub fn traverse(
        &self,
        node: Node,
    ) -> impl Iterator<Item = (&NodeType, crate::TagState, Node)> + '_ {
        let document = self.document(node.document_id);
        document
            .traverse(node.document_node)
            .map(move |(node_type, tag_state, n)| (node_type, tag_state, document.new_node(n)))
    }
}
