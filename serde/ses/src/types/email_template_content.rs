use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Debug, Clone)]
// use aws_sdk_sesv2::types::EmailTemplateContent;
// #[serde(remote = "EmailTemplateContent")]
#[serde(rename_all = "PascalCase")]
pub struct EmailTemplateContent {
    /// <p>The subject line of the email.</p>
    pub subject: ::std::option::Option<::std::string::String>,
    /// <p>The email body that will be visible to recipients whose email clients do not display HTML.</p>
    pub text: ::std::option::Option<::std::string::String>,
    /// <p>The HTML body of the email.</p>
    pub html: ::std::option::Option<::std::string::String>,
}
