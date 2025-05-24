use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Debug, Clone)]
// use aws_sdk_sesv2::types::MessageHeader;
// #[serde(remote = "MessageHeader")]
#[serde(rename_all = "PascalCase")]
pub struct MessageHeader {
    /// <p>The name of the message header. The message header name has to meet the following criteria:</p>
    /// <ul>
    /// <li>
    /// <p>Can contain any printable ASCII character (33 - 126) except for colon (:).</p></li>
    /// <li>
    /// <p>Can contain no more than 126 characters.</p></li>
    /// </ul>
    pub name: ::std::string::String,
    /// <p>The value of the message header. The message header value has to meet the following criteria:</p>
    /// <ul>
    /// <li>
    /// <p>Can contain any printable ASCII character.</p></li>
    /// <li>
    /// <p>Can contain no more than 870 characters.</p></li>
    /// </ul>
    pub value: ::std::string::String,
}
