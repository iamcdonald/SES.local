use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Debug, Clone, PartialEq)]
// use aws_sdk_sesv2::types::Content;
// #[serde(remote = "Content")]
#[serde(rename_all = "PascalCase")]
pub struct Content {
    /// <p>The content of the message itself.</p>
    pub data: ::std::string::String,
    /// <p>The character set for the content. Because of the constraints of the SMTP protocol, Amazon SES uses 7-bit ASCII by default. If the text includes characters outside of the ASCII range, you have to specify a character set. For example, you could specify <code>UTF-8</code>, <code>ISO-8859-1</code>, or <code>Shift_JIS</code>.</p>
    pub charset: ::std::option::Option<::std::string::String>,
}
