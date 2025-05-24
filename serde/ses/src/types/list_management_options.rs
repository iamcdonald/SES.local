use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Debug, Clone)]
// use aws_sdk_sesv2::types::ListManagementOptions;
// #[serde(remote = "ListManagementOptions")]
#[serde(rename_all = "PascalCase")]
pub struct ListManagementOptions {
    /// <p>The name of the contact list.</p>
    pub contact_list_name: ::std::string::String,
    /// <p>The name of the topic.</p>
    pub topic_name: ::std::option::Option<::std::string::String>,
}
