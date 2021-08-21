use crate::socket::{AsyncSocket, MessageCommon};
use crate::SocketError;
use crate::types::*;
use uuid::Uuid;
use crate::errors::SignaldError;

impl<T> SocketWrapper<T>
where T: AsyncSocket,
{
    /// Accept a v2 group invitation. Note that you must have a profile name set to join groups.
    pub async fn accept_invitation(&mut self, msg: AcceptInvitationRequestV1) -> Result<JsonGroupV2InfoV1, SocketError> {
        let id = Uuid::new_v4();
        let msg = MessageCommon::new(
            id.to_simple().to_string(),
            String::from("accept_invitation"),
            "v1".to_owned(),
            msg
        );

        let mut msg = serde_json::to_vec(&msg).unwrap();
        msg.push(b'\n');

        self.socket.write(&msg, &id).await?;
        let response = self.socket.get_response(id).await?;

        match response.get("error") {
            None => Ok(serde_json::from_value::<JsonGroupV2InfoV1>(response).unwrap()),
            Some(_) => Err(SocketError::Signald(serde_json::from_value::<SignaldError>(response).unwrap()))
        }
    }

    /// Link a new device to a local Signal account
    pub async fn add_device(&mut self, msg: AddLinkedDeviceRequestV1) -> Result<(), SocketError> {
        let id = Uuid::new_v4();
        let msg = MessageCommon::new(
            id.to_simple().to_string(),
            String::from("add_device"),
            "v1".to_owned(),
            msg
        );

        let mut msg = serde_json::to_vec(&msg).unwrap();
        msg.push(b'\n');

        self.socket.write(&msg, &id).await?;
        let response = self.socket.get_response(id).await?;

        match response.get("error") {
            None => Ok(()),
            Some(_) => Err(SocketError::Signald(serde_json::from_value::<SignaldError>(response).unwrap()))
        }
    }

    /// add a new server to connect to. Returns the new server's UUID.
    pub async fn add_server(&mut self, msg: AddServerRequestV1) -> Result<String, SocketError> {
        let id = Uuid::new_v4();
        let msg = MessageCommon::new(
            id.to_simple().to_string(),
            String::from("add_server"),
            "v1".to_owned(),
            msg
        );

        let mut msg = serde_json::to_vec(&msg).unwrap();
        msg.push(b'\n');

        self.socket.write(&msg, &id).await?;
        let response = self.socket.get_response(id).await?;

        match response.get("error") {
            None => Ok(serde_json::from_value::<String>(response).unwrap()),
            Some(_) => Err(SocketError::Signald(serde_json::from_value::<SignaldError>(response).unwrap()))
        }
    }

    /// approve a request to join a group
    pub async fn approve_membership(&mut self, msg: ApproveMembershipRequestV1) -> Result<JsonGroupV2InfoV1, SocketError> {
        let id = Uuid::new_v4();
        let msg = MessageCommon::new(
            id.to_simple().to_string(),
            String::from("approve_membership"),
            "v1".to_owned(),
            msg
        );

        let mut msg = serde_json::to_vec(&msg).unwrap();
        msg.push(b'\n');

        self.socket.write(&msg, &id).await?;
        let response = self.socket.get_response(id).await?;

        match response.get("error") {
            None => Ok(serde_json::from_value::<JsonGroupV2InfoV1>(response).unwrap()),
            Some(_) => Err(SocketError::Signald(serde_json::from_value::<SignaldError>(response).unwrap()))
        }
    }

    pub async fn create_group(&mut self, msg: CreateGroupRequestV1) -> Result<JsonGroupV2InfoV1, SocketError> {
        let id = Uuid::new_v4();
        let msg = MessageCommon::new(
            id.to_simple().to_string(),
            String::from("create_group"),
            "v1".to_owned(),
            msg
        );

        let mut msg = serde_json::to_vec(&msg).unwrap();
        msg.push(b'\n');

        self.socket.write(&msg, &id).await?;
        let response = self.socket.get_response(id).await?;

        match response.get("error") {
            None => Ok(serde_json::from_value::<JsonGroupV2InfoV1>(response).unwrap()),
            Some(_) => Err(SocketError::Signald(serde_json::from_value::<SignaldError>(response).unwrap()))
        }
    }

    /// delete all account data signald has on disk, and optionally delete the account from the server as well. Note that this is not "unlink" and will delete the entire account, even from a linked device.
    pub async fn delete_account(&mut self, msg: DeleteAccountRequestV1) -> Result<(), SocketError> {
        let id = Uuid::new_v4();
        let msg = MessageCommon::new(
            id.to_simple().to_string(),
            String::from("delete_account"),
            "v1".to_owned(),
            msg
        );

        let mut msg = serde_json::to_vec(&msg).unwrap();
        msg.push(b'\n');

        self.socket.write(&msg, &id).await?;
        let response = self.socket.get_response(id).await?;

        match response.get("error") {
            None => Ok(()),
            Some(_) => Err(SocketError::Signald(serde_json::from_value::<SignaldError>(response).unwrap()))
        }
    }

    pub async fn delete_server(&mut self, msg: RemoveServerRequestV1) -> Result<(), SocketError> {
        let id = Uuid::new_v4();
        let msg = MessageCommon::new(
            id.to_simple().to_string(),
            String::from("delete_server"),
            "v1".to_owned(),
            msg
        );

        let mut msg = serde_json::to_vec(&msg).unwrap();
        msg.push(b'\n');

        self.socket.write(&msg, &id).await?;
        let response = self.socket.get_response(id).await?;

        match response.get("error") {
            None => Ok(()),
            Some(_) => Err(SocketError::Signald(serde_json::from_value::<SignaldError>(response).unwrap()))
        }
    }

    /// After a linking URI has been requested, finish_link must be called with the session_id provided with the URI. it will return information about the new account once the linking process is completed by the other device.
    pub async fn finish_link(&mut self, msg: FinishLinkRequestV1) -> Result<AccountV1, SocketError> {
        let id = Uuid::new_v4();
        let msg = MessageCommon::new(
            id.to_simple().to_string(),
            String::from("finish_link"),
            "v1".to_owned(),
            msg
        );

        let mut msg = serde_json::to_vec(&msg).unwrap();
        msg.push(b'\n');

        self.socket.write(&msg, &id).await?;
        let response = self.socket.get_response(id).await?;

        match response.get("error") {
            None => Ok(serde_json::from_value::<AccountV1>(response).unwrap()),
            Some(_) => Err(SocketError::Signald(serde_json::from_value::<SignaldError>(response).unwrap()))
        }
    }

    /// Generate a linking URI. Typically this is QR encoded and scanned by the primary device. Submit the returned session_id with a finish_link request.
    pub async fn generate_linking_uri(&mut self, msg: GenerateLinkingURIRequestV1) -> Result<LinkingURIV1, SocketError> {
        let id = Uuid::new_v4();
        let msg = MessageCommon::new(
            id.to_simple().to_string(),
            String::from("generate_linking_uri"),
            "v1".to_owned(),
            msg
        );

        let mut msg = serde_json::to_vec(&msg).unwrap();
        msg.push(b'\n');

        self.socket.write(&msg, &id).await?;
        let response = self.socket.get_response(id).await?;

        match response.get("error") {
            None => Ok(serde_json::from_value::<LinkingURIV1>(response).unwrap()),
            Some(_) => Err(SocketError::Signald(serde_json::from_value::<SignaldError>(response).unwrap()))
        }
    }

    /// get all known identity keys
    pub async fn get_all_identities(&mut self, msg: GetAllIdentitiesV1) -> Result<AllIdentityKeyListV1, SocketError> {
        let id = Uuid::new_v4();
        let msg = MessageCommon::new(
            id.to_simple().to_string(),
            String::from("get_all_identities"),
            "v1".to_owned(),
            msg
        );

        let mut msg = serde_json::to_vec(&msg).unwrap();
        msg.push(b'\n');

        self.socket.write(&msg, &id).await?;
        let response = self.socket.get_response(id).await?;

        match response.get("error") {
            None => Ok(serde_json::from_value::<AllIdentityKeyListV1>(response).unwrap()),
            Some(_) => Err(SocketError::Signald(serde_json::from_value::<SignaldError>(response).unwrap()))
        }
    }

    /// Query the server for the latest state of a known group. If no account in signald is a member of the group (anymore), an error with error_type: 'UnknownGroupException' is returned.
    pub async fn get_group(&mut self, msg: GetGroupRequestV1) -> Result<JsonGroupV2InfoV1, SocketError> {
        let id = Uuid::new_v4();
        let msg = MessageCommon::new(
            id.to_simple().to_string(),
            String::from("get_group"),
            "v1".to_owned(),
            msg
        );

        let mut msg = serde_json::to_vec(&msg).unwrap();
        msg.push(b'\n');

        self.socket.write(&msg, &id).await?;
        let response = self.socket.get_response(id).await?;

        match response.get("error") {
            None => Ok(serde_json::from_value::<JsonGroupV2InfoV1>(response).unwrap()),
            Some(_) => Err(SocketError::Signald(serde_json::from_value::<SignaldError>(response).unwrap()))
        }
    }

    /// Get information about a known keys for a particular address
    pub async fn get_identities(&mut self, msg: GetIdentitiesRequestV1) -> Result<IdentityKeyListV1, SocketError> {
        let id = Uuid::new_v4();
        let msg = MessageCommon::new(
            id.to_simple().to_string(),
            String::from("get_identities"),
            "v1".to_owned(),
            msg
        );

        let mut msg = serde_json::to_vec(&msg).unwrap();
        msg.push(b'\n');

        self.socket.write(&msg, &id).await?;
        let response = self.socket.get_response(id).await?;

        match response.get("error") {
            None => Ok(serde_json::from_value::<IdentityKeyListV1>(response).unwrap()),
            Some(_) => Err(SocketError::Signald(serde_json::from_value::<SignaldError>(response).unwrap()))
        }
    }

    /// list all linked devices on a Signal account
    pub async fn get_linked_devices(&mut self, msg: GetLinkedDevicesRequestV1) -> Result<LinkedDevicesV1, SocketError> {
        let id = Uuid::new_v4();
        let msg = MessageCommon::new(
            id.to_simple().to_string(),
            String::from("get_linked_devices"),
            "v1".to_owned(),
            msg
        );

        let mut msg = serde_json::to_vec(&msg).unwrap();
        msg.push(b'\n');

        self.socket.write(&msg, &id).await?;
        let response = self.socket.get_response(id).await?;

        match response.get("error") {
            None => Ok(serde_json::from_value::<LinkedDevicesV1>(response).unwrap()),
            Some(_) => Err(SocketError::Signald(serde_json::from_value::<SignaldError>(response).unwrap()))
        }
    }

    /// Get all information available about a user
    pub async fn get_profile(&mut self, msg: GetProfileRequestV1) -> Result<ProfileV1, SocketError> {
        let id = Uuid::new_v4();
        let msg = MessageCommon::new(
            id.to_simple().to_string(),
            String::from("get_profile"),
            "v1".to_owned(),
            msg
        );

        let mut msg = serde_json::to_vec(&msg).unwrap();
        msg.push(b'\n');

        self.socket.write(&msg, &id).await?;
        let response = self.socket.get_response(id).await?;

        match response.get("error") {
            None => Ok(serde_json::from_value::<ProfileV1>(response).unwrap()),
            Some(_) => Err(SocketError::Signald(serde_json::from_value::<SignaldError>(response).unwrap()))
        }
    }

    pub async fn get_servers(&mut self, msg: GetServersRequestV1) -> Result<ServerListV1, SocketError> {
        let id = Uuid::new_v4();
        let msg = MessageCommon::new(
            id.to_simple().to_string(),
            String::from("get_servers"),
            "v1".to_owned(),
            msg
        );

        let mut msg = serde_json::to_vec(&msg).unwrap();
        msg.push(b'\n');

        self.socket.write(&msg, &id).await?;
        let response = self.socket.get_response(id).await?;

        match response.get("error") {
            None => Ok(serde_json::from_value::<ServerListV1>(response).unwrap()),
            Some(_) => Err(SocketError::Signald(serde_json::from_value::<SignaldError>(response).unwrap()))
        }
    }

    /// Get information about a group from a signal.group link
    pub async fn group_link_info(&mut self, msg: GroupLinkInfoRequestV1) -> Result<JsonGroupJoinInfoV1, SocketError> {
        let id = Uuid::new_v4();
        let msg = MessageCommon::new(
            id.to_simple().to_string(),
            String::from("group_link_info"),
            "v1".to_owned(),
            msg
        );

        let mut msg = serde_json::to_vec(&msg).unwrap();
        msg.push(b'\n');

        self.socket.write(&msg, &id).await?;
        let response = self.socket.get_response(id).await?;

        match response.get("error") {
            None => Ok(serde_json::from_value::<JsonGroupJoinInfoV1>(response).unwrap()),
            Some(_) => Err(SocketError::Signald(serde_json::from_value::<SignaldError>(response).unwrap()))
        }
    }

    /// Join a group using the a signal.group URL. Note that you must have a profile name set to join groups.
    pub async fn join_group(&mut self, msg: JoinGroupRequestV1) -> Result<JsonGroupJoinInfoV1, SocketError> {
        let id = Uuid::new_v4();
        let msg = MessageCommon::new(
            id.to_simple().to_string(),
            String::from("join_group"),
            "v1".to_owned(),
            msg
        );

        let mut msg = serde_json::to_vec(&msg).unwrap();
        msg.push(b'\n');

        self.socket.write(&msg, &id).await?;
        let response = self.socket.get_response(id).await?;

        match response.get("error") {
            None => Ok(serde_json::from_value::<JsonGroupJoinInfoV1>(response).unwrap()),
            Some(_) => Err(SocketError::Signald(serde_json::from_value::<SignaldError>(response).unwrap()))
        }
    }

    pub async fn leave_group(&mut self, msg: LeaveGroupRequestV1) -> Result<GroupInfoV1, SocketError> {
        let id = Uuid::new_v4();
        let msg = MessageCommon::new(
            id.to_simple().to_string(),
            String::from("leave_group"),
            "v1".to_owned(),
            msg
        );

        let mut msg = serde_json::to_vec(&msg).unwrap();
        msg.push(b'\n');

        self.socket.write(&msg, &id).await?;
        let response = self.socket.get_response(id).await?;

        match response.get("error") {
            None => Ok(serde_json::from_value::<GroupInfoV1>(response).unwrap()),
            Some(_) => Err(SocketError::Signald(serde_json::from_value::<SignaldError>(response).unwrap()))
        }
    }

    /// return all local accounts
    pub async fn list_accounts(&mut self, msg: ListAccountsRequestV1) -> Result<AccountListV1, SocketError> {
        let id = Uuid::new_v4();
        let msg = MessageCommon::new(
            id.to_simple().to_string(),
            String::from("list_accounts"),
            "v1".to_owned(),
            msg
        );

        let mut msg = serde_json::to_vec(&msg).unwrap();
        msg.push(b'\n');

        self.socket.write(&msg, &id).await?;
        let response = self.socket.get_response(id).await?;

        match response.get("error") {
            None => Ok(serde_json::from_value::<AccountListV1>(response).unwrap()),
            Some(_) => Err(SocketError::Signald(serde_json::from_value::<SignaldError>(response).unwrap()))
        }
    }

    pub async fn list_contacts(&mut self, msg: ListContactsRequestV1) -> Result<ProfileListV1, SocketError> {
        let id = Uuid::new_v4();
        let msg = MessageCommon::new(
            id.to_simple().to_string(),
            String::from("list_contacts"),
            "v1".to_owned(),
            msg
        );

        let mut msg = serde_json::to_vec(&msg).unwrap();
        msg.push(b'\n');

        self.socket.write(&msg, &id).await?;
        let response = self.socket.get_response(id).await?;

        match response.get("error") {
            None => Ok(serde_json::from_value::<ProfileListV1>(response).unwrap()),
            Some(_) => Err(SocketError::Signald(serde_json::from_value::<SignaldError>(response).unwrap()))
        }
    }

    pub async fn list_groups(&mut self, msg: ListGroupsRequestV1) -> Result<GroupListV1, SocketError> {
        let id = Uuid::new_v4();
        let msg = MessageCommon::new(
            id.to_simple().to_string(),
            String::from("list_groups"),
            "v1".to_owned(),
            msg
        );

        let mut msg = serde_json::to_vec(&msg).unwrap();
        msg.push(b'\n');

        self.socket.write(&msg, &id).await?;
        let response = self.socket.get_response(id).await?;

        match response.get("error") {
            None => Ok(serde_json::from_value::<GroupListV1>(response).unwrap()),
            Some(_) => Err(SocketError::Signald(serde_json::from_value::<SignaldError>(response).unwrap()))
        }
    }

    pub async fn mark_read(&mut self, msg: MarkReadRequestV1) -> Result<(), SocketError> {
        let id = Uuid::new_v4();
        let msg = MessageCommon::new(
            id.to_simple().to_string(),
            String::from("mark_read"),
            "v1".to_owned(),
            msg
        );

        let mut msg = serde_json::to_vec(&msg).unwrap();
        msg.push(b'\n');

        self.socket.write(&msg, &id).await?;
        let response = self.socket.get_response(id).await?;

        match response.get("error") {
            None => Ok(()),
            Some(_) => Err(SocketError::Signald(serde_json::from_value::<SignaldError>(response).unwrap()))
        }
    }

    /// react to a previous message
    pub async fn react(&mut self, msg: ReactRequestV1) -> Result<SendResponseV1, SocketError> {
        let id = Uuid::new_v4();
        let msg = MessageCommon::new(
            id.to_simple().to_string(),
            String::from("react"),
            "v1".to_owned(),
            msg
        );

        let mut msg = serde_json::to_vec(&msg).unwrap();
        msg.push(b'\n');

        self.socket.write(&msg, &id).await?;
        let response = self.socket.get_response(id).await?;

        match response.get("error") {
            None => Ok(serde_json::from_value::<SendResponseV1>(response).unwrap()),
            Some(_) => Err(SocketError::Signald(serde_json::from_value::<SignaldError>(response).unwrap()))
        }
    }

    /// begin the account registration process by requesting a phone number verification code. when the code is received, submit it with a verify request
    pub async fn register(&mut self, msg: RegisterRequestV1) -> Result<AccountV1, SocketError> {
        let id = Uuid::new_v4();
        let msg = MessageCommon::new(
            id.to_simple().to_string(),
            String::from("register"),
            "v1".to_owned(),
            msg
        );

        let mut msg = serde_json::to_vec(&msg).unwrap();
        msg.push(b'\n');

        self.socket.write(&msg, &id).await?;
        let response = self.socket.get_response(id).await?;

        match response.get("error") {
            None => Ok(serde_json::from_value::<AccountV1>(response).unwrap()),
            Some(_) => Err(SocketError::Signald(serde_json::from_value::<SignaldError>(response).unwrap()))
        }
    }

    /// delete a message previously sent
    pub async fn remote_delete(&mut self, msg: RemoteDeleteRequestV1) -> Result<SendResponseV1, SocketError> {
        let id = Uuid::new_v4();
        let msg = MessageCommon::new(
            id.to_simple().to_string(),
            String::from("remote_delete"),
            "v1".to_owned(),
            msg
        );

        let mut msg = serde_json::to_vec(&msg).unwrap();
        msg.push(b'\n');

        self.socket.write(&msg, &id).await?;
        let response = self.socket.get_response(id).await?;

        match response.get("error") {
            None => Ok(serde_json::from_value::<SendResponseV1>(response).unwrap()),
            Some(_) => Err(SocketError::Signald(serde_json::from_value::<SignaldError>(response).unwrap()))
        }
    }

    /// Remove a linked device from the Signal account. Only allowed when the local device id is 1
    pub async fn remove_linked_device(&mut self, msg: RemoveLinkedDeviceRequestV1) -> Result<(), SocketError> {
        let id = Uuid::new_v4();
        let msg = MessageCommon::new(
            id.to_simple().to_string(),
            String::from("remove_linked_device"),
            "v1".to_owned(),
            msg
        );

        let mut msg = serde_json::to_vec(&msg).unwrap();
        msg.push(b'\n');

        self.socket.write(&msg, &id).await?;
        let response = self.socket.get_response(id).await?;

        match response.get("error") {
            None => Ok(()),
            Some(_) => Err(SocketError::Signald(serde_json::from_value::<SignaldError>(response).unwrap()))
        }
    }

    /// Request other devices on the account send us their group list, syncable config and contact list.
    pub async fn request_sync(&mut self, msg: RequestSyncRequestV1) -> Result<(), SocketError> {
        let id = Uuid::new_v4();
        let msg = MessageCommon::new(
            id.to_simple().to_string(),
            String::from("request_sync"),
            "v1".to_owned(),
            msg
        );

        let mut msg = serde_json::to_vec(&msg).unwrap();
        msg.push(b'\n');

        self.socket.write(&msg, &id).await?;
        let response = self.socket.get_response(id).await?;

        match response.get("error") {
            None => Ok(()),
            Some(_) => Err(SocketError::Signald(serde_json::from_value::<SignaldError>(response).unwrap()))
        }
    }

    /// reset a session with a particular user
    pub async fn reset_session(&mut self, msg: ResetSessionRequestV1) -> Result<SendResponseV1, SocketError> {
        let id = Uuid::new_v4();
        let msg = MessageCommon::new(
            id.to_simple().to_string(),
            String::from("reset_session"),
            "v1".to_owned(),
            msg
        );

        let mut msg = serde_json::to_vec(&msg).unwrap();
        msg.push(b'\n');

        self.socket.write(&msg, &id).await?;
        let response = self.socket.get_response(id).await?;

        match response.get("error") {
            None => Ok(serde_json::from_value::<SendResponseV1>(response).unwrap()),
            Some(_) => Err(SocketError::Signald(serde_json::from_value::<SignaldError>(response).unwrap()))
        }
    }

    /// Resolve a partial JsonAddress with only a number or UUID to one with both. Anywhere that signald accepts a JsonAddress will except a partial, this is a convenience function for client authors, mostly because signald doesn't resolve all the partials it returns
    pub async fn resolve_address(&mut self, msg: ResolveAddressRequestV1) -> Result<JsonAddressV1, SocketError> {
        let id = Uuid::new_v4();
        let msg = MessageCommon::new(
            id.to_simple().to_string(),
            String::from("resolve_address"),
            "v1".to_owned(),
            msg
        );

        let mut msg = serde_json::to_vec(&msg).unwrap();
        msg.push(b'\n');

        self.socket.write(&msg, &id).await?;
        let response = self.socket.get_response(id).await?;

        match response.get("error") {
            None => Ok(serde_json::from_value::<JsonAddressV1>(response).unwrap()),
            Some(_) => Err(SocketError::Signald(serde_json::from_value::<SignaldError>(response).unwrap()))
        }
    }

    pub async fn send(&mut self, msg: SendRequestV1) -> Result<SendResponseV1, SocketError> {
        let id = Uuid::new_v4();
        let msg = MessageCommon::new(
            id.to_simple().to_string(),
            String::from("send"),
            "v1".to_owned(),
            msg
        );

        let mut msg = serde_json::to_vec(&msg).unwrap();
        msg.push(b'\n');

        self.socket.write(&msg, &id).await?;
        let response = self.socket.get_response(id).await?;

        match response.get("error") {
            None => Ok(serde_json::from_value::<SendResponseV1>(response).unwrap()),
            Some(_) => Err(SocketError::Signald(serde_json::from_value::<SignaldError>(response).unwrap()))
        }
    }

    /// send a mobilecoin payment
    pub async fn send_payment(&mut self, msg: SendPaymentRequestV1) -> Result<SendResponseV1, SocketError> {
        let id = Uuid::new_v4();
        let msg = MessageCommon::new(
            id.to_simple().to_string(),
            String::from("send_payment"),
            "v1".to_owned(),
            msg
        );

        let mut msg = serde_json::to_vec(&msg).unwrap();
        msg.push(b'\n');

        self.socket.write(&msg, &id).await?;
        let response = self.socket.get_response(id).await?;

        match response.get("error") {
            None => Ok(serde_json::from_value::<SendResponseV1>(response).unwrap()),
            Some(_) => Err(SocketError::Signald(serde_json::from_value::<SignaldError>(response).unwrap()))
        }
    }

    /// set this device's name. This will show up on the mobile device on the same account under 
    pub async fn set_device_name(&mut self, msg: SetDeviceNameRequestV1) -> Result<(), SocketError> {
        let id = Uuid::new_v4();
        let msg = MessageCommon::new(
            id.to_simple().to_string(),
            String::from("set_device_name"),
            "v1".to_owned(),
            msg
        );

        let mut msg = serde_json::to_vec(&msg).unwrap();
        msg.push(b'\n');

        self.socket.write(&msg, &id).await?;
        let response = self.socket.get_response(id).await?;

        match response.get("error") {
            None => Ok(()),
            Some(_) => Err(SocketError::Signald(serde_json::from_value::<SignaldError>(response).unwrap()))
        }
    }

    /// Set the message expiration timer for a thread. Expiration must be specified in seconds, set to 0 to disable timer
    pub async fn set_expiration(&mut self, msg: SetExpirationRequestV1) -> Result<SendResponseV1, SocketError> {
        let id = Uuid::new_v4();
        let msg = MessageCommon::new(
            id.to_simple().to_string(),
            String::from("set_expiration"),
            "v1".to_owned(),
            msg
        );

        let mut msg = serde_json::to_vec(&msg).unwrap();
        msg.push(b'\n');

        self.socket.write(&msg, &id).await?;
        let response = self.socket.get_response(id).await?;

        match response.get("error") {
            None => Ok(serde_json::from_value::<SendResponseV1>(response).unwrap()),
            Some(_) => Err(SocketError::Signald(serde_json::from_value::<SignaldError>(response).unwrap()))
        }
    }

    pub async fn set_profile(&mut self, msg: SetProfileV1) -> Result<(), SocketError> {
        let id = Uuid::new_v4();
        let msg = MessageCommon::new(
            id.to_simple().to_string(),
            String::from("set_profile"),
            "v1".to_owned(),
            msg
        );

        let mut msg = serde_json::to_vec(&msg).unwrap();
        msg.push(b'\n');

        self.socket.write(&msg, &id).await?;
        let response = self.socket.get_response(id).await?;

        match response.get("error") {
            None => Ok(()),
            Some(_) => Err(SocketError::Signald(serde_json::from_value::<SignaldError>(response).unwrap()))
        }
    }

    /// receive incoming messages. After making a subscribe request, incoming messages will be sent to the client encoded as ClientMessageWrapper. Send an unsubscribe request or disconnect from the socket to stop receiving messages.
    pub async fn subscribe(&mut self, msg: SubscribeRequestV1) -> Result<(), SocketError> {
        let id = Uuid::new_v4();
        let msg = MessageCommon::new(
            id.to_simple().to_string(),
            String::from("subscribe"),
            "v1".to_owned(),
            msg
        );

        let mut msg = serde_json::to_vec(&msg).unwrap();
        msg.push(b'\n');

        self.socket.write(&msg, &id).await?;
        let response = self.socket.get_response(id).await?;

        match response.get("error") {
            None => Ok(()),
            Some(_) => Err(SocketError::Signald(serde_json::from_value::<SignaldError>(response).unwrap()))
        }
    }

    /// Trust another user's safety number using either the QR code data or the safety number text
    pub async fn trust(&mut self, msg: TrustRequestV1) -> Result<(), SocketError> {
        let id = Uuid::new_v4();
        let msg = MessageCommon::new(
            id.to_simple().to_string(),
            String::from("trust"),
            "v1".to_owned(),
            msg
        );

        let mut msg = serde_json::to_vec(&msg).unwrap();
        msg.push(b'\n');

        self.socket.write(&msg, &id).await?;
        let response = self.socket.get_response(id).await?;

        match response.get("error") {
            None => Ok(()),
            Some(_) => Err(SocketError::Signald(serde_json::from_value::<SignaldError>(response).unwrap()))
        }
    }

    /// send a typing started or stopped message
    pub async fn typing(&mut self, msg: TypingRequestV1) -> Result<(), SocketError> {
        let id = Uuid::new_v4();
        let msg = MessageCommon::new(
            id.to_simple().to_string(),
            String::from("typing"),
            "v1".to_owned(),
            msg
        );

        let mut msg = serde_json::to_vec(&msg).unwrap();
        msg.push(b'\n');

        self.socket.write(&msg, &id).await?;
        let response = self.socket.get_response(id).await?;

        match response.get("error") {
            None => Ok(()),
            Some(_) => Err(SocketError::Signald(serde_json::from_value::<SignaldError>(response).unwrap()))
        }
    }

    /// See subscribe for more info
    pub async fn unsubscribe(&mut self, msg: UnsubscribeRequestV1) -> Result<(), SocketError> {
        let id = Uuid::new_v4();
        let msg = MessageCommon::new(
            id.to_simple().to_string(),
            String::from("unsubscribe"),
            "v1".to_owned(),
            msg
        );

        let mut msg = serde_json::to_vec(&msg).unwrap();
        msg.push(b'\n');

        self.socket.write(&msg, &id).await?;
        let response = self.socket.get_response(id).await?;

        match response.get("error") {
            None => Ok(()),
            Some(_) => Err(SocketError::Signald(serde_json::from_value::<SignaldError>(response).unwrap()))
        }
    }

    /// update information about a local contact
    pub async fn update_contact(&mut self, msg: UpdateContactRequestV1) -> Result<ProfileV1, SocketError> {
        let id = Uuid::new_v4();
        let msg = MessageCommon::new(
            id.to_simple().to_string(),
            String::from("update_contact"),
            "v1".to_owned(),
            msg
        );

        let mut msg = serde_json::to_vec(&msg).unwrap();
        msg.push(b'\n');

        self.socket.write(&msg, &id).await?;
        let response = self.socket.get_response(id).await?;

        match response.get("error") {
            None => Ok(serde_json::from_value::<ProfileV1>(response).unwrap()),
            Some(_) => Err(SocketError::Signald(serde_json::from_value::<SignaldError>(response).unwrap()))
        }
    }

    /// modify a group. Note that only one modification action may be preformed at once
    pub async fn update_group(&mut self, msg: UpdateGroupRequestV1) -> Result<GroupInfoV1, SocketError> {
        let id = Uuid::new_v4();
        let msg = MessageCommon::new(
            id.to_simple().to_string(),
            String::from("update_group"),
            "v1".to_owned(),
            msg
        );

        let mut msg = serde_json::to_vec(&msg).unwrap();
        msg.push(b'\n');

        self.socket.write(&msg, &id).await?;
        let response = self.socket.get_response(id).await?;

        match response.get("error") {
            None => Ok(serde_json::from_value::<GroupInfoV1>(response).unwrap()),
            Some(_) => Err(SocketError::Signald(serde_json::from_value::<SignaldError>(response).unwrap()))
        }
    }

    /// verify an account's phone number with a code after registering, completing the account creation process
    pub async fn verify(&mut self, msg: VerifyRequestV1) -> Result<AccountV1, SocketError> {
        let id = Uuid::new_v4();
        let msg = MessageCommon::new(
            id.to_simple().to_string(),
            String::from("verify"),
            "v1".to_owned(),
            msg
        );

        let mut msg = serde_json::to_vec(&msg).unwrap();
        msg.push(b'\n');

        self.socket.write(&msg, &id).await?;
        let response = self.socket.get_response(id).await?;

        match response.get("error") {
            None => Ok(serde_json::from_value::<AccountV1>(response).unwrap()),
            Some(_) => Err(SocketError::Signald(serde_json::from_value::<SignaldError>(response).unwrap()))
        }
    }

    pub async fn version(&mut self, msg: VersionRequestV1) -> Result<JsonVersionMessageV1, SocketError> {
        let id = Uuid::new_v4();
        let msg = MessageCommon::new(
            id.to_simple().to_string(),
            String::from("version"),
            "v1".to_owned(),
            msg
        );

        let mut msg = serde_json::to_vec(&msg).unwrap();
        msg.push(b'\n');

        self.socket.write(&msg, &id).await?;
        let response = self.socket.get_response(id).await?;

        match response.get("error") {
            None => Ok(serde_json::from_value::<JsonVersionMessageV1>(response).unwrap()),
            Some(_) => Err(SocketError::Signald(serde_json::from_value::<SignaldError>(response).unwrap()))
        }
    }
}

pub struct SocketWrapper<T> {
    pub socket: T,
}