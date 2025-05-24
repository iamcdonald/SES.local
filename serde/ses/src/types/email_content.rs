use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Debug, Clone)]
// #[serde(remote = "EmailContent")]
// use aws_sdk_sesv2::types::EmailContent;
#[serde(rename_all = "PascalCase")]
pub struct EmailContent {
    pub simple: ::std::option::Option<crate::types::Message>,
    /// <p>The raw email message. The message has to meet the following criteria:</p>
    /// <ul>
    /// <li>
    /// <p>The message has to contain a header and a body, separated by one blank line.</p></li>
    /// <li>
    /// <p>All of the required header fields must be present in the message.</p></li>
    /// <li>
    /// <p>Each part of a multipart MIME message must be formatted properly.</p></li>
    /// <li>
    /// <p>If you include attachments, they must be in a file format that the Amazon SES API v2 supports.</p></li>
    /// <li>
    /// <p>The raw data of the message needs to base64-encoded if you are accessing Amazon SES directly through the HTTPS interface. If you are accessing Amazon SES using an Amazon Web Services SDK, the SDK takes care of the base 64-encoding for you.</p></li>
    /// <li>
    /// <p>If any of the MIME parts in your message contain content that is outside of the 7-bit ASCII character range, you should encode that content to ensure that recipients' email clients render the message properly.</p></li>
    /// <li>
    /// <p>The length of any single line of text in the message can't exceed 1,000 characters. This restriction is defined in <a href="https://tools.ietf.org/html/rfc5321">RFC 5321</a>.</p></li>
    /// </ul>
    pub raw: ::std::option::Option<crate::types::RawMessage>,
    /// <p>The template to use for the email message.</p>
    pub template: ::std::option::Option<crate::types::Template>,
}
