use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Debug, Clone, PartialEq)]
// use aws_sdk_sesv2::types::Message;
// #[serde(remote = "Message")]
#[serde(rename_all = "PascalCase")]
pub struct Message {
    /// <p>The subject line of the email. The subject line can only contain 7-bit ASCII characters. However, you can specify non-ASCII characters in the subject line by using encoded-word syntax, as described in <a href="https://tools.ietf.org/html/rfc2047">RFC 2047</a>.</p>
    pub subject: ::std::option::Option<crate::types::Content>,
    /// <p>The body of the message. You can specify an HTML version of the message, a text-only version of the message, or both.</p>
    pub body: ::std::option::Option<crate::types::Body>,
    /// <p>The list of message headers that will be added to the email message.</p>
    pub headers: ::std::option::Option<::std::vec::Vec<crate::types::MessageHeader>>,
    /// <p>The List of attachments to include in your email. All recipients will receive the same attachments.</p>
    pub attachments: ::std::option::Option<::std::vec::Vec<crate::types::Attachment>>,
}
