use serde::{Deserialize, Deserializer};

#[derive(Debug, Deserialize)]
#[serde(rename_all = "camelCase")]
#[allow(dead_code)]
pub struct ResponseModel {
    pub candidates: Vec<Candidate>,
    pub model_version: String,
    pub usage_metadata: UsageMetadata,
}

#[derive(Debug)]
#[allow(dead_code)]
pub struct Candidate {
    pub content: Content,
    pub grounding_metadata: bool,
    pub index: u32,
}

// don't need the complex structure of grounding_metadata
impl<'de> Deserialize<'de> for Candidate {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: Deserializer<'de>,
    {
        #[derive(Deserialize)]
        #[serde(rename_all = "camelCase")]
        struct CandidateHelper {
            content: Content,
            #[serde(default)]
            grounding_metadata: Option<serde_json::Value>, // we use this as a probe
            index: u32,
        }

        let helper = CandidateHelper::deserialize(deserializer)?;

        let has_content = match helper.grounding_metadata {
            Some(serde_json::Value::Object(ref map)) if !map.is_empty() => true,
            _ => false,
        };
        Ok(Candidate {
            content: helper.content,
            grounding_metadata: has_content,
            index: helper.index,
        })
    }
}

#[derive(Debug, Deserialize)]
#[allow(dead_code)]
pub struct Content {
    pub parts: Vec<Part>,
    pub role: String,
}

#[derive(Debug, Deserialize)]
#[allow(dead_code)]
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
    pub thoughts_token_count: Option<u32>,
    pub total_token_count: u32,
}

#[derive(Debug, Deserialize)]
#[serde(rename_all = "camelCase")]
#[allow(dead_code)]
pub struct PromptTokensDetails {
    pub modality: String,
    pub token_count: u32,
}
