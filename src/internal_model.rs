use std::borrow::Cow;

pub(crate) enum KdlEntity {
    Whitespace(String),
    Comment(KdlComment),
    Node(KdlType, KdlNode),
}

pub(crate) enum KdlNodeEntity {
    Whitespace(String),
    Comment(KdlComment),
    Value(KdlType, KdlValue),
    Property(KdlType, KdlProperty),
    Node(KdlType, KdlNode),
}

pub(crate) enum KdlComment {
    SingleLine(String),
    MultiLine(String),
    SlashDash(String),
}

pub(crate) struct KdlNode {
    name: String,
    entities: Vec<KdlEntity>,
    children: Vec<KdlEntity>,
}

pub(crate) struct KdlProperty {
    quoted: bool,
    name: String,
    value: KdlValue,
}

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

pub struct KdlType(String);
