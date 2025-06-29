use anyhow::anyhow;
use serde::{Deserialize, Serialize};
use serde_json::{Map, Value};
use std::cmp::{Ord, Ordering};

#[derive(Debug, Clone, Eq, PartialEq, Serialize, Deserialize)]
pub struct Item {
    pub id: usize,
    pub key: String,
    pub fields: Map<String, Value>,
    pub score: Option<u32>,
    #[serde(rename = "matchIndices")]
    pub match_indices: Option<Vec<usize>>,
}

impl Item {
    pub fn new(id: usize, key: String, fields: Map<String, Value>) -> Self {
        Self {
            id,
            key,
            fields,
            score: None,
            match_indices: None,
        }
    }

    pub fn try_from_json(id: usize, json: Value) -> anyhow::Result<Self> {
        if let Value::Object(props) = json {
            let key = props
                .get("key")
                .ok_or_else(|| anyhow!("No key in object props {props:?}"))
                .and_then(|key_json| {
                    if let Value::String(key) = key_json {
                        Ok(key)
                    } else {
                        Err(anyhow!("Expected key to be a string. Got: {key_json:?}"))
                    }
                })?
                .clone();
            Ok(Item::new(id, key, props))
        } else {
            Err(anyhow!("Expected object, got: {json:?}"))
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
