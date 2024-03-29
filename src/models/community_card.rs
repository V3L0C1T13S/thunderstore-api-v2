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
pub struct CommunityCard {
    #[serde(rename = "bg_image_src", deserialize_with = "Option::deserialize")]
    pub bg_image_src: Option<String>,
    #[serde(rename = "download_count")]
    pub download_count: i32,
    #[serde(rename = "identifier")]
    pub identifier: String,
    #[serde(rename = "name")]
    pub name: String,
    #[serde(rename = "package_count")]
    pub package_count: i32,
}

impl CommunityCard {
    pub fn new(bg_image_src: Option<String>, download_count: i32, identifier: String, name: String, package_count: i32) -> CommunityCard {
        CommunityCard {
            bg_image_src,
            download_count,
            identifier,
            name,
            package_count,
        }
    }
}


