pub mod zip;

pub fn classify_serde_error(e: &serde_json::Error) -> &'static str {
    match e.classify() {
        serde_json::error::Category::Io => "Io error",
        serde_json::error::Category::Syntax => "Syntax error",
        serde_json::error::Category::Data => "Data error",
        serde_json::error::Category::Eof => "End of file error",
    }
}
