use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Debug, Clone)]
// use aws_sdk_sesv2::types::Attachment;
// #[serde(remote = "Attachment")]
#[serde(rename_all = "PascalCase")]
pub struct Attachment {
    /// <p>The raw data of the attachment. It needs to be base64-encoded if you are accessing Amazon SES directly through the HTTPS interface. If you are accessing Amazon SES using an Amazon Web Services SDK, the SDK takes care of the base 64-encoding for you.</p>
    pub raw_content: aws_smithy_serde::types::Blob,
    /// <p>A standard descriptor indicating how the attachment should be rendered in the email. Supported values: <code>ATTACHMENT</code> or <code>INLINE</code>.</p>
    pub content_disposition: ::std::option::Option<crate::types::AttachmentContentDisposition>,
    /// <p>The file name for the attachment as it will appear in the email. Amazon SES restricts certain file extensions. To ensure attachments are accepted, check the <a href="https://docs.aws.amazon.com/ses/latest/dg/mime-types.html">Unsupported attachment types</a> in the Amazon SES Developer Guide.</p>
    pub file_name: ::std::string::String,
    /// <p>A brief description of the attachment content.</p>
    pub content_description: ::std::option::Option<::std::string::String>,
    /// <p>Unique identifier for the attachment, used for referencing attachments with INLINE disposition in HTML content.</p>
    pub content_id: ::std::option::Option<::std::string::String>,
    /// <p>Specifies how the attachment is encoded. Supported values: <code>BASE64</code>, <code>QUOTED_PRINTABLE</code>, <code>SEVEN_BIT</code>.</p>
    pub content_transfer_encoding:
        ::std::option::Option<crate::types::AttachmentContentTransferEncoding>,
    /// <p>The MIME type of the attachment.</p><note>
    /// <p>Example: <code>application/pdf</code>, <code>image/jpeg</code></p>
    /// </note>
    pub content_type: ::std::option::Option<::std::string::String>,
}
