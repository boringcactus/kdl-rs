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
    pub(crate) name: String,
    pub(crate) entities: Vec<KdlNodeEntity>,
    pub(crate) children: Option<Vec<KdlEntity>>,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub(crate) struct KdlProperty {
    pub(crate) quoted: bool,
    pub(crate) name: String,
    pub(crate) value: KdlValue,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub(crate) enum KdlValue {
    RawString(KdlString),
    String(KdlString),
    Base2(String),
    Base8(String),
    Base10(String),
    Base16(String),
    Bool(bool),
    Null,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub(crate) struct KdlString {
    pub(crate) value: String,
    pub(crate) original: String,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub(crate) struct KdlType(pub(crate) String);
