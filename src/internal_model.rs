#[derive(Debug, Clone, PartialEq, Eq)]
pub(crate) enum KdlEntity {
    Whitespace(String),
    Comment(KdlComment),
    Node(KdlType, KdlNode),
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub(crate) enum KdlNodeEntity {
    Whitespace(String),
    Comment(KdlComment),
    Value(KdlType, KdlValue),
    Property(KdlType, KdlProperty),
    Node(KdlType, KdlNode),
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub(crate) enum KdlComment {
    SingleLine(String),
    MultiLine(String),
    SlashDash(String),
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub(crate) struct KdlNode {
    name: String,
    entities: Vec<KdlEntity>,
    children: Vec<KdlEntity>,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub(crate) struct KdlProperty {
    quoted: bool,
    name: String,
    value: KdlValue,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub(crate) enum KdlValue {
    RawString(String),
    String(String),
    Base2(String),
    Base8(String),
    Base10(String),
    Base16(String),
    Bool(bool),
    Null,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub(crate) struct KdlType(pub(crate) String);
