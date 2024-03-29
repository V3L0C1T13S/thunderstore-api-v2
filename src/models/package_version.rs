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
pub struct PackageVersion {
    #[serde(rename = "date_created")]
    pub date_created: String,
    #[serde(rename = "download_count")]
    pub download_count: i32,
    #[serde(rename = "download_url")]
    pub download_url: String,
    #[serde(rename = "install_url")]
    pub install_url: String,
    #[serde(rename = "version_number")]
    pub version_number: String,
}

impl PackageVersion {
    pub fn new(date_created: String, download_count: i32, download_url: String, install_url: String, version_number: String) -> PackageVersion {
        PackageVersion {
            date_created,
            download_count,
            download_url,
            install_url,
            version_number,
        }
    }
}


