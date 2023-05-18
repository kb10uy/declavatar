use kdl::KdlDocument;

pub fn parse_node(source: &str) -> KdlDocument {
    source.parse().expect("invalid document")
}
