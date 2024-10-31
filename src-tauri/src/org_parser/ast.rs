#[derive(Debug, Clone, PartialEq)]
pub struct OrgDocument {
    pub title: Option<String>,
    pub nodes: Vec<OrgNode>,
}

#[derive(Debug, Clone, PartialEq)]
pub enum OrgNode {
    Heading(Heading),
    Paragraph(String),
    List(List),
    // We'll add more variants as we go
}

#[derive(Debug, Clone, PartialEq)]
pub struct Heading {
    pub level: u32,
    pub title: String,
    pub tags: Vec<String>,
    pub content: Vec<OrgNode>,
}

#[derive(Debug, Clone, PartialEq)]
pub struct List {
    pub items: Vec<ListItem>,
    pub kind: ListKind,
}

#[derive(Debug, Clone, PartialEq)]
pub struct ListItem {
    pub content: String,
    pub sub_items: Vec<ListItem>,
}

#[derive(Debug, Clone, PartialEq)]
pub enum ListKind {
    Unordered,
    Ordered,
} 