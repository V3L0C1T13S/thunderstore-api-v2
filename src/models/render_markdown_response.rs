/*
 * Thunderstore API
 *
 * Schema is automatically generated and not completely accurate.
 *
 * The version of the OpenAPI document: v1
 * 
 * Generated by: https://openapi-generator.tech
 */




#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct RenderMarkdownResponse {
    #[serde(rename = "html")]
    pub html: String,
}

impl RenderMarkdownResponse {
    pub fn new(html: String) -> RenderMarkdownResponse {
        RenderMarkdownResponse {
            html,
        }
    }
}


