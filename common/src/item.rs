use serde::{Deserialize, Serialize};
use std::cmp::{Ord, Ordering};
use std::{error::Error, io};

#[derive(Debug, Clone, Eq, PartialEq, Serialize, Deserialize)]
pub struct ItemData {
    pub key: String,
}

#[derive(Debug, Clone, Eq, PartialEq, Serialize, Deserialize)]
pub struct Item {
    pub id: u16,
    pub data: ItemData,

    #[serde(default)]
    pub selected: bool,
    #[serde(skip)]
    pub score: Option<u32>,
    #[serde(skip)]
    pub match_indices: Option<Vec<usize>>,
}

impl Item {
    pub fn new(id: u16, data: ItemData) -> Self {
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
            // Fallback to the current order of the items
            _ => Ordering::Equal,
        }
    }
}

impl PartialOrd for Item {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        Some(self.cmp(other))
    }
}

pub fn parse_items(source: impl io::Read) -> Result<Vec<Item>, Box<dyn Error>> {
    let mut rdr = csv::ReaderBuilder::new()
        .has_headers(false)
        .from_reader(source);
    let mut result = Vec::new();
    for (i, data) in rdr.deserialize::<ItemData>().enumerate() {
        result.push(Item::new(i as u16, data?));
    }
    Ok(result)
}
