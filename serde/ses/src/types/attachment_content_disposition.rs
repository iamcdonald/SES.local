use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Debug, Clone, PartialEq)]
// use aws_sdk_sesv2::types::AttachmentContentDisposition;
// #[serde(remote = "AttachmentContentDisposition")]
#[serde(rename_all = "PascalCase")]
pub enum AttachmentContentDisposition {
    #[allow(missing_docs)] // documentation missing in model
    Attachment,
    #[allow(missing_docs)] // documentation missing in model
    Inline,
    // /// `Unknown` contains new variants that have been added since this code was generated.
    // #[deprecated(
    //     note = "Don't directly match on `Unknown`. See the docs on this enum for the correct way to handle unknown variants."
    // )]
    // Unknown(crate::primitives::sealed_enum_unknown::UnknownVariantValue),
}
