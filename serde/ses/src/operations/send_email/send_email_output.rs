use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Debug, Clone, PartialEq)]
// use aws_sdk_sesv2::operation::send_email::SendEmailOutput;
// #[serde(remote = "SendEmailOutput")]
#[serde(rename_all = "PascalCase")]
pub struct SendEmailOutput {
    /// <p>A unique identifier for the message that is generated when the message is accepted.</p><note>
    /// <p>It's possible for Amazon SES to accept a message without sending it. For example, this can happen when the message that you're trying to send has an attachment that contains a virus, or when you send a templated email that contains invalid personalization content.</p>
    /// </note>
    pub message_id: ::std::option::Option<::std::string::String>,
    // _request_id: Option<String>,
}
