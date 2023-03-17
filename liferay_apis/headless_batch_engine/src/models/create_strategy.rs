use std::fmt::Display;

#[derive(Debug, Serialize, Deserialize, Clone)]
#[serde(rename_all = "UPPERCASE")]
pub enum CreateStrategy {
    Create,
    Upsert,
}

impl Display for CreateStrategy {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            CreateStrategy::Upsert => write!(f, "UPSERT"),
            CreateStrategy::Create => write!(f, "CREATE"),
        }
    }
}
