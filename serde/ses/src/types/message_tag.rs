use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Debug, Clone)]
// use aws_sdk_sesv2::types::MessageTag;
// #[serde(remote = "MessageTag")]
#[serde(rename_all = "PascalCase")]
pub struct MessageTag {
    /// <p>The name of the message tag. The message tag name has to meet the following criteria:</p>
    /// <ul>
    /// <li>
    /// <p>It can only contain ASCII letters (a–z, A–Z), numbers (0–9), underscores (_), or dashes (-).</p></li>
    /// <li>
    /// <p>It can contain no more than 256 characters.</p></li>
    /// </ul>
    pub name: ::std::string::String,
    /// <p>The value of the message tag. The message tag value has to meet the following criteria:</p>
    /// <ul>
    /// <li>
    /// <p>It can only contain ASCII letters (a–z, A–Z), numbers (0–9), underscores (_), or dashes (-).</p></li>
    /// <li>
    /// <p>It can contain no more than 256 characters.</p></li>
    /// </ul>
    pub value: ::std::string::String,
}
