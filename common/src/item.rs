use std::cmp::{Ord, Ordering};

#[derive(Debug, Clone, Eq, PartialEq, serde::Deserialize, serde::Serialize)]
pub struct Item<'a> {
    pub id: u16,
    pub key: &'a str,
    pub score: Option<u32>,
    pub match_indices: Option<Vec<usize>>,
    pub selected: bool,
}

impl<'a> Item<'a> {
    pub fn new(id: u16, key: &'a str) -> Item<'a> {
        Self {
            id,
            key,
            score: None,
            match_indices: None,
            selected: false,
        }
    }
}

impl Ord for Item<'_> {
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

impl PartialOrd for Item<'_> {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        Some(self.cmp(other))
    }
}

// use std::{error::Error, io};
// pub fn parse_items(source: impl io::Read) -> Result<Vec<Item>, Box<dyn Error>> {
//     let mut rdr = csv::ReaderBuilder::new()
//         .has_headers(false)
//         .from_reader(source);
//     let mut result = Vec::new();
//     for (i, data) in rdr.deserialize::<ItemData>().enumerate() {
//         result.push(Item::new(i, data?));
//     }
//     Ok(result)
// }
