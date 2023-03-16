use std::fmt::Display;

#[derive(Debug, Serialize, Deserialize)]
#[serde(rename_all = "UPPERCASE")]
pub enum CreateStrategy {
    Upsert,
}

impl Display for CreateStrategy {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            CreateStrategy::Upsert => write!(f, "UPSERT"),
        }
    }
}
