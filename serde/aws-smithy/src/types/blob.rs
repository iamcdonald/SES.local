use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Debug, Clone)]
// use aws_smithy_types::Blob;
// #[serde(remote = "Blob")]
#[serde(rename_all = "PascalCase")]
pub struct Blob {
    pub inner: Vec<u8>,
}
