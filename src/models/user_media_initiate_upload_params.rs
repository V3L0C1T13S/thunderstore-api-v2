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
pub struct UserMediaInitiateUploadParams {
    #[serde(rename = "filename")]
    pub filename: String,
    #[serde(rename = "file_size_bytes")]
    pub file_size_bytes: i32,
}

impl UserMediaInitiateUploadParams {
    pub fn new(filename: String, file_size_bytes: i32) -> UserMediaInitiateUploadParams {
        UserMediaInitiateUploadParams {
            filename,
            file_size_bytes,
        }
    }
}


