use serde::{Serialize, Deserialize};
use std::collections::HashMap;
use crate::errors::SignaldError;

/// Accept a v2 group invitation. Note that you must have a profile name set to join groups.
#[derive(Serialize, Deserialize, Clone, Default)]
pub struct AcceptInvitationRequestV1 {
    /// The account to interact with
    /// Example: "+12024561414"
    #[serde(skip_serializing_if = "Option::is_none")]
    pub account: Option<String>,
    /// Example: "EdSqI90cS0UomDpgUXOlCoObWvQOXlH5G3Z2d3f4ayE="
    #[serde(skip_serializing_if = "Option::is_none")]
    #[serde(rename = "groupID")]
    pub group_id: Option<String>,
}

/// A local account in signald
#[derive(Serialize, Deserialize, Clone, Default)]
pub struct AccountV1 {
    /// The primary identifier on the account, included with all requests to signald for this account. Previously called 'username'
    #[serde(skip_serializing_if = "Option::is_none")]
    pub account_id: Option<String>,
    /// The address of this account
    #[serde(skip_serializing_if = "Option::is_none")]
    pub address: Option<JsonAddressV1>,
    /// The Signal device ID. Official Signal mobile clients (iPhone and Android) have device ID = 1, while linked devices such as Signal Desktop or Signal iPad have higher device IDs.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub device_id: Option<i32>,
    /// indicates the account has not completed registration
    #[serde(skip_serializing_if = "Option::is_none")]
    pub pending: Option<bool>,
}

#[derive(Serialize, Deserialize, Clone, Default)]
pub struct AccountAlreadyVerifiedErrorV1 {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub message: Option<String>,
}

#[derive(Serialize, Deserialize, Clone, Default)]
pub struct AccountHasNoKeysErrorV1 {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub message: Option<String>,
}

#[derive(Serialize, Deserialize, Clone, Default)]
pub struct AccountListV1 {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub accounts: Option<Vec<AccountV1>>,
}

#[derive(Serialize, Deserialize, Clone, Default)]
pub struct AccountLockedErrorV1 {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub message: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub more: Option<String>,
}

/// Link a new device to a local Signal account
#[derive(Serialize, Deserialize, Clone, Default)]
pub struct AddLinkedDeviceRequestV1 {
    /// The account to interact with
    /// Example: "+12024561414"
    #[serde(skip_serializing_if = "Option::is_none")]
    pub account: Option<String>,
    /// the tsdevice:/ uri provided (typically in qr code form) by the new device
    /// Example: "tsdevice:/?uuid=jAaZ5lxLfh7zVw5WELd6-Q&pub_key=BfFbjSwmAgpVJBXUdfmSgf61eX3a%2Bq9AoxAVpl1HUap9"
    #[serde(skip_serializing_if = "Option::is_none")]
    pub uri: Option<String>,
}

/// add a new server to connect to. Returns the new server's UUID.
#[derive(Serialize, Deserialize, Clone, Default)]
pub struct AddServerRequestV1 {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub server: Option<ServerV1>,
}

#[derive(Serialize, Deserialize, Clone, Default)]
pub struct AllIdentityKeyListV1 {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub identity_keys: Option<Vec<IdentityKeyListV1>>,
}

#[derive(Serialize, Deserialize, Clone, Default)]
pub struct AnswerMessageV1 {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub id: Option<i64>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub opaque: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub sdp: Option<String>,
}

/// approve a request to join a group
#[derive(Serialize, Deserialize, Clone, Default)]
pub struct ApproveMembershipRequestV1 {
    /// The account to interact with
    /// Example: "+12024561414"
    #[serde(skip_serializing_if = "Option::is_none")]
    pub account: Option<String>,
    /// Example: "EdSqI90cS0UomDpgUXOlCoObWvQOXlH5G3Z2d3f4ayE="
    #[serde(skip_serializing_if = "Option::is_none")]
    #[serde(rename = "groupID")]
    pub group_id: Option<String>,
    /// list of requesting members to approve
    #[serde(skip_serializing_if = "Option::is_none")]
    pub members: Option<Vec<JsonAddressV1>>,
}

#[derive(Serialize, Deserialize, Clone, Default)]
pub struct BusyMessageV1 {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub id: Option<i64>,
}

#[derive(Serialize, Deserialize, Clone, Default)]
pub struct CallMessageV1 {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub answer_message: Option<AnswerMessageV1>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub busy_message: Option<BusyMessageV1>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub destination_device_id: Option<i32>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub hangup_message: Option<HangupMessageV1>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub ice_update_message: Option<Vec<IceUpdateMessageV1>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub multi_ring: Option<bool>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub offer_message: Option<OfferMessageV1>,
}

#[derive(Serialize, Deserialize, Clone, Default)]
pub struct CapabilitiesV1 {
    #[serde(skip_serializing_if = "Option::is_none")]
    #[serde(rename = "gv1-migration")]
    pub gv_1_migration: Option<bool>,
    #[serde(skip_serializing_if = "Option::is_none")]
    #[serde(rename = "gv2")]
    pub gv_2: Option<bool>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub storage: Option<bool>,
}

#[derive(Serialize, Deserialize, Clone, Default)]
pub struct CaptchaRequiredErrorV1 {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub message: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub more: Option<String>,
}

/// Wraps all incoming messages after a v1 subscribe request is issued
#[derive(Serialize, Deserialize, Clone, Default)]
pub struct ClientMessageWrapperV1 {
    /// the incoming object. The structure will vary from message to message, see `type` and `version` fields
    #[serde(skip_serializing_if = "Option::is_none")]
    pub data: Option<HashMap<String, String>>,
    /// true if the incoming message represents an error
    #[serde(skip_serializing_if = "Option::is_none")]
    pub error: Option<bool>,
    /// the type of object to expect in the `data` field
    #[serde(skip_serializing_if = "Option::is_none")]
    #[serde(rename = "type")]
    pub type_: Option<String>,
    /// the version of the object in the `data` field
    #[serde(skip_serializing_if = "Option::is_none")]
    pub version: Option<String>,
}

#[derive(Serialize, Deserialize, Clone, Default)]
pub struct CreateGroupRequestV1 {
    /// The account to interact with
    /// Example: "+12024561414"
    #[serde(skip_serializing_if = "Option::is_none")]
    pub account: Option<String>,
    /// Example: "/tmp/image.jpg"
    #[serde(skip_serializing_if = "Option::is_none")]
    pub avatar: Option<String>,
    /// The role of all members other than the group creator. Options are ADMINISTRATOR or DEFAULT (case insensitive)
    /// Example: "ADMINISTRATOR"
    #[serde(skip_serializing_if = "Option::is_none")]
    pub member_role: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub members: Option<Vec<JsonAddressV1>>,
    /// the message expiration timer
    #[serde(skip_serializing_if = "Option::is_none")]
    pub timer: Option<i32>,
    /// Example: "Parkdale Run Club"
    #[serde(skip_serializing_if = "Option::is_none")]
    pub title: Option<String>,
}

/// delete all account data signald has on disk, and optionally delete the account from the server as well. Note that this is not "unlink" and will delete the entire account, even from a linked device.
#[derive(Serialize, Deserialize, Clone, Default)]
pub struct DeleteAccountRequestV1 {
    /// The account to delete
    /// Example: "+12024561414"
    #[serde(skip_serializing_if = "Option::is_none")]
    pub account: Option<String>,
    /// delete account information from the server as well (default false)
    #[serde(skip_serializing_if = "Option::is_none")]
    pub server: Option<bool>,
}

#[derive(Serialize, Deserialize, Clone, Default)]
pub struct DeviceInfoV1 {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub created: Option<i64>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub id: Option<i64>,
    #[serde(skip_serializing_if = "Option::is_none")]
    #[serde(rename = "lastSeen")]
    pub last_seen: Option<i64>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub name: Option<String>,
}

#[derive(Serialize, Deserialize, Clone, Default)]
pub struct FingerprintVersionMismatchErrorV1 {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub message: Option<String>,
}

/// After a linking URI has been requested, finish_link must be called with the session_id provided with the URI. it will return information about the new account once the linking process is completed by the other device.
#[derive(Serialize, Deserialize, Clone, Default)]
pub struct FinishLinkRequestV1 {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub device_name: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub session_id: Option<String>,
}

/// Generate a linking URI. Typically this is QR encoded and scanned by the primary device. Submit the returned session_id with a finish_link request.
#[derive(Serialize, Deserialize, Clone, Default)]
pub struct GenerateLinkingURIRequestV1 {
    /// The identifier of the server to use. Leave blank for default (usually Signal production servers but configurable at build time)
    #[serde(skip_serializing_if = "Option::is_none")]
    pub server: Option<String>,
}

/// get all known identity keys
#[derive(Serialize, Deserialize, Clone, Default)]
pub struct GetAllIdentitiesV1 {
    /// The account to interact with
    /// Example: "+12024561414"
    #[serde(skip_serializing_if = "Option::is_none")]
    pub account: Option<String>,
}

/// Query the server for the latest state of a known group. If no account in signald is a member of the group (anymore), an error with error_type: 'UnknownGroupError' is returned.
#[derive(Serialize, Deserialize, Clone, Default)]
pub struct GetGroupRequestV1 {
    /// The account to interact with
    /// Example: "+12024561414"
    #[serde(skip_serializing_if = "Option::is_none")]
    pub account: Option<String>,
    /// Example: "EdSqI90cS0UomDpgUXOlCoObWvQOXlH5G3Z2d3f4ayE="
    #[serde(skip_serializing_if = "Option::is_none")]
    #[serde(rename = "groupID")]
    pub group_id: Option<String>,
    /// the latest known revision, default value (-1) forces fetch from server
    #[serde(skip_serializing_if = "Option::is_none")]
    pub revision: Option<i32>,
}

/// Get information about a known keys for a particular address
#[derive(Serialize, Deserialize, Clone, Default)]
pub struct GetIdentitiesRequestV1 {
    /// The account to interact with
    /// Example: "+12024561414"
    #[serde(skip_serializing_if = "Option::is_none")]
    pub account: Option<String>,
    /// address to get keys for
    #[serde(skip_serializing_if = "Option::is_none")]
    pub address: Option<JsonAddressV1>,
}

/// list all linked devices on a Signal account
#[derive(Serialize, Deserialize, Clone, Default)]
pub struct GetLinkedDevicesRequestV1 {
    /// The account to interact with
    /// Example: "+12024561414"
    #[serde(skip_serializing_if = "Option::is_none")]
    pub account: Option<String>,
}

/// Get all information available about a user
#[derive(Serialize, Deserialize, Clone, Default)]
pub struct GetProfileRequestV1 {
    /// the signald account to use
    #[serde(skip_serializing_if = "Option::is_none")]
    pub account: Option<String>,
    /// the address to look up
    #[serde(skip_serializing_if = "Option::is_none")]
    pub address: Option<JsonAddressV1>,
    /// if true, return results from local store immediately, refreshing from server in the background if needed. if false (default), block until profile can be retrieved from server
    #[serde(skip_serializing_if = "Option::is_none")]
    #[serde(rename = "async")]
    pub async_: Option<bool>,
}

#[derive(Serialize, Deserialize, Clone, Default)]
pub struct GetServersRequestV1;

/// group access control settings. Options for each controlled action are: UNKNOWN, ANY, MEMBER, ADMINISTRATOR, UNSATISFIABLE and UNRECOGNIZED
#[derive(Serialize, Deserialize, Clone, Default)]
pub struct GroupAccessControlV1 {
    /// who can edit group info
    #[serde(skip_serializing_if = "Option::is_none")]
    pub attributes: Option<String>,
    /// UNSATISFIABLE when the group link is disabled, ADMINISTRATOR when the group link is enabled but an administrator must approve new members, ANY when the group link is enabled and no approval is required
    /// Example: "ANY"
    #[serde(skip_serializing_if = "Option::is_none")]
    pub link: Option<String>,
    /// who can add members
    #[serde(skip_serializing_if = "Option::is_none")]
    pub members: Option<String>,
}

/// A generic type that is used when the group version is not known
#[derive(Serialize, Deserialize, Clone, Default)]
pub struct GroupInfoV1 {
    #[serde(skip_serializing_if = "Option::is_none")]
    #[serde(rename = "v1")]
    pub v_1: Option<JsonGroupInfoV1>,
    #[serde(skip_serializing_if = "Option::is_none")]
    #[serde(rename = "v2")]
    pub v_2: Option<JsonGroupV2InfoV1>,
}

/// Get information about a group from a signal.group link
#[derive(Serialize, Deserialize, Clone, Default)]
pub struct GroupLinkInfoRequestV1 {
    /// The account to use
    /// Example: "+12024561414"
    #[serde(skip_serializing_if = "Option::is_none")]
    pub account: Option<String>,
    /// the signald.group link
    /// Example: "https://signal.group/#CjQKINH_GZhXhfifTcnBkaKTNRxW-hHKnGSq-cJNyPVqHRp8EhDUB7zjKNEl0NaULhsqJCX3"
    #[serde(skip_serializing_if = "Option::is_none")]
    pub uri: Option<String>,
}

#[derive(Serialize, Deserialize, Clone, Default)]
pub struct GroupLinkNotActiveErrorV1 {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub message: Option<String>,
}

#[derive(Serialize, Deserialize, Clone, Default)]
pub struct GroupListV1 {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub groups: Option<Vec<JsonGroupV2InfoV1>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    #[serde(rename = "legacyGroups")]
    pub legacy_groups: Option<Vec<JsonGroupInfoV1>>,
}

#[derive(Serialize, Deserialize, Clone, Default)]
pub struct GroupMemberV1 {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub joined_revision: Option<i32>,
    /// possible values are: UNKNOWN, DEFAULT, ADMINISTRATOR and UNRECOGNIZED
    /// Example: "DEFAULT"
    #[serde(skip_serializing_if = "Option::is_none")]
    pub role: Option<String>,
    /// Example: "aeed01f0-a234-478e-8cf7-261c283151e7"
    #[serde(skip_serializing_if = "Option::is_none")]
    pub uuid: Option<String>,
}

#[derive(Serialize, Deserialize, Clone, Default)]
pub struct GroupNotActiveErrorV1 {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub message: Option<String>,
}

#[derive(Serialize, Deserialize, Clone, Default)]
pub struct GroupVerificationErrorV1 {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub message: Option<String>,
}

#[derive(Serialize, Deserialize, Clone, Default)]
pub struct HangupMessageV1 {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub device_id: Option<i32>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub id: Option<i64>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub legacy: Option<bool>,
    #[serde(skip_serializing_if = "Option::is_none")]
    #[serde(rename = "type")]
    pub type_: Option<String>,
}

#[derive(Serialize, Deserialize, Clone, Default)]
pub struct IceUpdateMessageV1 {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub id: Option<i64>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub opaque: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub sdp: Option<String>,
}

#[derive(Serialize, Deserialize, Clone, Default)]
pub struct IdentityKeyV1 {
    /// the first time this identity key was seen
    #[serde(skip_serializing_if = "Option::is_none")]
    pub added: Option<i64>,
    /// base64-encoded QR code data
    #[serde(skip_serializing_if = "Option::is_none")]
    pub qr_code_data: Option<String>,
    /// Example: "373453558586758076680580548714989751943247272727416091564451"
    #[serde(skip_serializing_if = "Option::is_none")]
    pub safety_number: Option<String>,
    /// One of TRUSTED_UNVERIFIED, TRUSTED_VERIFIED or UNTRUSTED
    #[serde(skip_serializing_if = "Option::is_none")]
    pub trust_level: Option<String>,
}

/// a list of identity keys associated with a particular address
#[derive(Serialize, Deserialize, Clone, Default)]
pub struct IdentityKeyListV1 {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub address: Option<JsonAddressV1>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub identities: Option<Vec<IdentityKeyV1>>,
}

#[derive(Serialize, Deserialize, Clone, Default)]
pub struct IncomingMessageV1 {
    /// Example: "+12024561414"
    #[serde(skip_serializing_if = "Option::is_none")]
    pub account: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub call_message: Option<CallMessageV1>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub data_message: Option<JsonDataMessageV1>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub has_content: Option<bool>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub has_legacy_message: Option<bool>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub receipt_message: Option<ReceiptMessageV1>,
    /// Example: 1615576442475
    #[serde(skip_serializing_if = "Option::is_none")]
    pub server_deliver_timestamp: Option<i64>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub server_guid: Option<String>,
    /// Example: 1615576442475
    #[serde(skip_serializing_if = "Option::is_none")]
    pub server_receiver_timestamp: Option<i64>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub source: Option<JsonAddressV1>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub source_device: Option<i32>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub sync_message: Option<JsonSyncMessageV1>,
    /// Example: 1615576442475
    #[serde(skip_serializing_if = "Option::is_none")]
    pub timestamp: Option<i64>,
    #[serde(skip_serializing_if = "Option::is_none")]
    #[serde(rename = "type")]
    pub type_: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub typing_message: Option<TypingMessageV1>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub unidentified_sender: Option<bool>,
}

/// an internal error in signald has occured.
#[derive(Serialize, Deserialize, Clone, Default)]
pub struct InternalErrorV1 {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub exceptions: Option<Vec<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub message: Option<String>,
}

#[derive(Serialize, Deserialize, Clone, Default)]
pub struct InvalidAttachmentErrorV1 {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub filename: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub message: Option<String>,
}

#[derive(Serialize, Deserialize, Clone, Default)]
pub struct InvalidBase64ErrorV1 {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub message: Option<String>,
}

#[derive(Serialize, Deserialize, Clone, Default)]
pub struct InvalidFingerprintErrorV1 {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub message: Option<String>,
}

#[derive(Serialize, Deserialize, Clone, Default)]
pub struct InvalidGroupErrorV1 {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub message: Option<String>,
}

#[derive(Serialize, Deserialize, Clone, Default)]
pub struct InvalidGroupStateErrorV1 {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub message: Option<String>,
}

#[derive(Serialize, Deserialize, Clone, Default)]
pub struct InvalidInviteURIErrorV1 {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub message: Option<String>,
}

#[derive(Serialize, Deserialize, Clone, Default)]
pub struct InvalidProxyErrorV1 {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub message: Option<String>,
}

#[derive(Serialize, Deserialize, Clone, Default)]
pub struct InvalidRecipientErrorV1 {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub message: Option<String>,
}

#[derive(Serialize, Deserialize, Clone, Default)]
pub struct InvalidRequestErrorV1 {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub message: Option<String>,
}

/// Join a group using the a signal.group URL. Note that you must have a profile name set to join groups.
#[derive(Serialize, Deserialize, Clone, Default)]
pub struct JoinGroupRequestV1 {
    /// The account to interact with
    /// Example: "+12024561414"
    #[serde(skip_serializing_if = "Option::is_none")]
    pub account: Option<String>,
    /// The signal.group URL
    /// Example: "https://signal.group/#CjQKINH_GZhXhfifTcnBkaKTNRxW-hHKnGSq-cJNyPVqHRp8EhDUB7zjKNEl0NaULhsqJCX3"
    #[serde(skip_serializing_if = "Option::is_none")]
    pub uri: Option<String>,
}

#[derive(Serialize, Deserialize, Clone, Default)]
pub struct JsonAddressV1 {
    /// An e164 phone number, starting with +. Currently the only available user-facing Signal identifier.
    /// Example: "+13215551234"
    #[serde(skip_serializing_if = "Option::is_none")]
    pub number: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub relay: Option<String>,
    /// A UUID, the unique identifier for a particular Signal account.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub uuid: Option<String>,
}

/// represents a file attached to a message. When seding, only `filename` is required.
#[derive(Serialize, Deserialize, Clone, Default)]
pub struct JsonAttachmentV1 {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub blurhash: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub caption: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    #[serde(rename = "contentType")]
    pub content_type: Option<String>,
    /// the original name of the file
    #[serde(skip_serializing_if = "Option::is_none")]
    #[serde(rename = "customFilename")]
    pub custom_filename: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub digest: Option<String>,
    /// when sending, the path to the local file to upload
    #[serde(skip_serializing_if = "Option::is_none")]
    pub filename: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub height: Option<i32>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub id: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub key: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub size: Option<i32>,
    /// when receiving, the path that file has been downloaded to
    #[serde(skip_serializing_if = "Option::is_none")]
    #[serde(rename = "storedFilename")]
    pub stored_filename: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    #[serde(rename = "voiceNote")]
    pub voice_note: Option<bool>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub width: Option<i32>,
}

#[derive(Serialize, Deserialize, Clone, Default)]
pub struct JsonBlockedListMessageV1 {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub addresses: Option<Vec<JsonAddressV1>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    #[serde(rename = "groupIds")]
    pub group_ids: Option<Vec<String>>,
}

#[derive(Serialize, Deserialize, Clone, Default)]
pub struct JsonDataMessageV1 {
    /// files attached to the incoming message
    #[serde(skip_serializing_if = "Option::is_none")]
    pub attachments: Option<Vec<JsonAttachmentV1>>,
    /// the text body of the incoming message.
    /// Example: "hello"
    #[serde(skip_serializing_if = "Option::is_none")]
    pub body: Option<String>,
    /// if the incoming message has a shared contact, the contact's information will be here
    #[serde(skip_serializing_if = "Option::is_none")]
    pub contacts: Option<Vec<SharedContactV0>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    #[serde(rename = "endSession")]
    pub end_session: Option<bool>,
    /// the expiry timer on the incoming message. Clients should delete records of the message within this number of seconds
    #[serde(skip_serializing_if = "Option::is_none")]
    #[serde(rename = "expiresInSeconds")]
    pub expires_in_seconds: Option<i32>,
    /// if the incoming message was sent to a v1 group, information about that group will be here
    #[serde(skip_serializing_if = "Option::is_none")]
    pub group: Option<JsonGroupInfoV1>,
    /// if the incoming message was sent to a v2 group, basic identifying information about that group will be here. If group information changes, JsonGroupV2Info.revision is incremented. If the group revision is higher than previously seen, a client can retrieve the group information by calling get_group.
    #[serde(skip_serializing_if = "Option::is_none")]
    #[serde(rename = "groupV2")]
    pub group_v_2: Option<JsonGroupV2InfoV1>,
    /// the eraId string from a group call message update
    #[serde(skip_serializing_if = "Option::is_none")]
    pub group_call_update: Option<String>,
    /// list of mentions in the message
    #[serde(skip_serializing_if = "Option::is_none")]
    pub mentions: Option<Vec<JsonMentionV1>>,
    /// details about the MobileCoin payment attached to the message, if present
    #[serde(skip_serializing_if = "Option::is_none")]
    pub payment: Option<PaymentV1>,
    /// if the incoming message has a link preview, information about that preview will be here
    #[serde(skip_serializing_if = "Option::is_none")]
    pub previews: Option<Vec<JsonPreviewV1>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    #[serde(rename = "profileKeyUpdate")]
    pub profile_key_update: Option<bool>,
    /// if the incoming message is a quote or reply to another message, this will contain information about that message
    #[serde(skip_serializing_if = "Option::is_none")]
    pub quote: Option<JsonQuoteV1>,
    /// if the message adds or removes a reaction to another message, this will indicate what change is being made
    #[serde(skip_serializing_if = "Option::is_none")]
    pub reaction: Option<JsonReactionV1>,
    /// if the inbound message is deleting a previously sent message, indicates which message should be deleted
    #[serde(skip_serializing_if = "Option::is_none")]
    #[serde(rename = "remoteDelete")]
    pub remote_delete: Option<RemoteDeleteV1>,
    /// if the incoming message is a sticker, information about the sicker will be here
    #[serde(skip_serializing_if = "Option::is_none")]
    pub sticker: Option<JsonStickerV0>,
    /// the timestamp that the message was sent at, according to the sender's device. This is used to uniquely identify this message for things like reactions and quotes.
    /// Example: 1615576442475
    #[serde(skip_serializing_if = "Option::is_none")]
    pub timestamp: Option<i64>,
    /// indicates the message is a view once message. View once messages typically include no body and a single image attachment. Official Signal clients will prevent the user from saving the image, and once the user has viewed the image once they will destroy the image.
    #[serde(skip_serializing_if = "Option::is_none")]
    #[serde(rename = "viewOnce")]
    pub view_once: Option<bool>,
}

/// information about a legacy group
#[derive(Serialize, Deserialize, Clone, Default)]
pub struct JsonGroupInfoV1 {
    #[serde(skip_serializing_if = "Option::is_none")]
    #[serde(rename = "avatarId")]
    pub avatar_id: Option<i64>,
    #[serde(skip_serializing_if = "Option::is_none")]
    #[serde(rename = "groupId")]
    pub group_id: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub members: Option<Vec<JsonAddressV1>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub name: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    #[serde(rename = "type")]
    pub type_: Option<String>,
}

#[derive(Serialize, Deserialize, Clone, Default)]
pub struct JsonGroupJoinInfoV1 {
    /// The access level required in order to join the group from the invite link, as an AccessControl.AccessRequired enum from the upstream Signal groups.proto file. This is UNSATISFIABLE (4) when the group link is disabled; ADMINISTRATOR (3) when the group link is enabled, but an administrator must approve new members; and ANY (1) when the group link is enabled and no approval is required. See theGroupAccessControl structure and the upstream enum ordinals.
    #[serde(skip_serializing_if = "Option::is_none")]
    #[serde(rename = "addFromInviteLink")]
    pub add_from_invite_link: Option<i32>,
    /// Example: "A club for running in Parkdale"
    #[serde(skip_serializing_if = "Option::is_none")]
    pub description: Option<String>,
    /// Example: "EdSqI90cS0UomDpgUXOlCoObWvQOXlH5G3Z2d3f4ayE="
    #[serde(skip_serializing_if = "Option::is_none")]
    #[serde(rename = "groupID")]
    pub group_id: Option<String>,
    /// Example: 3
    #[serde(skip_serializing_if = "Option::is_none")]
    #[serde(rename = "memberCount")]
    pub member_count: Option<i32>,
    /// Whether the account is waiting for admin approval in order to be added to the group.
    #[serde(skip_serializing_if = "Option::is_none")]
    #[serde(rename = "pendingAdminApproval")]
    pub pending_admin_approval: Option<bool>,
    /// The Group V2 revision. This is incremented by clients whenever they update group information, and it is often used by clients to determine if the local group state is out-of-date with the server's revision.
    /// Example: 5
    #[serde(skip_serializing_if = "Option::is_none")]
    pub revision: Option<i32>,
    /// Example: "Parkdale Run Club"
    #[serde(skip_serializing_if = "Option::is_none")]
    pub title: Option<String>,
}

/// Information about a Signal group
#[derive(Serialize, Deserialize, Clone, Default)]
pub struct JsonGroupV2InfoV1 {
    /// current access control settings for this group
    #[serde(skip_serializing_if = "Option::is_none")]
    #[serde(rename = "accessControl")]
    pub access_control: Option<GroupAccessControlV1>,
    /// indicates if the group is an announcements group. Only admins are allowed to send messages to announcements groups. Options are UNKNOWN, ENABLED or DISABLED
    #[serde(skip_serializing_if = "Option::is_none")]
    pub announcements: Option<String>,
    /// path to the group's avatar on local disk, if available
    /// Example: "/var/lib/signald/avatars/group-EdSqI90cS0UomDpgUXOlCoObWvQOXlH5G3Z2d3f4ayE="
    #[serde(skip_serializing_if = "Option::is_none")]
    pub avatar: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub description: Option<String>,
    /// Example: "EdSqI90cS0UomDpgUXOlCoObWvQOXlH5G3Z2d3f4ayE="
    #[serde(skip_serializing_if = "Option::is_none")]
    pub id: Option<String>,
    /// the signal.group link, if applicable
    #[serde(skip_serializing_if = "Option::is_none")]
    #[serde(rename = "inviteLink")]
    pub invite_link: Option<String>,
    /// detailed member list
    #[serde(skip_serializing_if = "Option::is_none")]
    #[serde(rename = "memberDetail")]
    pub member_detail: Option<Vec<GroupMemberV1>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub members: Option<Vec<JsonAddressV1>>,
    /// detailed pending member list
    #[serde(skip_serializing_if = "Option::is_none")]
    #[serde(rename = "pendingMemberDetail")]
    pub pending_member_detail: Option<Vec<GroupMemberV1>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    #[serde(rename = "pendingMembers")]
    pub pending_members: Option<Vec<JsonAddressV1>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    #[serde(rename = "requestingMembers")]
    pub requesting_members: Option<Vec<JsonAddressV1>>,
    /// Example: 5
    #[serde(skip_serializing_if = "Option::is_none")]
    pub revision: Option<i32>,
    /// Example: 604800
    #[serde(skip_serializing_if = "Option::is_none")]
    pub timer: Option<i32>,
    /// Example: "Parkdale Run Club"
    #[serde(skip_serializing_if = "Option::is_none")]
    pub title: Option<String>,
}

#[derive(Serialize, Deserialize, Clone, Default)]
pub struct JsonMentionV1 {
    /// The length of the mention represented in the message. Seems to always be 1 but included here in case that changes.
    /// Example: 1
    #[serde(skip_serializing_if = "Option::is_none")]
    pub length: Option<i32>,
    /// The number of characters in that the mention starts at. Note that due to a quirk of how signald encodes JSON, if this value is 0 (for example if the first character in the message is the mention) the field won't show up.
    /// Example: 4
    #[serde(skip_serializing_if = "Option::is_none")]
    pub start: Option<i32>,
    /// The UUID of the account being mentioned
    /// Example: "aeed01f0-a234-478e-8cf7-261c283151e7"
    #[serde(skip_serializing_if = "Option::is_none")]
    pub uuid: Option<String>,
}

#[derive(Serialize, Deserialize, Clone, Default)]
pub struct JsonMessageEnvelopeV1 {
    #[serde(skip_serializing_if = "Option::is_none")]
    #[serde(rename = "callMessage")]
    pub call_message: Option<JsonCallMessageV0>,
    #[serde(skip_serializing_if = "Option::is_none")]
    #[serde(rename = "dataMessage")]
    pub data_message: Option<JsonDataMessageV1>,
    #[serde(skip_serializing_if = "Option::is_none")]
    #[serde(rename = "hasContent")]
    pub has_content: Option<bool>,
    #[serde(skip_serializing_if = "Option::is_none")]
    #[serde(rename = "hasLegacyMessage")]
    pub has_legacy_message: Option<bool>,
    #[serde(skip_serializing_if = "Option::is_none")]
    #[serde(rename = "isUnidentifiedSender")]
    pub is_unidentified_sender: Option<bool>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub receipt: Option<JsonReceiptMessageV0>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub relay: Option<String>,
    /// Example: 161557644247580
    #[serde(skip_serializing_if = "Option::is_none")]
    #[serde(rename = "serverDeliveredTimestamp")]
    pub server_delivered_timestamp: Option<i64>,
    #[serde(skip_serializing_if = "Option::is_none")]
    #[serde(rename = "serverTimestamp")]
    pub server_timestamp: Option<i64>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub source: Option<JsonAddressV1>,
    #[serde(skip_serializing_if = "Option::is_none")]
    #[serde(rename = "sourceDevice")]
    pub source_device: Option<i32>,
    #[serde(skip_serializing_if = "Option::is_none")]
    #[serde(rename = "syncMessage")]
    pub sync_message: Option<JsonSyncMessageV1>,
    /// Example: 1615576442475
    #[serde(skip_serializing_if = "Option::is_none")]
    pub timestamp: Option<i64>,
    #[serde(skip_serializing_if = "Option::is_none")]
    #[serde(rename = "timestampISO")]
    pub timestamp_iso: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    #[serde(rename = "type")]
    pub type_: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub typing: Option<JsonTypingMessageV0>,
    /// Example: "+12024561414"
    #[serde(skip_serializing_if = "Option::is_none")]
    pub username: Option<String>,
    /// Example: "0cc10e61-d64c-4dbc-b51c-334f7dd45a4a"
    #[serde(skip_serializing_if = "Option::is_none")]
    pub uuid: Option<String>,
}

#[derive(Serialize, Deserialize, Clone, Default)]
pub struct JsonMessageRequestResponseMessageV1 {
    #[serde(skip_serializing_if = "Option::is_none")]
    #[serde(rename = "groupId")]
    pub group_id: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub person: Option<JsonAddressV1>,
    #[serde(skip_serializing_if = "Option::is_none")]
    #[serde(rename = "type")]
    pub type_: Option<String>,
}

/// metadata about one of the links in a message
#[derive(Serialize, Deserialize, Clone, Default)]
pub struct JsonPreviewV1 {
    /// an optional image file attached to the preview
    #[serde(skip_serializing_if = "Option::is_none")]
    pub attachment: Option<JsonAttachmentV1>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub date: Option<i64>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub description: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub title: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub url: Option<String>,
}

/// A quote is a reply to a previous message. ID is the sent time of the message being replied to
#[derive(Serialize, Deserialize, Clone, Default)]
pub struct JsonQuoteV1 {
    /// list of files attached to the quoted message
    #[serde(skip_serializing_if = "Option::is_none")]
    pub attachments: Option<Vec<JsonQuotedAttachmentV0>>,
    /// the author of the message being quoted
    #[serde(skip_serializing_if = "Option::is_none")]
    pub author: Option<JsonAddressV1>,
    /// the client timestamp of the message being quoted
    /// Example: 1615576442475
    #[serde(skip_serializing_if = "Option::is_none")]
    pub id: Option<i64>,
    /// list of mentions in the quoted message
    #[serde(skip_serializing_if = "Option::is_none")]
    pub mentions: Option<Vec<JsonMentionV1>>,
    /// the body of the message being quoted
    /// Example: "hey ? what's up?"
    #[serde(skip_serializing_if = "Option::is_none")]
    pub text: Option<String>,
}

#[derive(Serialize, Deserialize, Clone, Default)]
pub struct JsonReactionV1 {
    /// the emoji to react with
    /// Example: "?"
    #[serde(skip_serializing_if = "Option::is_none")]
    pub emoji: Option<String>,
    /// set to true to remove the reaction. requires emoji be set to previously reacted emoji
    #[serde(skip_serializing_if = "Option::is_none")]
    pub remove: Option<bool>,
    /// the author of the message being reacted to
    #[serde(skip_serializing_if = "Option::is_none")]
    #[serde(rename = "targetAuthor")]
    pub target_author: Option<JsonAddressV1>,
    /// the client timestamp of the message being reacted to
    /// Example: 1615576442475
    #[serde(skip_serializing_if = "Option::is_none")]
    #[serde(rename = "targetSentTimestamp")]
    pub target_sent_timestamp: Option<i64>,
}

#[derive(Serialize, Deserialize, Clone, Default)]
pub struct JsonReadMessageV1 {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub sender: Option<JsonAddressV1>,
    /// Example: 1615576442475
    #[serde(skip_serializing_if = "Option::is_none")]
    pub timestamp: Option<i64>,
}

#[derive(Serialize, Deserialize, Clone, Default)]
pub struct JsonSendMessageResultV1 {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub address: Option<JsonAddressV1>,
    #[serde(skip_serializing_if = "Option::is_none")]
    #[serde(rename = "identityFailure")]
    pub identity_failure: Option<String>,
    /// Example: false
    #[serde(skip_serializing_if = "Option::is_none")]
    #[serde(rename = "networkFailure")]
    pub network_failure: Option<bool>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub success: Option<SendSuccessV1>,
    /// Example: false
    #[serde(skip_serializing_if = "Option::is_none")]
    #[serde(rename = "unregisteredFailure")]
    pub unregistered_failure: Option<bool>,
}

#[derive(Serialize, Deserialize, Clone, Default)]
pub struct JsonSentTranscriptMessageV1 {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub destination: Option<JsonAddressV1>,
    #[serde(skip_serializing_if = "Option::is_none")]
    #[serde(rename = "expirationStartTimestamp")]
    pub expiration_start_timestamp: Option<i64>,
    #[serde(skip_serializing_if = "Option::is_none")]
    #[serde(rename = "isRecipientUpdate")]
    pub is_recipient_update: Option<bool>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub message: Option<JsonDataMessageV1>,
    /// Example: 1615576442475
    #[serde(skip_serializing_if = "Option::is_none")]
    pub timestamp: Option<i64>,
    #[serde(skip_serializing_if = "Option::is_none")]
    #[serde(rename = "unidentifiedStatus")]
    pub unidentified_status: Option<HashMap<String, String>>,
}

#[derive(Serialize, Deserialize, Clone, Default)]
pub struct JsonSyncMessageV1 {
    #[serde(skip_serializing_if = "Option::is_none")]
    #[serde(rename = "blockedList")]
    pub blocked_list: Option<JsonBlockedListMessageV1>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub configuration: Option<ConfigurationMessageV0>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub contacts: Option<JsonAttachmentV1>,
    #[serde(skip_serializing_if = "Option::is_none")]
    #[serde(rename = "contactsComplete")]
    pub contacts_complete: Option<bool>,
    #[serde(skip_serializing_if = "Option::is_none")]
    #[serde(rename = "fetchType")]
    pub fetch_type: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub groups: Option<JsonAttachmentV1>,
    #[serde(skip_serializing_if = "Option::is_none")]
    #[serde(rename = "messageRequestResponse")]
    pub message_request_response: Option<JsonMessageRequestResponseMessageV1>,
    #[serde(skip_serializing_if = "Option::is_none")]
    #[serde(rename = "readMessages")]
    pub read_messages: Option<Vec<JsonReadMessageV1>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub request: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub sent: Option<JsonSentTranscriptMessageV1>,
    #[serde(skip_serializing_if = "Option::is_none")]
    #[serde(rename = "stickerPackOperations")]
    pub sticker_pack_operations: Option<Vec<JsonStickerPackOperationMessageV0>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub verified: Option<JsonVerifiedMessageV1>,
    #[serde(skip_serializing_if = "Option::is_none")]
    #[serde(rename = "viewOnceOpen")]
    pub view_once_open: Option<JsonViewOnceOpenMessageV1>,
}

#[derive(Serialize, Deserialize, Clone, Default)]
pub struct JsonVerifiedMessageV1 {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub destination: Option<JsonAddressV1>,
    #[serde(skip_serializing_if = "Option::is_none")]
    #[serde(rename = "identityKey")]
    pub identity_key: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub timestamp: Option<i64>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub verified: Option<String>,
}

#[derive(Serialize, Deserialize, Clone, Default)]
pub struct JsonVersionMessageV1 {
    /// Example: "main"
    #[serde(skip_serializing_if = "Option::is_none")]
    pub branch: Option<String>,
    /// Example: "7fb323214faf9924355b73a6a383ce8c0137c8d0"
    #[serde(skip_serializing_if = "Option::is_none")]
    pub commit: Option<String>,
    /// Example: "signald"
    #[serde(skip_serializing_if = "Option::is_none")]
    pub name: Option<String>,
    /// Example: "0.15.0-6-7fb32321"
    #[serde(skip_serializing_if = "Option::is_none")]
    pub version: Option<String>,
}

#[derive(Serialize, Deserialize, Clone, Default)]
pub struct JsonViewOnceOpenMessageV1 {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub sender: Option<JsonAddressV1>,
    /// Example: 1615576442475
    #[serde(skip_serializing_if = "Option::is_none")]
    pub timestamp: Option<i64>,
}

#[derive(Serialize, Deserialize, Clone, Default)]
pub struct LeaveGroupRequestV1 {
    /// The account to use
    /// Example: "+12024561414"
    #[serde(skip_serializing_if = "Option::is_none")]
    pub account: Option<String>,
    /// The group to leave
    /// Example: "EdSqI90cS0UomDpgUXOlCoObWvQOXlH5G3Z2d3f4ayE="
    #[serde(skip_serializing_if = "Option::is_none")]
    #[serde(rename = "groupID")]
    pub group_id: Option<String>,
}

#[derive(Serialize, Deserialize, Clone, Default)]
pub struct LinkedDevicesV1 {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub devices: Option<Vec<DeviceInfoV1>>,
}

#[derive(Serialize, Deserialize, Clone, Default)]
pub struct LinkingURIV1 {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub session_id: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub uri: Option<String>,
}

/// return all local accounts
#[derive(Serialize, Deserialize, Clone, Default)]
pub struct ListAccountsRequestV1;

#[derive(Serialize, Deserialize, Clone, Default)]
pub struct ListContactsRequestV1 {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub account: Option<String>,
    /// return results from local store immediately, refreshing from server afterward if needed. If false (default), block until all pending profiles have been retrieved.
    #[serde(skip_serializing_if = "Option::is_none")]
    #[serde(rename = "async")]
    pub async_: Option<bool>,
}

#[derive(Serialize, Deserialize, Clone, Default)]
pub struct ListGroupsRequestV1 {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub account: Option<String>,
}

/// indicates when the incoming connection to the signal server has started or stopped
#[derive(Serialize, Deserialize, Clone, Default)]
pub struct ListenerStateV1 {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub connected: Option<bool>,
}

#[derive(Serialize, Deserialize, Clone, Default)]
pub struct MarkReadRequestV1 {
    /// The account to interact with
    /// Example: "+12024561414"
    #[serde(skip_serializing_if = "Option::is_none")]
    pub account: Option<String>,
    /// List of messages to mark as read
    /// Example: 1615576442475
    #[serde(skip_serializing_if = "Option::is_none")]
    pub timestamps: Option<Vec<i64>>,
    /// The address that sent the message being marked as read
    #[serde(skip_serializing_if = "Option::is_none")]
    pub to: Option<JsonAddressV1>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub when: Option<i64>,
}

#[derive(Serialize, Deserialize, Clone, Default)]
pub struct NoKnownUUIDErrorV1 {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub message: Option<String>,
}

#[derive(Serialize, Deserialize, Clone, Default)]
pub struct NoSendPermissionErrorV1 {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub message: Option<String>,
}

#[derive(Serialize, Deserialize, Clone, Default)]
pub struct NoSuchAccountErrorV1 {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub account: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub message: Option<String>,
}

#[derive(Serialize, Deserialize, Clone, Default)]
pub struct NoSuchSessionErrorV1 {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub message: Option<String>,
}

#[derive(Serialize, Deserialize, Clone, Default)]
pub struct OfferMessageV1 {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub id: Option<i64>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub opaque: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub sdp: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    #[serde(rename = "type")]
    pub type_: Option<String>,
}

#[derive(Serialize, Deserialize, Clone, Default)]
pub struct OwnProfileKeyDoesNotExistErrorV1 {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub message: Option<String>,
}

/// details about a MobileCoin payment
#[derive(Serialize, Deserialize, Clone, Default)]
pub struct PaymentV1 {
    /// note attached to the payment
    #[serde(skip_serializing_if = "Option::is_none")]
    pub note: Option<String>,
    /// base64 encoded payment receipt data. This is a protobuf value which can be decoded as the Receipt object described in https://github.com/mobilecoinfoundation/mobilecoin/blob/master/api/proto/external.proto
    #[serde(skip_serializing_if = "Option::is_none")]
    pub receipt: Option<String>,
}

/// Information about a Signal user
#[derive(Serialize, Deserialize, Clone, Default)]
pub struct ProfileV1 {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub about: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub address: Option<JsonAddressV1>,
    /// path to avatar on local disk
    #[serde(skip_serializing_if = "Option::is_none")]
    pub avatar: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub capabilities: Option<CapabilitiesV1>,
    /// color of the chat with this user
    #[serde(skip_serializing_if = "Option::is_none")]
    pub color: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub emoji: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub expiration_time: Option<i32>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub inbox_position: Option<i32>,
    /// *base64-encoded* mobilecoin address. Note that this is not the traditional MobileCoin address encoding. Clients are responsible for converting between MobileCoin's custom base58 on the user-facing side and base64 encoding on the signald side. If unset, null or an empty string, will empty the profile payment address
    #[serde(skip_serializing_if = "Option::is_none")]
    pub mobilecoin_address: Option<String>,
    /// The user's name from local contact names if available, or if not in contact list their Signal profile name
    #[serde(skip_serializing_if = "Option::is_none")]
    pub name: Option<String>,
    /// The user's Signal profile name
    #[serde(skip_serializing_if = "Option::is_none")]
    pub profile_name: Option<String>,
}

#[derive(Serialize, Deserialize, Clone, Default)]
pub struct ProfileListV1 {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub profiles: Option<Vec<ProfileV1>>,
}

#[derive(Serialize, Deserialize, Clone, Default)]
pub struct ProfileUnavailableErrorV1 {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub message: Option<String>,
}

#[derive(Serialize, Deserialize, Clone, Default)]
pub struct RateLimitErrorV1 {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub message: Option<String>,
}

/// react to a previous message
#[derive(Serialize, Deserialize, Clone, Default)]
pub struct ReactRequestV1 {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub reaction: Option<JsonReactionV1>,
    #[serde(skip_serializing_if = "Option::is_none")]
    #[serde(rename = "recipientAddress")]
    pub recipient_address: Option<JsonAddressV1>,
    /// Example: "EdSqI90cS0UomDpgUXOlCoObWvQOXlH5G3Z2d3f4ayE="
    #[serde(skip_serializing_if = "Option::is_none")]
    #[serde(rename = "recipientGroupId")]
    pub recipient_group_id: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub timestamp: Option<i64>,
    /// Example: "+12024561414"
    #[serde(skip_serializing_if = "Option::is_none")]
    pub username: Option<String>,
}

#[derive(Serialize, Deserialize, Clone, Default)]
pub struct ReceiptMessageV1 {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub timestamps: Option<Vec<i64>>,
    /// options: UNKNOWN, DELIVERY, READ, VIEWED
    #[serde(skip_serializing_if = "Option::is_none")]
    #[serde(rename = "type")]
    pub type_: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub when: Option<i64>,
}

/// deny a request to join a group
#[derive(Serialize, Deserialize, Clone, Default)]
pub struct RefuseMembershipRequestV1 {
    /// The account to interact with
    /// Example: "+12024561414"
    #[serde(skip_serializing_if = "Option::is_none")]
    pub account: Option<String>,
    /// Example: "EdSqI90cS0UomDpgUXOlCoObWvQOXlH5G3Z2d3f4ayE="
    #[serde(skip_serializing_if = "Option::is_none")]
    pub group_id: Option<String>,
    /// list of requesting members to refuse
    #[serde(skip_serializing_if = "Option::is_none")]
    pub members: Option<Vec<JsonAddressV1>>,
}

/// begin the account registration process by requesting a phone number verification code. when the code is received, submit it with a verify request
#[derive(Serialize, Deserialize, Clone, Default)]
pub struct RegisterRequestV1 {
    /// the e164 phone number to register with
    /// Example: "+12024561414"
    #[serde(skip_serializing_if = "Option::is_none")]
    pub account: Option<String>,
    /// See https://signald.org/articles/captcha/
    #[serde(skip_serializing_if = "Option::is_none")]
    pub captcha: Option<String>,
    /// The identifier of the server to use. Leave blank for default (usually Signal production servers but configurable at build time)
    #[serde(skip_serializing_if = "Option::is_none")]
    pub server: Option<String>,
    /// set to true to request a voice call instead of an SMS for verification
    #[serde(skip_serializing_if = "Option::is_none")]
    pub voice: Option<bool>,
}

/// A remote config (feature flag) entry.
#[derive(Serialize, Deserialize, Clone, Default)]
pub struct RemoteConfigV1 {
    /// The name of this remote config entry. These names may be prefixed with the platform type ("android.", "ios.", "desktop.", etc.) Typically, clients only handle the relevant configs for its platform, hardcoding the names it cares about handling and ignoring the rest.
    /// Example: desktop.mediaQuality.levels
    #[serde(skip_serializing_if = "Option::is_none")]
    pub name: Option<String>,
    /// The value for this remote config entry. Even though this is a string, it could be a boolean as a string, an integer/long value, a comma-delimited list, etc. Clients usually consume this by hardcoding the feature flagsit should track in the app and assuming that the server will send the type that the client expects. If an unexpected type occurs, it falls back to a default value.
    /// Example: 1:2,61:2,81:2,82:2,65:2,31:2,47:2,41:2,32:2,385:2,971:2,974:2,49:2,33:2,*:1
    #[serde(skip_serializing_if = "Option::is_none")]
    pub value: Option<String>,
}

#[derive(Serialize, Deserialize, Clone, Default)]
pub struct RemoteConfigListV1 {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub config: Option<Vec<RemoteConfigV1>>,
}

/// Retrieves the remote config (feature flags) from the server.
#[derive(Serialize, Deserialize, Clone, Default)]
pub struct RemoteConfigRequestV1 {
    /// The account to use to retrieve the remote config
    /// Example: "+12024561414"
    #[serde(skip_serializing_if = "Option::is_none")]
    pub account: Option<String>,
}

#[derive(Serialize, Deserialize, Clone, Default)]
pub struct RemoteDeleteV1 {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub target_sent_timestamp: Option<i64>,
}

/// delete a message previously sent
#[derive(Serialize, Deserialize, Clone, Default)]
pub struct RemoteDeleteRequestV1 {
    /// the account to use
    /// Example: "+12024561414"
    #[serde(skip_serializing_if = "Option::is_none")]
    pub account: Option<String>,
    /// the address to send the delete message to. should match address the message to be deleted was sent to. required if group is not set.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub address: Option<JsonAddressV1>,
    /// the group to send the delete message to. should match group the message to be deleted was sent to. required if address is not set.
    /// Example: "EdSqI90cS0UomDpgUXOlCoObWvQOXlH5G3Z2d3f4ayE="
    #[serde(skip_serializing_if = "Option::is_none")]
    pub group: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub timestamp: Option<i64>,
}

/// Remove a linked device from the Signal account. Only allowed when the local device id is 1
#[derive(Serialize, Deserialize, Clone, Default)]
pub struct RemoveLinkedDeviceRequestV1 {
    /// The account to interact with
    /// Example: "+12024561414"
    #[serde(skip_serializing_if = "Option::is_none")]
    pub account: Option<String>,
    /// the ID of the device to unlink
    /// Example: 3
    #[serde(skip_serializing_if = "Option::is_none")]
    #[serde(rename = "deviceId")]
    pub device_id: Option<i64>,
}

#[derive(Serialize, Deserialize, Clone, Default)]
pub struct RemoveServerRequestV1 {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub uuid: Option<String>,
}

/// Request other devices on the account send us their group list, syncable config and contact list.
#[derive(Serialize, Deserialize, Clone, Default)]
pub struct RequestSyncRequestV1 {
    /// The account to use
    /// Example: "+12024561414"
    #[serde(skip_serializing_if = "Option::is_none")]
    pub account: Option<String>,
    /// request block list sync (default true)
    #[serde(skip_serializing_if = "Option::is_none")]
    pub blocked: Option<bool>,
    /// request configuration sync (default true)
    #[serde(skip_serializing_if = "Option::is_none")]
    pub configuration: Option<bool>,
    /// request contact sync (default true)
    #[serde(skip_serializing_if = "Option::is_none")]
    pub contacts: Option<bool>,
    /// request group sync (default true)
    #[serde(skip_serializing_if = "Option::is_none")]
    pub groups: Option<bool>,
}

/// reset a session with a particular user
#[derive(Serialize, Deserialize, Clone, Default)]
pub struct ResetSessionRequestV1 {
    /// The account to use
    /// Example: "+12024561414"
    #[serde(skip_serializing_if = "Option::is_none")]
    pub account: Option<String>,
    /// the user to reset session with
    #[serde(skip_serializing_if = "Option::is_none")]
    pub address: Option<JsonAddressV1>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub timestamp: Option<i64>,
}

/// Resolve a partial JsonAddress with only a number or UUID to one with both. Anywhere that signald accepts a JsonAddress will except a partial, this is a convenience function for client authors, mostly because signald doesn't resolve all the partials it returns.
#[derive(Serialize, Deserialize, Clone, Default)]
pub struct ResolveAddressRequestV1 {
    /// The signal account to use
    /// Example: "+12024561414"
    #[serde(skip_serializing_if = "Option::is_none")]
    pub account: Option<String>,
    /// The partial address, missing fields
    #[serde(skip_serializing_if = "Option::is_none")]
    pub partial: Option<JsonAddressV1>,
}

/// send a mobilecoin payment
#[derive(Serialize, Deserialize, Clone, Default)]
pub struct SendPaymentRequestV1 {
    /// the account to use
    /// Example: "+12024561414"
    #[serde(skip_serializing_if = "Option::is_none")]
    pub account: Option<String>,
    /// the address to send the payment message to
    #[serde(skip_serializing_if = "Option::is_none")]
    pub address: Option<JsonAddressV1>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub payment: Option<PaymentV1>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub when: Option<i64>,
}

#[derive(Serialize, Deserialize, Clone, Default)]
pub struct SendRequestV1 {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub attachments: Option<Vec<JsonAttachmentV1>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub mentions: Option<Vec<JsonMentionV1>>,
    /// Example: "hello"
    #[serde(skip_serializing_if = "Option::is_none")]
    #[serde(rename = "messageBody")]
    pub message_body: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub previews: Option<Vec<JsonPreviewV1>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub quote: Option<JsonQuoteV1>,
    #[serde(skip_serializing_if = "Option::is_none")]
    #[serde(rename = "recipientAddress")]
    pub recipient_address: Option<JsonAddressV1>,
    /// Example: "EdSqI90cS0UomDpgUXOlCoObWvQOXlH5G3Z2d3f4ayE="
    #[serde(skip_serializing_if = "Option::is_none")]
    #[serde(rename = "recipientGroupId")]
    pub recipient_group_id: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub timestamp: Option<i64>,
    /// Example: "+12024561414"
    #[serde(skip_serializing_if = "Option::is_none")]
    pub username: Option<String>,
}

#[derive(Serialize, Deserialize, Clone, Default)]
pub struct SendResponseV1 {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub results: Option<Vec<JsonSendMessageResultV1>>,
    /// Example: 1615576442475
    #[serde(skip_serializing_if = "Option::is_none")]
    pub timestamp: Option<i64>,
}

#[derive(Serialize, Deserialize, Clone, Default)]
pub struct SendSuccessV1 {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub devices: Option<Vec<i32>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub duration: Option<i64>,
    #[serde(skip_serializing_if = "Option::is_none")]
    #[serde(rename = "needsSync")]
    pub needs_sync: Option<bool>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub unidentified: Option<bool>,
}

/// a Signal server
#[derive(Serialize, Deserialize, Clone, Default)]
pub struct ServerV1 {
    /// base64 encoded trust store, password must be 'whisper'
    #[serde(skip_serializing_if = "Option::is_none")]
    pub ca: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub cdn_urls: Option<Vec<ServerCDNV1>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub cds_mrenclave: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub contact_discovery_url: Option<String>,
    /// base64 encoded trust store, password must be 'whisper'
    #[serde(skip_serializing_if = "Option::is_none")]
    pub ias_ca: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub key_backup_mrenclave: Option<String>,
    /// base64 encoded
    #[serde(skip_serializing_if = "Option::is_none")]
    pub key_backup_service_id: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub key_backup_service_name: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub key_backup_url: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub proxy: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub service_url: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub storage_url: Option<String>,
    /// base64 encoded
    #[serde(skip_serializing_if = "Option::is_none")]
    pub unidentified_sender_root: Option<String>,
    /// A unique identifier for the server, referenced when adding accounts. Must be a valid UUID. Will be generated if not specified when creating.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub uuid: Option<String>,
    /// base64 encoded ZKGROUP_SERVER_PUBLIC_PARAMS value
    #[serde(skip_serializing_if = "Option::is_none")]
    pub zk_param: Option<String>,
}

#[derive(Serialize, Deserialize, Clone, Default)]
pub struct ServerCDNV1 {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub number: Option<i32>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub url: Option<String>,
}

#[derive(Serialize, Deserialize, Clone, Default)]
pub struct ServerListV1 {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub servers: Option<Vec<ServerV1>>,
}

#[derive(Serialize, Deserialize, Clone, Default)]
pub struct ServerNotFoundErrorV1 {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub message: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub uuid: Option<String>,
}

/// set this device's name. This will show up on the mobile device on the same account under 
#[derive(Serialize, Deserialize, Clone, Default)]
pub struct SetDeviceNameRequestV1 {
    /// The account to set the device name of
    /// Example: "+12024561414"
    #[serde(skip_serializing_if = "Option::is_none")]
    pub account: Option<String>,
    /// The device name
    #[serde(skip_serializing_if = "Option::is_none")]
    pub device_name: Option<String>,
}

/// Set the message expiration timer for a thread. Expiration must be specified in seconds, set to 0 to disable timer
#[derive(Serialize, Deserialize, Clone, Default)]
pub struct SetExpirationRequestV1 {
    /// The account to use
    /// Example: "+12024561414"
    #[serde(skip_serializing_if = "Option::is_none")]
    pub account: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub address: Option<JsonAddressV1>,
    /// Example: 604800
    #[serde(skip_serializing_if = "Option::is_none")]
    pub expiration: Option<i32>,
    /// Example: "EdSqI90cS0UomDpgUXOlCoObWvQOXlH5G3Z2d3f4ayE="
    #[serde(skip_serializing_if = "Option::is_none")]
    pub group: Option<String>,
}

#[derive(Serialize, Deserialize, Clone, Default)]
pub struct SetProfileV1 {
    /// an optional about string. If unset, null or an empty string will unset profile about field
    #[serde(skip_serializing_if = "Option::is_none")]
    pub about: Option<String>,
    /// The phone number of the account to use
    /// Example: "+12024561414"
    #[serde(skip_serializing_if = "Option::is_none")]
    pub account: Option<String>,
    /// Path to new profile avatar file. If unset or null, unset the profile avatar
    /// Example: "/tmp/image.jpg"
    #[serde(skip_serializing_if = "Option::is_none")]
    #[serde(rename = "avatarFile")]
    pub avatar_file: Option<String>,
    /// an optional single emoji character. If unset, null or an empty string will unset profile emoji
    #[serde(skip_serializing_if = "Option::is_none")]
    pub emoji: Option<String>,
    /// an optional *base64-encoded* MobileCoin address to set in the profile. Note that this is not the traditional MobileCoin address encoding, which is custom. Clients are responsible for converting between MobileCoin's custom base58 on the user-facing side and base64 encoding on the signald side. If unset, null or an empty string, will empty the profile payment address
    #[serde(skip_serializing_if = "Option::is_none")]
    pub mobilecoin_address: Option<String>,
    /// New profile name. Set to empty string for no profile name
    /// Example: "signald user"
    #[serde(skip_serializing_if = "Option::is_none")]
    pub name: Option<String>,
}

/// receive incoming messages. After making a subscribe request, incoming messages will be sent to the client encoded as ClientMessageWrapper. Send an unsubscribe request or disconnect from the socket to stop receiving messages.
#[derive(Serialize, Deserialize, Clone, Default)]
pub struct SubscribeRequestV1 {
    /// The account to subscribe to incoming message for
    /// Example: "+12024561414"
    #[serde(skip_serializing_if = "Option::is_none")]
    pub account: Option<String>,
}

/// Trust another user's safety number using either the QR code data or the safety number text
#[derive(Serialize, Deserialize, Clone, Default)]
pub struct TrustRequestV1 {
    /// The account to interact with
    /// Example: "+12024561414"
    #[serde(skip_serializing_if = "Option::is_none")]
    pub account: Option<String>,
    /// The user to query identity keys for
    #[serde(skip_serializing_if = "Option::is_none")]
    pub address: Option<JsonAddressV1>,
    /// base64-encoded QR code data. required if safety_number is absent
    #[serde(skip_serializing_if = "Option::is_none")]
    pub qr_code_data: Option<String>,
    /// required if qr_code_data is absent
    /// Example: "373453558586758076680580548714989751943247272727416091564451"
    #[serde(skip_serializing_if = "Option::is_none")]
    pub safety_number: Option<String>,
    /// One of TRUSTED_UNVERIFIED, TRUSTED_VERIFIED or UNTRUSTED. Default is TRUSTED_VERIFIED
    /// Example: "TRUSTED_VERIFIED"
    #[serde(skip_serializing_if = "Option::is_none")]
    pub trust_level: Option<String>,
}

#[derive(Serialize, Deserialize, Clone, Default)]
pub struct TypingMessageV1 {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub action: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub group_id: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub timestamp: Option<i64>,
}

/// send a typing started or stopped message
#[derive(Serialize, Deserialize, Clone, Default)]
pub struct TypingRequestV1 {
    /// The account to use
    /// Example: "+12024561414"
    #[serde(skip_serializing_if = "Option::is_none")]
    pub account: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub address: Option<JsonAddressV1>,
    /// Example: "EdSqI90cS0UomDpgUXOlCoObWvQOXlH5G3Z2d3f4ayE="
    #[serde(skip_serializing_if = "Option::is_none")]
    pub group: Option<String>,
    /// Example: true
    #[serde(skip_serializing_if = "Option::is_none")]
    pub typing: Option<bool>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub when: Option<i64>,
}

#[derive(Serialize, Deserialize, Clone, Default)]
pub struct UnknownGroupErrorV1 {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub message: Option<String>,
}

#[derive(Serialize, Deserialize, Clone, Default)]
pub struct UnknownIdentityKeyErrorV1 {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub message: Option<String>,
}

/// See subscribe for more info
#[derive(Serialize, Deserialize, Clone, Default)]
pub struct UnsubscribeRequestV1 {
    /// The account to unsubscribe from
    /// Example: "+12024561414"
    #[serde(skip_serializing_if = "Option::is_none")]
    pub account: Option<String>,
}

#[derive(Serialize, Deserialize, Clone, Default)]
pub struct UntrustedIdentityErrorV1 {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub identifier: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub identity_key: Option<IdentityKeyV1>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub message: Option<String>,
}

/// update information about a local contact
#[derive(Serialize, Deserialize, Clone, Default)]
pub struct UpdateContactRequestV1 {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub account: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub address: Option<JsonAddressV1>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub color: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub inbox_position: Option<i32>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub name: Option<String>,
}

/// modify a group. Note that only one modification action may be performed at once
#[derive(Serialize, Deserialize, Clone, Default)]
pub struct UpdateGroupRequestV1 {
    /// The identifier of the account to interact with
    /// Example: "+12024561414"
    #[serde(skip_serializing_if = "Option::is_none")]
    pub account: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    #[serde(rename = "addMembers")]
    pub add_members: Option<Vec<JsonAddressV1>>,
    /// ENABLED to only allow admins to post messages, DISABLED to allow anyone to post
    #[serde(skip_serializing_if = "Option::is_none")]
    pub announcements: Option<String>,
    /// Example: "/tmp/image.jpg"
    #[serde(skip_serializing_if = "Option::is_none")]
    pub avatar: Option<String>,
    /// A new group description. Set to empty string to remove an existing description.
    /// Example: "A club for running in Parkdale"
    #[serde(skip_serializing_if = "Option::is_none")]
    pub description: Option<String>,
    /// the ID of the group to update
    /// Example: "EdSqI90cS0UomDpgUXOlCoObWvQOXlH5G3Z2d3f4ayE="
    #[serde(skip_serializing_if = "Option::is_none")]
    #[serde(rename = "groupID")]
    pub group_id: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    #[serde(rename = "removeMembers")]
    pub remove_members: Option<Vec<JsonAddressV1>>,
    /// regenerate the group link password, invalidating the old one
    #[serde(skip_serializing_if = "Option::is_none")]
    #[serde(rename = "resetLink")]
    pub reset_link: Option<bool>,
    /// Example: "Parkdale Run Club"
    #[serde(skip_serializing_if = "Option::is_none")]
    pub title: Option<String>,
    /// note that only one of the access controls may be updated per request
    #[serde(skip_serializing_if = "Option::is_none")]
    #[serde(rename = "updateAccessControl")]
    pub update_access_control: Option<GroupAccessControlV1>,
    #[serde(skip_serializing_if = "Option::is_none")]
    #[serde(rename = "updateRole")]
    pub update_role: Option<GroupMemberV1>,
    /// update the group timer.
    #[serde(skip_serializing_if = "Option::is_none")]
    #[serde(rename = "updateTimer")]
    pub update_timer: Option<i32>,
}

#[derive(Serialize, Deserialize, Clone, Default)]
pub struct UserAlreadyExistsErrorV1 {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub message: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub uuid: Option<String>,
}

/// verify an account's phone number with a code after registering, completing the account creation process
#[derive(Serialize, Deserialize, Clone, Default)]
pub struct VerifyRequestV1 {
    /// the e164 phone number being verified
    /// Example: "+12024561414"
    #[serde(skip_serializing_if = "Option::is_none")]
    pub account: Option<String>,
    /// the verification code, dash (-) optional
    /// Example: "555555"
    #[serde(skip_serializing_if = "Option::is_none")]
    pub code: Option<String>,
}

#[derive(Serialize, Deserialize, Clone, Default)]
pub struct VersionRequestV1;

#[derive(Serialize, Deserialize, Clone, Default)]
pub struct AnswerMessageV0 {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub id: Option<i64>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub opaque: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub sdp: Option<String>,
}

#[derive(Serialize, Deserialize, Clone, Default)]
pub struct BusyMessageV0 {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub id: Option<i64>,
}

#[derive(Serialize, Deserialize, Clone, Default)]
pub struct ConfigurationMessageV0 {
    #[serde(skip_serializing_if = "Option::is_none")]
    #[serde(rename = "linkPreviews")]
    pub link_previews: Option<OptionalV0>,
    #[serde(skip_serializing_if = "Option::is_none")]
    #[serde(rename = "readReceipts")]
    pub read_receipts: Option<OptionalV0>,
    #[serde(skip_serializing_if = "Option::is_none")]
    #[serde(rename = "typingIndicators")]
    pub typing_indicators: Option<OptionalV0>,
    #[serde(skip_serializing_if = "Option::is_none")]
    #[serde(rename = "unidentifiedDeliveryIndicators")]
    pub unidentified_delivery_indicators: Option<OptionalV0>,
}

/// group access control settings. Options for each controlled action are: UNKNOWN, ANY, MEMBER, ADMINISTRATOR, UNSATISFIABLE and UNRECOGNIZED
#[derive(Serialize, Deserialize, Clone, Default)]
pub struct GroupAccessControlV0 {
    /// who can edit group info
    #[serde(skip_serializing_if = "Option::is_none")]
    pub attributes: Option<String>,
    /// UNSATISFIABLE when the group link is disabled, ADMINISTRATOR when the group link is enabled but an administrator must approve new members, ANY when the group link is enabled and no approval is required
    /// Example: "ANY"
    #[serde(skip_serializing_if = "Option::is_none")]
    pub link: Option<String>,
    /// who can add members
    #[serde(skip_serializing_if = "Option::is_none")]
    pub members: Option<String>,
}

#[derive(Serialize, Deserialize, Clone, Default)]
pub struct GroupMemberV0 {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub joined_revision: Option<i32>,
    /// possible values are: UNKNOWN, DEFAULT, ADMINISTRATOR and UNRECOGNIZED
    /// Example: "DEFAULT"
    #[serde(skip_serializing_if = "Option::is_none")]
    pub role: Option<String>,
    /// Example: "aeed01f0-a234-478e-8cf7-261c283151e7"
    #[serde(skip_serializing_if = "Option::is_none")]
    pub uuid: Option<String>,
}

#[derive(Serialize, Deserialize, Clone, Default)]
pub struct HangupMessageV0 {
    #[serde(skip_serializing_if = "Option::is_none")]
    #[serde(rename = "deviceId")]
    pub device_id: Option<i32>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub id: Option<i64>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub legacy: Option<bool>,
    #[serde(skip_serializing_if = "Option::is_none")]
    #[serde(rename = "type")]
    pub type_: Option<TypeV0>,
}

#[derive(Serialize, Deserialize, Clone, Default)]
pub struct IceUpdateMessageV0 {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub id: Option<i64>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub opaque: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub sdp: Option<String>,
}

#[derive(Serialize, Deserialize, Clone, Default)]
pub struct JsonAccountV0 {
    #[serde(skip_serializing_if = "Option::is_none")]
    #[serde(rename = "deviceId")]
    pub device_id: Option<i32>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub filename: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub has_keys: Option<bool>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub registered: Option<bool>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub subscribed: Option<bool>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub username: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub uuid: Option<String>,
}

#[derive(Serialize, Deserialize, Clone, Default)]
pub struct JsonAccountListV0 {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub accounts: Option<Vec<JsonAccountV0>>,
}

#[derive(Serialize, Deserialize, Clone, Default)]
pub struct JsonAddressV0 {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub number: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub relay: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub uuid: Option<String>,
}

#[derive(Serialize, Deserialize, Clone, Default)]
pub struct JsonAttachmentV0 {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub blurhash: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub caption: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    #[serde(rename = "contentType")]
    pub content_type: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    #[serde(rename = "customFilename")]
    pub custom_filename: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub digest: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub filename: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub height: Option<i32>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub id: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub key: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub size: Option<i32>,
    #[serde(skip_serializing_if = "Option::is_none")]
    #[serde(rename = "storedFilename")]
    pub stored_filename: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    #[serde(rename = "voiceNote")]
    pub voice_note: Option<bool>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub width: Option<i32>,
}

#[derive(Serialize, Deserialize, Clone, Default)]
pub struct JsonBlockedListMessageV0 {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub addresses: Option<Vec<JsonAddressV0>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    #[serde(rename = "groupIds")]
    pub group_ids: Option<Vec<String>>,
}

#[derive(Serialize, Deserialize, Clone, Default)]
pub struct JsonCallMessageV0 {
    #[serde(skip_serializing_if = "Option::is_none")]
    #[serde(rename = "answerMessage")]
    pub answer_message: Option<AnswerMessageV0>,
    #[serde(skip_serializing_if = "Option::is_none")]
    #[serde(rename = "busyMessage")]
    pub busy_message: Option<BusyMessageV0>,
    #[serde(skip_serializing_if = "Option::is_none")]
    #[serde(rename = "destinationDeviceId")]
    pub destination_device_id: Option<i32>,
    #[serde(skip_serializing_if = "Option::is_none")]
    #[serde(rename = "hangupMessage")]
    pub hangup_message: Option<HangupMessageV0>,
    #[serde(skip_serializing_if = "Option::is_none")]
    #[serde(rename = "iceUpdateMessages")]
    pub ice_update_messages: Option<Vec<IceUpdateMessageV0>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    #[serde(rename = "isMultiRing")]
    pub is_multi_ring: Option<bool>,
    #[serde(skip_serializing_if = "Option::is_none")]
    #[serde(rename = "offerMessage")]
    pub offer_message: Option<OfferMessageV0>,
}

#[derive(Serialize, Deserialize, Clone, Default)]
pub struct JsonDataMessageV0 {
    /// files attached to the incoming message
    #[serde(skip_serializing_if = "Option::is_none")]
    pub attachments: Option<Vec<JsonAttachmentV0>>,
    /// the text body of the incoming message.
    /// Example: "hello"
    #[serde(skip_serializing_if = "Option::is_none")]
    pub body: Option<String>,
    /// if the incoming message has a shared contact, the contact's information will be here
    #[serde(skip_serializing_if = "Option::is_none")]
    pub contacts: Option<Vec<SharedContactV0>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    #[serde(rename = "endSession")]
    pub end_session: Option<bool>,
    /// the expiry timer on the incoming message. Clients should delete records of the message within this number of seconds
    #[serde(skip_serializing_if = "Option::is_none")]
    #[serde(rename = "expiresInSeconds")]
    pub expires_in_seconds: Option<i32>,
    /// if the incoming message was sent to a v1 group, information about that group will be here
    #[serde(skip_serializing_if = "Option::is_none")]
    pub group: Option<JsonGroupInfoV0>,
    /// is the incoming message was sent to a v2 group, basic identifying information about that group will be here. For full information, use list_groups
    #[serde(skip_serializing_if = "Option::is_none")]
    #[serde(rename = "groupV2")]
    pub group_v_2: Option<JsonGroupV2InfoV0>,
    /// list of mentions in the message
    #[serde(skip_serializing_if = "Option::is_none")]
    pub mentions: Option<Vec<JsonMentionV0>>,
    /// if the incoming message has a link preview, information about that preview will be here
    #[serde(skip_serializing_if = "Option::is_none")]
    pub previews: Option<Vec<JsonPreviewV0>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    #[serde(rename = "profileKeyUpdate")]
    pub profile_key_update: Option<bool>,
    /// if the incoming message is a quote or reply to another message, this will contain information about that message
    #[serde(skip_serializing_if = "Option::is_none")]
    pub quote: Option<JsonQuoteV0>,
    /// if the message adds or removes a reaction to another message, this will indicate what change is being made
    #[serde(skip_serializing_if = "Option::is_none")]
    pub reaction: Option<JsonReactionV0>,
    /// if the inbound message is deleting a previously sent message, indicates which message should be deleted
    #[serde(skip_serializing_if = "Option::is_none")]
    #[serde(rename = "remoteDelete")]
    pub remote_delete: Option<RemoteDeleteV0>,
    /// if the incoming message is a sticker, information about the sicker will be here
    #[serde(skip_serializing_if = "Option::is_none")]
    pub sticker: Option<JsonStickerV0>,
    /// the timestamp that the message was sent at, according to the sender's device. This is used to uniquely identify this message for things like reactions and quotes.
    /// Example: 1615576442475
    #[serde(skip_serializing_if = "Option::is_none")]
    pub timestamp: Option<i64>,
    /// indicates the message is a view once message. View once messages typically include no body and a single image attachment. Official Signal clients will prevent the user from saving the image, and once the user has viewed the image once they will destroy the image.
    #[serde(skip_serializing_if = "Option::is_none")]
    #[serde(rename = "viewOnce")]
    pub view_once: Option<bool>,
}

#[derive(Serialize, Deserialize, Clone, Default)]
pub struct JsonGroupInfoV0 {
    #[serde(skip_serializing_if = "Option::is_none")]
    #[serde(rename = "avatarId")]
    pub avatar_id: Option<i64>,
    #[serde(skip_serializing_if = "Option::is_none")]
    #[serde(rename = "groupId")]
    pub group_id: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub members: Option<Vec<JsonAddressV0>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub name: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    #[serde(rename = "type")]
    pub type_: Option<String>,
}

#[derive(Serialize, Deserialize, Clone, Default)]
pub struct JsonGroupV2InfoV0 {
    /// current access control settings for this group
    #[serde(skip_serializing_if = "Option::is_none")]
    #[serde(rename = "accessControl")]
    pub access_control: Option<GroupAccessControlV0>,
    /// path to the group's avatar on local disk, if available
    /// Example: "/var/lib/signald/avatars/group-EdSqI90cS0UomDpgUXOlCoObWvQOXlH5G3Z2d3f4ayE="
    #[serde(skip_serializing_if = "Option::is_none")]
    pub avatar: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub description: Option<String>,
    /// Example: "EdSqI90cS0UomDpgUXOlCoObWvQOXlH5G3Z2d3f4ayE="
    #[serde(skip_serializing_if = "Option::is_none")]
    pub id: Option<String>,
    /// the signal.group link, if applicable
    #[serde(skip_serializing_if = "Option::is_none")]
    #[serde(rename = "inviteLink")]
    pub invite_link: Option<String>,
    /// detailed member list
    #[serde(skip_serializing_if = "Option::is_none")]
    #[serde(rename = "memberDetail")]
    pub member_detail: Option<Vec<GroupMemberV0>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub members: Option<Vec<JsonAddressV0>>,
    /// detailed pending member list
    #[serde(skip_serializing_if = "Option::is_none")]
    #[serde(rename = "pendingMemberDetail")]
    pub pending_member_detail: Option<Vec<GroupMemberV0>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    #[serde(rename = "pendingMembers")]
    pub pending_members: Option<Vec<JsonAddressV0>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    #[serde(rename = "requestingMembers")]
    pub requesting_members: Option<Vec<JsonAddressV0>>,
    /// Example: 5
    #[serde(skip_serializing_if = "Option::is_none")]
    pub revision: Option<i32>,
    /// Example: 604800
    #[serde(skip_serializing_if = "Option::is_none")]
    pub timer: Option<i32>,
    /// Example: "Parkdale Run Club"
    #[serde(skip_serializing_if = "Option::is_none")]
    pub title: Option<String>,
}

#[derive(Serialize, Deserialize, Clone, Default)]
pub struct JsonMentionV0 {
    /// The length of the mention represented in the message. Seems to always be 1 but included here in case that changes.
    /// Example: 1
    #[serde(skip_serializing_if = "Option::is_none")]
    pub length: Option<i32>,
    /// The number of characters in that the mention starts at. Note that due to a quirk of how signald encodes JSON, if this value is 0 (for example if the first character in the message is the mention) the field won't show up.
    /// Example: 4
    #[serde(skip_serializing_if = "Option::is_none")]
    pub start: Option<i32>,
    /// The UUID of the account being mentioned
    /// Example: "aeed01f0-a234-478e-8cf7-261c283151e7"
    #[serde(skip_serializing_if = "Option::is_none")]
    pub uuid: Option<String>,
}

#[derive(Serialize, Deserialize, Clone, Default)]
pub struct JsonMessageEnvelopeV0 {
    #[serde(skip_serializing_if = "Option::is_none")]
    #[serde(rename = "callMessage")]
    pub call_message: Option<JsonCallMessageV0>,
    #[serde(skip_serializing_if = "Option::is_none")]
    #[serde(rename = "dataMessage")]
    pub data_message: Option<JsonDataMessageV0>,
    #[serde(skip_serializing_if = "Option::is_none")]
    #[serde(rename = "hasContent")]
    pub has_content: Option<bool>,
    #[serde(skip_serializing_if = "Option::is_none")]
    #[serde(rename = "hasLegacyMessage")]
    pub has_legacy_message: Option<bool>,
    #[serde(skip_serializing_if = "Option::is_none")]
    #[serde(rename = "isUnidentifiedSender")]
    pub is_unidentified_sender: Option<bool>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub receipt: Option<JsonReceiptMessageV0>,
    /// this field is no longer available and will never be populated
    #[serde(skip_serializing_if = "Option::is_none")]
    pub relay: Option<String>,
    /// Example: 161557644247580
    #[serde(skip_serializing_if = "Option::is_none")]
    #[serde(rename = "serverDeliveredTimestamp")]
    pub server_delivered_timestamp: Option<i64>,
    #[serde(skip_serializing_if = "Option::is_none")]
    #[serde(rename = "serverTimestamp")]
    pub server_timestamp: Option<i64>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub source: Option<JsonAddressV0>,
    #[serde(skip_serializing_if = "Option::is_none")]
    #[serde(rename = "sourceDevice")]
    pub source_device: Option<i32>,
    #[serde(skip_serializing_if = "Option::is_none")]
    #[serde(rename = "syncMessage")]
    pub sync_message: Option<JsonSyncMessageV0>,
    /// Example: 1615576442475
    #[serde(skip_serializing_if = "Option::is_none")]
    pub timestamp: Option<i64>,
    #[serde(skip_serializing_if = "Option::is_none")]
    #[serde(rename = "timestampISO")]
    pub timestamp_iso: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    #[serde(rename = "type")]
    pub type_: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub typing: Option<JsonTypingMessageV0>,
    /// Example: "+12024561414"
    #[serde(skip_serializing_if = "Option::is_none")]
    pub username: Option<String>,
    /// Example: "0cc10e61-d64c-4dbc-b51c-334f7dd45a4a"
    #[serde(skip_serializing_if = "Option::is_none")]
    pub uuid: Option<String>,
}

#[derive(Serialize, Deserialize, Clone, Default)]
pub struct JsonMessageRequestResponseMessageV0 {
    #[serde(skip_serializing_if = "Option::is_none")]
    #[serde(rename = "groupId")]
    pub group_id: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub person: Option<JsonAddressV0>,
    #[serde(skip_serializing_if = "Option::is_none")]
    #[serde(rename = "type")]
    pub type_: Option<String>,
}

#[derive(Serialize, Deserialize, Clone, Default)]
pub struct JsonPreviewV0 {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub attachment: Option<JsonAttachmentV0>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub title: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub url: Option<String>,
}

/// A quote is a reply to a previous message. ID is the sent time of the message being replied to
#[derive(Serialize, Deserialize, Clone, Default)]
pub struct JsonQuoteV0 {
    /// list of files attached to the quoted message
    #[serde(skip_serializing_if = "Option::is_none")]
    pub attachments: Option<Vec<JsonQuotedAttachmentV0>>,
    /// the author of the message being quoted
    #[serde(skip_serializing_if = "Option::is_none")]
    pub author: Option<JsonAddressV0>,
    /// the client timestamp of the message being quoted
    /// Example: 1615576442475
    #[serde(skip_serializing_if = "Option::is_none")]
    pub id: Option<i64>,
    /// list of mentions in the quoted message
    #[serde(skip_serializing_if = "Option::is_none")]
    pub mentions: Option<Vec<JsonMentionV0>>,
    /// the body of the message being quoted
    /// Example: "hey ? what's up?"
    #[serde(skip_serializing_if = "Option::is_none")]
    pub text: Option<String>,
}

#[derive(Serialize, Deserialize, Clone, Default)]
pub struct JsonQuotedAttachmentV0 {
    #[serde(skip_serializing_if = "Option::is_none")]
    #[serde(rename = "contentType")]
    pub content_type: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    #[serde(rename = "fileName")]
    pub file_name: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub thumbnail: Option<JsonAttachmentV0>,
}

#[derive(Serialize, Deserialize, Clone, Default)]
pub struct JsonReactionV0 {
    /// the emoji to react with
    /// Example: "?"
    #[serde(skip_serializing_if = "Option::is_none")]
    pub emoji: Option<String>,
    /// set to true to remove the reaction. requires emoji be set to previously reacted emoji
    #[serde(skip_serializing_if = "Option::is_none")]
    pub remove: Option<bool>,
    /// the author of the message being reacted to
    #[serde(skip_serializing_if = "Option::is_none")]
    #[serde(rename = "targetAuthor")]
    pub target_author: Option<JsonAddressV0>,
    /// the client timestamp of the message being reacted to
    /// Example: 1615576442475
    #[serde(skip_serializing_if = "Option::is_none")]
    #[serde(rename = "targetSentTimestamp")]
    pub target_sent_timestamp: Option<i64>,
}

#[derive(Serialize, Deserialize, Clone, Default)]
pub struct JsonReadMessageV0 {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub sender: Option<JsonAddressV0>,
    /// Example: 1615576442475
    #[serde(skip_serializing_if = "Option::is_none")]
    pub timestamp: Option<i64>,
}

#[derive(Serialize, Deserialize, Clone, Default)]
pub struct JsonReceiptMessageV0 {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub timestamps: Option<Vec<i64>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    #[serde(rename = "type")]
    pub type_: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub when: Option<i64>,
}

#[derive(Serialize, Deserialize, Clone, Default)]
pub struct JsonSentTranscriptMessageV0 {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub destination: Option<JsonAddressV0>,
    #[serde(skip_serializing_if = "Option::is_none")]
    #[serde(rename = "expirationStartTimestamp")]
    pub expiration_start_timestamp: Option<i64>,
    #[serde(skip_serializing_if = "Option::is_none")]
    #[serde(rename = "isRecipientUpdate")]
    pub is_recipient_update: Option<bool>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub message: Option<JsonDataMessageV0>,
    /// Example: 1615576442475
    #[serde(skip_serializing_if = "Option::is_none")]
    pub timestamp: Option<i64>,
    #[serde(skip_serializing_if = "Option::is_none")]
    #[serde(rename = "unidentifiedStatus")]
    pub unidentified_status: Option<HashMap<String, String>>,
}

#[derive(Serialize, Deserialize, Clone, Default)]
pub struct JsonStickerV0 {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub attachment: Option<JsonAttachmentV0>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub image: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    #[serde(rename = "packID")]
    pub pack_id: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    #[serde(rename = "packKey")]
    pub pack_key: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    #[serde(rename = "stickerID")]
    pub sticker_id: Option<i32>,
}

#[derive(Serialize, Deserialize, Clone, Default)]
pub struct JsonStickerPackOperationMessageV0 {
    #[serde(skip_serializing_if = "Option::is_none")]
    #[serde(rename = "packID")]
    pub pack_id: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    #[serde(rename = "packKey")]
    pub pack_key: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    #[serde(rename = "type")]
    pub type_: Option<String>,
}

#[derive(Serialize, Deserialize, Clone, Default)]
pub struct JsonSyncMessageV0 {
    #[serde(skip_serializing_if = "Option::is_none")]
    #[serde(rename = "blockedList")]
    pub blocked_list: Option<JsonBlockedListMessageV0>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub configuration: Option<ConfigurationMessageV0>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub contacts: Option<JsonAttachmentV0>,
    #[serde(skip_serializing_if = "Option::is_none")]
    #[serde(rename = "contactsComplete")]
    pub contacts_complete: Option<bool>,
    #[serde(skip_serializing_if = "Option::is_none")]
    #[serde(rename = "fetchType")]
    pub fetch_type: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub groups: Option<JsonAttachmentV0>,
    #[serde(skip_serializing_if = "Option::is_none")]
    #[serde(rename = "messageRequestResponse")]
    pub message_request_response: Option<JsonMessageRequestResponseMessageV0>,
    #[serde(skip_serializing_if = "Option::is_none")]
    #[serde(rename = "readMessages")]
    pub read_messages: Option<Vec<JsonReadMessageV0>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub request: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub sent: Option<JsonSentTranscriptMessageV0>,
    #[serde(skip_serializing_if = "Option::is_none")]
    #[serde(rename = "stickerPackOperations")]
    pub sticker_pack_operations: Option<Vec<JsonStickerPackOperationMessageV0>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub verified: Option<JsonVerifiedMessageV0>,
    #[serde(skip_serializing_if = "Option::is_none")]
    #[serde(rename = "viewOnceOpen")]
    pub view_once_open: Option<JsonViewOnceOpenMessageV0>,
}

#[derive(Serialize, Deserialize, Clone, Default)]
pub struct JsonTypingMessageV0 {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub action: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    #[serde(rename = "groupId")]
    pub group_id: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub timestamp: Option<i64>,
}

#[derive(Serialize, Deserialize, Clone, Default)]
pub struct JsonVerifiedMessageV0 {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub destination: Option<JsonAddressV0>,
    #[serde(skip_serializing_if = "Option::is_none")]
    #[serde(rename = "identityKey")]
    pub identity_key: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub timestamp: Option<i64>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub verified: Option<String>,
}

#[derive(Serialize, Deserialize, Clone, Default)]
pub struct JsonViewOnceOpenMessageV0 {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub sender: Option<JsonAddressV0>,
    /// Example: 1615576442475
    #[serde(skip_serializing_if = "Option::is_none")]
    pub timestamp: Option<i64>,
}

#[derive(Serialize, Deserialize, Clone, Default)]
pub struct NameV0 {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub display: Option<OptionalV0>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub family: Option<OptionalV0>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub given: Option<OptionalV0>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub middle: Option<OptionalV0>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub prefix: Option<OptionalV0>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub suffix: Option<OptionalV0>,
}

#[derive(Serialize, Deserialize, Clone, Default)]
pub struct OfferMessageV0 {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub id: Option<i64>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub opaque: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub sdp: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    #[serde(rename = "type")]
    pub type_: Option<TypeV0>,
}

#[derive(Serialize, Deserialize, Clone, Default)]
pub struct OptionalV0 {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub present: Option<bool>,
}

#[derive(Serialize, Deserialize, Clone, Default)]
pub struct RemoteDeleteV0 {
    #[serde(skip_serializing_if = "Option::is_none")]
    #[serde(rename = "targetSentTimestamp")]
    pub target_sent_timestamp: Option<i64>,
}

#[derive(Serialize, Deserialize, Clone, Default)]
pub struct SharedContactV0 {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub address: Option<OptionalV0>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub avatar: Option<OptionalV0>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub email: Option<OptionalV0>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub name: Option<NameV0>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub organization: Option<OptionalV0>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub phone: Option<OptionalV0>,
}

#[derive(Serialize, Deserialize, Clone, Default)]
pub struct TypeV0;

#[derive(Serialize, Deserialize)]
pub enum SignaldTypes {
    SignaldError(SignaldError),
    NoResponse,
    String(String),
    AcceptInvitationRequestV1(AcceptInvitationRequestV1),
    AccountV1(AccountV1),
    AccountAlreadyVerifiedErrorV1(AccountAlreadyVerifiedErrorV1),
    AccountHasNoKeysErrorV1(AccountHasNoKeysErrorV1),
    AccountListV1(AccountListV1),
    AccountLockedErrorV1(AccountLockedErrorV1),
    AddLinkedDeviceRequestV1(AddLinkedDeviceRequestV1),
    AddServerRequestV1(AddServerRequestV1),
    AllIdentityKeyListV1(AllIdentityKeyListV1),
    AnswerMessageV1(AnswerMessageV1),
    ApproveMembershipRequestV1(ApproveMembershipRequestV1),
    BusyMessageV1(BusyMessageV1),
    CallMessageV1(CallMessageV1),
    CapabilitiesV1(CapabilitiesV1),
    CaptchaRequiredErrorV1(CaptchaRequiredErrorV1),
    ClientMessageWrapperV1(ClientMessageWrapperV1),
    CreateGroupRequestV1(CreateGroupRequestV1),
    DeleteAccountRequestV1(DeleteAccountRequestV1),
    DeviceInfoV1(DeviceInfoV1),
    FingerprintVersionMismatchErrorV1(FingerprintVersionMismatchErrorV1),
    FinishLinkRequestV1(FinishLinkRequestV1),
    GenerateLinkingURIRequestV1(GenerateLinkingURIRequestV1),
    GetAllIdentitiesV1(GetAllIdentitiesV1),
    GetGroupRequestV1(GetGroupRequestV1),
    GetIdentitiesRequestV1(GetIdentitiesRequestV1),
    GetLinkedDevicesRequestV1(GetLinkedDevicesRequestV1),
    GetProfileRequestV1(GetProfileRequestV1),
    GetServersRequestV1(GetServersRequestV1),
    GroupAccessControlV1(GroupAccessControlV1),
    GroupInfoV1(GroupInfoV1),
    GroupLinkInfoRequestV1(GroupLinkInfoRequestV1),
    GroupLinkNotActiveErrorV1(GroupLinkNotActiveErrorV1),
    GroupListV1(GroupListV1),
    GroupMemberV1(GroupMemberV1),
    GroupNotActiveErrorV1(GroupNotActiveErrorV1),
    GroupVerificationErrorV1(GroupVerificationErrorV1),
    HangupMessageV1(HangupMessageV1),
    IceUpdateMessageV1(IceUpdateMessageV1),
    IdentityKeyV1(IdentityKeyV1),
    IdentityKeyListV1(IdentityKeyListV1),
    IncomingMessageV1(IncomingMessageV1),
    InternalErrorV1(InternalErrorV1),
    InvalidAttachmentErrorV1(InvalidAttachmentErrorV1),
    InvalidBase64ErrorV1(InvalidBase64ErrorV1),
    InvalidFingerprintErrorV1(InvalidFingerprintErrorV1),
    InvalidGroupErrorV1(InvalidGroupErrorV1),
    InvalidGroupStateErrorV1(InvalidGroupStateErrorV1),
    InvalidInviteURIErrorV1(InvalidInviteURIErrorV1),
    InvalidProxyErrorV1(InvalidProxyErrorV1),
    InvalidRecipientErrorV1(InvalidRecipientErrorV1),
    InvalidRequestErrorV1(InvalidRequestErrorV1),
    JoinGroupRequestV1(JoinGroupRequestV1),
    JsonAddressV1(JsonAddressV1),
    JsonAttachmentV1(JsonAttachmentV1),
    JsonBlockedListMessageV1(JsonBlockedListMessageV1),
    JsonDataMessageV1(JsonDataMessageV1),
    JsonGroupInfoV1(JsonGroupInfoV1),
    JsonGroupJoinInfoV1(JsonGroupJoinInfoV1),
    JsonGroupV2InfoV1(JsonGroupV2InfoV1),
    JsonMentionV1(JsonMentionV1),
    JsonMessageEnvelopeV1(JsonMessageEnvelopeV1),
    JsonMessageRequestResponseMessageV1(JsonMessageRequestResponseMessageV1),
    JsonPreviewV1(JsonPreviewV1),
    JsonQuoteV1(JsonQuoteV1),
    JsonReactionV1(JsonReactionV1),
    JsonReadMessageV1(JsonReadMessageV1),
    JsonSendMessageResultV1(JsonSendMessageResultV1),
    JsonSentTranscriptMessageV1(JsonSentTranscriptMessageV1),
    JsonSyncMessageV1(JsonSyncMessageV1),
    JsonVerifiedMessageV1(JsonVerifiedMessageV1),
    JsonVersionMessageV1(JsonVersionMessageV1),
    JsonViewOnceOpenMessageV1(JsonViewOnceOpenMessageV1),
    LeaveGroupRequestV1(LeaveGroupRequestV1),
    LinkedDevicesV1(LinkedDevicesV1),
    LinkingURIV1(LinkingURIV1),
    ListAccountsRequestV1(ListAccountsRequestV1),
    ListContactsRequestV1(ListContactsRequestV1),
    ListGroupsRequestV1(ListGroupsRequestV1),
    ListenerStateV1(ListenerStateV1),
    MarkReadRequestV1(MarkReadRequestV1),
    NoKnownUUIDErrorV1(NoKnownUUIDErrorV1),
    NoSendPermissionErrorV1(NoSendPermissionErrorV1),
    NoSuchAccountErrorV1(NoSuchAccountErrorV1),
    NoSuchSessionErrorV1(NoSuchSessionErrorV1),
    OfferMessageV1(OfferMessageV1),
    OwnProfileKeyDoesNotExistErrorV1(OwnProfileKeyDoesNotExistErrorV1),
    PaymentV1(PaymentV1),
    ProfileV1(ProfileV1),
    ProfileListV1(ProfileListV1),
    ProfileUnavailableErrorV1(ProfileUnavailableErrorV1),
    RateLimitErrorV1(RateLimitErrorV1),
    ReactRequestV1(ReactRequestV1),
    ReceiptMessageV1(ReceiptMessageV1),
    RefuseMembershipRequestV1(RefuseMembershipRequestV1),
    RegisterRequestV1(RegisterRequestV1),
    RemoteConfigV1(RemoteConfigV1),
    RemoteConfigListV1(RemoteConfigListV1),
    RemoteConfigRequestV1(RemoteConfigRequestV1),
    RemoteDeleteV1(RemoteDeleteV1),
    RemoteDeleteRequestV1(RemoteDeleteRequestV1),
    RemoveLinkedDeviceRequestV1(RemoveLinkedDeviceRequestV1),
    RemoveServerRequestV1(RemoveServerRequestV1),
    RequestSyncRequestV1(RequestSyncRequestV1),
    ResetSessionRequestV1(ResetSessionRequestV1),
    ResolveAddressRequestV1(ResolveAddressRequestV1),
    SendPaymentRequestV1(SendPaymentRequestV1),
    SendRequestV1(SendRequestV1),
    SendResponseV1(SendResponseV1),
    SendSuccessV1(SendSuccessV1),
    ServerV1(ServerV1),
    ServerCDNV1(ServerCDNV1),
    ServerListV1(ServerListV1),
    ServerNotFoundErrorV1(ServerNotFoundErrorV1),
    SetDeviceNameRequestV1(SetDeviceNameRequestV1),
    SetExpirationRequestV1(SetExpirationRequestV1),
    SetProfileV1(SetProfileV1),
    SubscribeRequestV1(SubscribeRequestV1),
    TrustRequestV1(TrustRequestV1),
    TypingMessageV1(TypingMessageV1),
    TypingRequestV1(TypingRequestV1),
    UnknownGroupErrorV1(UnknownGroupErrorV1),
    UnknownIdentityKeyErrorV1(UnknownIdentityKeyErrorV1),
    UnsubscribeRequestV1(UnsubscribeRequestV1),
    UntrustedIdentityErrorV1(UntrustedIdentityErrorV1),
    UpdateContactRequestV1(UpdateContactRequestV1),
    UpdateGroupRequestV1(UpdateGroupRequestV1),
    UserAlreadyExistsErrorV1(UserAlreadyExistsErrorV1),
    VerifyRequestV1(VerifyRequestV1),
    VersionRequestV1(VersionRequestV1),
    AnswerMessageV0(AnswerMessageV0),
    BusyMessageV0(BusyMessageV0),
    ConfigurationMessageV0(ConfigurationMessageV0),
    GroupAccessControlV0(GroupAccessControlV0),
    GroupMemberV0(GroupMemberV0),
    HangupMessageV0(HangupMessageV0),
    IceUpdateMessageV0(IceUpdateMessageV0),
    JsonAccountV0(JsonAccountV0),
    JsonAccountListV0(JsonAccountListV0),
    JsonAddressV0(JsonAddressV0),
    JsonAttachmentV0(JsonAttachmentV0),
    JsonBlockedListMessageV0(JsonBlockedListMessageV0),
    JsonCallMessageV0(JsonCallMessageV0),
    JsonDataMessageV0(JsonDataMessageV0),
    JsonGroupInfoV0(JsonGroupInfoV0),
    JsonGroupV2InfoV0(JsonGroupV2InfoV0),
    JsonMentionV0(JsonMentionV0),
    JsonMessageEnvelopeV0(JsonMessageEnvelopeV0),
    JsonMessageRequestResponseMessageV0(JsonMessageRequestResponseMessageV0),
    JsonPreviewV0(JsonPreviewV0),
    JsonQuoteV0(JsonQuoteV0),
    JsonQuotedAttachmentV0(JsonQuotedAttachmentV0),
    JsonReactionV0(JsonReactionV0),
    JsonReadMessageV0(JsonReadMessageV0),
    JsonReceiptMessageV0(JsonReceiptMessageV0),
    JsonSentTranscriptMessageV0(JsonSentTranscriptMessageV0),
    JsonStickerV0(JsonStickerV0),
    JsonStickerPackOperationMessageV0(JsonStickerPackOperationMessageV0),
    JsonSyncMessageV0(JsonSyncMessageV0),
    JsonTypingMessageV0(JsonTypingMessageV0),
    JsonVerifiedMessageV0(JsonVerifiedMessageV0),
    JsonViewOnceOpenMessageV0(JsonViewOnceOpenMessageV0),
    NameV0(NameV0),
    OfferMessageV0(OfferMessageV0),
    OptionalV0(OptionalV0),
    RemoteDeleteV0(RemoteDeleteV0),
    SharedContactV0(SharedContactV0),
    TypeV0(TypeV0),
}