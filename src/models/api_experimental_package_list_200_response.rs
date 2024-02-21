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
pub struct ApiExperimentalPackageList200Response {
    #[serde(rename = "next", default, with = "::serde_with::rust::double_option", skip_serializing_if = "Option::is_none")]
    pub next: Option<Option<String>>,
    #[serde(rename = "previous", default, with = "::serde_with::rust::double_option", skip_serializing_if = "Option::is_none")]
    pub previous: Option<Option<String>>,
    #[serde(rename = "results")]
    pub results: Vec<crate::models::PackageExperimental>,
}

impl ApiExperimentalPackageList200Response {
    pub fn new(results: Vec<crate::models::PackageExperimental>) -> ApiExperimentalPackageList200Response {
        ApiExperimentalPackageList200Response {
            next: None,
            previous: None,
            results,
        }
    }
}

