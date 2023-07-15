use serde::{Deserialize, Serialize};
use std::cmp::{Ord, Ordering};

#[derive(Debug, Clone, Eq, PartialEq, Serialize, Deserialize)]
pub struct ItemData {
    pub key: String,
    pub value: Option<String>,
}

#[derive(Debug, Clone, Eq, PartialEq, Serialize, Deserialize)]
pub struct Item {
    pub id: usize,
    pub data: ItemData,
    #[serde(default)]
    pub selected: bool,
    #[serde(skip)]
    pub score: Option<u32>,
    #[serde(skip)]
    pub match_indices: Option<Vec<usize>>,
}

impl Item {
    pub fn new(id: usize, data: ItemData) -> Self {
        Self {
            id,
            data,
            score: None,
            match_indices: None,
            selected: false,
        }
    }
}

impl Ord for Item {
    fn cmp(&self, other: &Self) -> Ordering {
        match (self.score, other.score) {
            // Sort by score
            (Some(a), Some(b)) => a.cmp(&b),
            // Items with a score should be above those without
            (Some(_), _) => Ordering::Greater,
            (None, Some(_)) => Ordering::Less,
            // Fallback to the original order of the items
            _ => other.id.cmp(&self.id),
        }
    }
}

impl PartialOrd for Item {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        Some(self.cmp(other))
    }
}
