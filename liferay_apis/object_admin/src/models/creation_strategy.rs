use std::fmt::Display;

#[derive(Debug, Serialize, Deserialize)]
#[serde(rename_all = "UPPERCASE")]
pub enum CreationStrategy {
    Upsert,
}

impl Display for CreationStrategy {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            CreationStrategy::Upsert => write!(f, "UPSERT"),
        }
    }
}
