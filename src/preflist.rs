/// Preflists are lists of VNodes responsible for a Riak object
///
/// For more information: https://docs.basho.com/riak/kv/latest/learn/concepts/replication/

/// `PrefListItem` represents a Riak preflist for a key
#[derive(Clone, Debug)]
pub struct PreflistItem {
    pub partition: i64,
    pub node: String,
    pub is_primary: bool,
}

impl PreflistItem {
    pub fn new(partition: i64, node: &str, is_primary: bool) -> PreflistItem {
        PreflistItem {
            partition: partition,
            node: node.to_string(),
            is_primary: is_primary,
        }
    }
}
