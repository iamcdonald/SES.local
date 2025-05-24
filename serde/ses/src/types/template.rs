use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Debug, Clone)]
// use aws_sdk_sesv2::types::Template;
// #[serde(remote = "Template")]
#[serde(rename_all = "PascalCase")]
pub struct Template {
    /// <p>The name of the template. You will refer to this name when you send email using the <code>SendTemplatedEmail</code> or <code>SendBulkTemplatedEmail</code> operations.</p>
    pub template_name: ::std::option::Option<::std::string::String>,
    /// <p>The Amazon Resource Name (ARN) of the template.</p>
    pub template_arn: ::std::option::Option<::std::string::String>,
    /// <p>The content of the template.</p><note>
    /// <p>Amazon SES supports only simple substitions when you send email using the <code>SendEmail</code> or <code>SendBulkEmail</code> operations and you provide the full template content in the request.</p>
    /// </note>
    pub template_content: ::std::option::Option<crate::types::EmailTemplateContent>,
    /// <p>An object that defines the values to use for message variables in the template. This object is a set of key-value pairs. Each key defines a message variable in the template. The corresponding value defines the value to use for that variable.</p>
    pub template_data: ::std::option::Option<::std::string::String>,
    /// <p>The list of message headers that will be added to the email message.</p>
    pub headers: ::std::option::Option<::std::vec::Vec<crate::types::MessageHeader>>,
    /// <p>The List of attachments to include in your email. All recipients will receive the same attachments.</p>
    pub attachments: ::std::option::Option<::std::vec::Vec<crate::types::Attachment>>,
}
