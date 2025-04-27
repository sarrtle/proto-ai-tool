use serde::Deserialize;

#[derive(Debug, Deserialize)]
#[serde(rename_all = "camelCase")]
#[allow(dead_code)]
pub struct ResponseModel {
    pub candidates: Vec<Candidate>,
    pub model_version: String,
    pub usage_metadata: UsageMetadata,
}

#[derive(Debug, Deserialize)]
#[allow(dead_code)]
pub struct Candidate {
    pub content: Content,
    pub index: u32,
}

#[derive(Debug, Deserialize)]
#[allow(dead_code)]
pub struct Content {
    pub parts: Vec<Part>,
    pub role: String,
}

#[derive(Debug, Deserialize)]
pub struct Part {
    pub text: String,
}

#[derive(Debug, Deserialize)]
#[serde(rename_all = "camelCase")]
#[allow(dead_code)]
pub struct UsageMetadata {
    pub candidates_token_count: Option<u32>,
    pub prompt_token_count: u32,
    pub prompt_tokens_details: Vec<PromptTokensDetails>,
    pub thoughts_token_count: u32,
    pub total_token_count: u32,
}

#[derive(Debug, Deserialize)]
#[serde(rename_all = "camelCase")]
#[allow(dead_code)]
pub struct PromptTokensDetails {
    pub modality: String,
    pub token_count: u32,
}
