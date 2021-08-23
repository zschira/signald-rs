use crate::socket::{AsyncSocket, MessageCommon};
use crate::SocketError;
use crate::types::*;
use uuid::Uuid;
use crate::errors::SignaldError;

impl<T> SocketWrapper<T>
where T: AsyncSocket,
{
    /// Accept a v2 group invitation. Note that you must have a profile name set to join groups.
    pub async fn accept_invitation(&mut self, msg: AcceptInvitationRequestV1, id: Option<Uuid>) -> Result<JsonGroupV2InfoV1, SocketError> {
        let id = match id {
            Some(id) => id,
            None => Uuid::new_v4()
        };
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
    pub async fn add_device(&mut self, msg: AddLinkedDeviceRequestV1, id: Option<Uuid>) -> Result<(), SocketError> {
        let id = match id {
            Some(id) => id,
            None => Uuid::new_v4()
        };
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
    pub async fn add_server(&mut self, msg: AddServerRequestV1, id: Option<Uuid>) -> Result<String, SocketError> {
        let id = match id {
            Some(id) => id,
            None => Uuid::new_v4()
        };
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
    pub async fn approve_membership(&mut self, msg: ApproveMembershipRequestV1, id: Option<Uuid>) -> Result<JsonGroupV2InfoV1, SocketError> {
        let id = match id {
            Some(id) => id,
            None => Uuid::new_v4()
        };
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

    pub async fn create_group(&mut self, msg: CreateGroupRequestV1, id: Option<Uuid>) -> Result<JsonGroupV2InfoV1, SocketError> {
        let id = match id {
            Some(id) => id,
            None => Uuid::new_v4()
        };
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
    pub async fn delete_account(&mut self, msg: DeleteAccountRequestV1, id: Option<Uuid>) -> Result<(), SocketError> {
        let id = match id {
            Some(id) => id,
            None => Uuid::new_v4()
        };
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

    pub async fn delete_server(&mut self, msg: RemoveServerRequestV1, id: Option<Uuid>) -> Result<(), SocketError> {
        let id = match id {
            Some(id) => id,
            None => Uuid::new_v4()
        };
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
    pub async fn finish_link(&mut self, msg: FinishLinkRequestV1, id: Option<Uuid>) -> Result<AccountV1, SocketError> {
        let id = match id {
            Some(id) => id,
            None => Uuid::new_v4()
        };
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
    pub async fn generate_linking_uri(&mut self, msg: GenerateLinkingURIRequestV1, id: Option<Uuid>) -> Result<LinkingURIV1, SocketError> {
        let id = match id {
            Some(id) => id,
            None => Uuid::new_v4()
        };
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
    pub async fn get_all_identities(&mut self, msg: GetAllIdentitiesV1, id: Option<Uuid>) -> Result<AllIdentityKeyListV1, SocketError> {
        let id = match id {
            Some(id) => id,
            None => Uuid::new_v4()
        };
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
    pub async fn get_group(&mut self, msg: GetGroupRequestV1, id: Option<Uuid>) -> Result<JsonGroupV2InfoV1, SocketError> {
        let id = match id {
            Some(id) => id,
            None => Uuid::new_v4()
        };
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
    pub async fn get_identities(&mut self, msg: GetIdentitiesRequestV1, id: Option<Uuid>) -> Result<IdentityKeyListV1, SocketError> {
        let id = match id {
            Some(id) => id,
            None => Uuid::new_v4()
        };
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
    pub async fn get_linked_devices(&mut self, msg: GetLinkedDevicesRequestV1, id: Option<Uuid>) -> Result<LinkedDevicesV1, SocketError> {
        let id = match id {
            Some(id) => id,
            None => Uuid::new_v4()
        };
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
    pub async fn get_profile(&mut self, msg: GetProfileRequestV1, id: Option<Uuid>) -> Result<ProfileV1, SocketError> {
        let id = match id {
            Some(id) => id,
            None => Uuid::new_v4()
        };
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

    pub async fn get_servers(&mut self, msg: GetServersRequestV1, id: Option<Uuid>) -> Result<ServerListV1, SocketError> {
        let id = match id {
            Some(id) => id,
            None => Uuid::new_v4()
        };
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
    pub async fn group_link_info(&mut self, msg: GroupLinkInfoRequestV1, id: Option<Uuid>) -> Result<JsonGroupJoinInfoV1, SocketError> {
        let id = match id {
            Some(id) => id,
            None => Uuid::new_v4()
        };
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
    pub async fn join_group(&mut self, msg: JoinGroupRequestV1, id: Option<Uuid>) -> Result<JsonGroupJoinInfoV1, SocketError> {
        let id = match id {
            Some(id) => id,
            None => Uuid::new_v4()
        };
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

    pub async fn leave_group(&mut self, msg: LeaveGroupRequestV1, id: Option<Uuid>) -> Result<GroupInfoV1, SocketError> {
        let id = match id {
            Some(id) => id,
            None => Uuid::new_v4()
        };
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
    pub async fn list_accounts(&mut self, msg: ListAccountsRequestV1, id: Option<Uuid>) -> Result<AccountListV1, SocketError> {
        let id = match id {
            Some(id) => id,
            None => Uuid::new_v4()
        };
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

    pub async fn list_contacts(&mut self, msg: ListContactsRequestV1, id: Option<Uuid>) -> Result<ProfileListV1, SocketError> {
        let id = match id {
            Some(id) => id,
            None => Uuid::new_v4()
        };
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

    pub async fn list_groups(&mut self, msg: ListGroupsRequestV1, id: Option<Uuid>) -> Result<GroupListV1, SocketError> {
        let id = match id {
            Some(id) => id,
            None => Uuid::new_v4()
        };
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

    pub async fn mark_read(&mut self, msg: MarkReadRequestV1, id: Option<Uuid>) -> Result<(), SocketError> {
        let id = match id {
            Some(id) => id,
            None => Uuid::new_v4()
        };
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
    pub async fn react(&mut self, msg: ReactRequestV1, id: Option<Uuid>) -> Result<SendResponseV1, SocketError> {
        let id = match id {
            Some(id) => id,
            None => Uuid::new_v4()
        };
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
    pub async fn register(&mut self, msg: RegisterRequestV1, id: Option<Uuid>) -> Result<AccountV1, SocketError> {
        let id = match id {
            Some(id) => id,
            None => Uuid::new_v4()
        };
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
    pub async fn remote_delete(&mut self, msg: RemoteDeleteRequestV1, id: Option<Uuid>) -> Result<SendResponseV1, SocketError> {
        let id = match id {
            Some(id) => id,
            None => Uuid::new_v4()
        };
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
    pub async fn remove_linked_device(&mut self, msg: RemoveLinkedDeviceRequestV1, id: Option<Uuid>) -> Result<(), SocketError> {
        let id = match id {
            Some(id) => id,
            None => Uuid::new_v4()
        };
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
    pub async fn request_sync(&mut self, msg: RequestSyncRequestV1, id: Option<Uuid>) -> Result<(), SocketError> {
        let id = match id {
            Some(id) => id,
            None => Uuid::new_v4()
        };
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
    pub async fn reset_session(&mut self, msg: ResetSessionRequestV1, id: Option<Uuid>) -> Result<SendResponseV1, SocketError> {
        let id = match id {
            Some(id) => id,
            None => Uuid::new_v4()
        };
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
    pub async fn resolve_address(&mut self, msg: ResolveAddressRequestV1, id: Option<Uuid>) -> Result<JsonAddressV1, SocketError> {
        let id = match id {
            Some(id) => id,
            None => Uuid::new_v4()
        };
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

    pub async fn send(&mut self, msg: SendRequestV1, id: Option<Uuid>) -> Result<SendResponseV1, SocketError> {
        let id = match id {
            Some(id) => id,
            None => Uuid::new_v4()
        };
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
    pub async fn send_payment(&mut self, msg: SendPaymentRequestV1, id: Option<Uuid>) -> Result<SendResponseV1, SocketError> {
        let id = match id {
            Some(id) => id,
            None => Uuid::new_v4()
        };
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
    pub async fn set_device_name(&mut self, msg: SetDeviceNameRequestV1, id: Option<Uuid>) -> Result<(), SocketError> {
        let id = match id {
            Some(id) => id,
            None => Uuid::new_v4()
        };
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
    pub async fn set_expiration(&mut self, msg: SetExpirationRequestV1, id: Option<Uuid>) -> Result<SendResponseV1, SocketError> {
        let id = match id {
            Some(id) => id,
            None => Uuid::new_v4()
        };
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

    pub async fn set_profile(&mut self, msg: SetProfileV1, id: Option<Uuid>) -> Result<(), SocketError> {
        let id = match id {
            Some(id) => id,
            None => Uuid::new_v4()
        };
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
    pub async fn subscribe(&mut self, msg: SubscribeRequestV1, id: Option<Uuid>) -> Result<(), SocketError> {
        let id = match id {
            Some(id) => id,
            None => Uuid::new_v4()
        };
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
    pub async fn trust(&mut self, msg: TrustRequestV1, id: Option<Uuid>) -> Result<(), SocketError> {
        let id = match id {
            Some(id) => id,
            None => Uuid::new_v4()
        };
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
    pub async fn typing(&mut self, msg: TypingRequestV1, id: Option<Uuid>) -> Result<(), SocketError> {
        let id = match id {
            Some(id) => id,
            None => Uuid::new_v4()
        };
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
    pub async fn unsubscribe(&mut self, msg: UnsubscribeRequestV1, id: Option<Uuid>) -> Result<(), SocketError> {
        let id = match id {
            Some(id) => id,
            None => Uuid::new_v4()
        };
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
    pub async fn update_contact(&mut self, msg: UpdateContactRequestV1, id: Option<Uuid>) -> Result<ProfileV1, SocketError> {
        let id = match id {
            Some(id) => id,
            None => Uuid::new_v4()
        };
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
    pub async fn update_group(&mut self, msg: UpdateGroupRequestV1, id: Option<Uuid>) -> Result<GroupInfoV1, SocketError> {
        let id = match id {
            Some(id) => id,
            None => Uuid::new_v4()
        };
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
    pub async fn verify(&mut self, msg: VerifyRequestV1, id: Option<Uuid>) -> Result<AccountV1, SocketError> {
        let id = match id {
            Some(id) => id,
            None => Uuid::new_v4()
        };
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

    pub async fn version(&mut self, msg: VersionRequestV1, id: Option<Uuid>) -> Result<JsonVersionMessageV1, SocketError> {
        let id = match id {
            Some(id) => id,
            None => Uuid::new_v4()
        };
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

    /// Call api function indirectly from string key
    pub async fn remote_call(&mut self, api_fn: &str, id: Uuid, msg: SignaldTypes) -> Result<SignaldTypes, SocketError> {
        match api_fn {
            "accept_invitation" => {
                if let SignaldTypes::AcceptInvitationRequestV1(msg) = msg {
                    self.accept_invitation(msg, Some(id)).await
                        .map(|response| SignaldTypes::JsonGroupV2InfoV1(response))
                } else {
                    Err(SocketError::General("Incorrect message type"))
                }
            },
            "add_device" => {
                if let SignaldTypes::AddLinkedDeviceRequestV1(msg) = msg {
                    self.add_device(msg, Some(id)).await
                        .map(|_| SignaldTypes::NoResponse)
                } else {
                    Err(SocketError::General("Incorrect message type"))
                }
            },
            "add_server" => {
                if let SignaldTypes::AddServerRequestV1(msg) = msg {
                    self.add_server(msg, Some(id)).await
                        .map(|response| SignaldTypes::String(response))
                } else {
                    Err(SocketError::General("Incorrect message type"))
                }
            },
            "approve_membership" => {
                if let SignaldTypes::ApproveMembershipRequestV1(msg) = msg {
                    self.approve_membership(msg, Some(id)).await
                        .map(|response| SignaldTypes::JsonGroupV2InfoV1(response))
                } else {
                    Err(SocketError::General("Incorrect message type"))
                }
            },
            "create_group" => {
                if let SignaldTypes::CreateGroupRequestV1(msg) = msg {
                    self.create_group(msg, Some(id)).await
                        .map(|response| SignaldTypes::JsonGroupV2InfoV1(response))
                } else {
                    Err(SocketError::General("Incorrect message type"))
                }
            },
            "delete_account" => {
                if let SignaldTypes::DeleteAccountRequestV1(msg) = msg {
                    self.delete_account(msg, Some(id)).await
                        .map(|_| SignaldTypes::NoResponse)
                } else {
                    Err(SocketError::General("Incorrect message type"))
                }
            },
            "delete_server" => {
                if let SignaldTypes::RemoveServerRequestV1(msg) = msg {
                    self.delete_server(msg, Some(id)).await
                        .map(|_| SignaldTypes::NoResponse)
                } else {
                    Err(SocketError::General("Incorrect message type"))
                }
            },
            "finish_link" => {
                if let SignaldTypes::FinishLinkRequestV1(msg) = msg {
                    self.finish_link(msg, Some(id)).await
                        .map(|response| SignaldTypes::AccountV1(response))
                } else {
                    Err(SocketError::General("Incorrect message type"))
                }
            },
            "generate_linking_uri" => {
                if let SignaldTypes::GenerateLinkingURIRequestV1(msg) = msg {
                    self.generate_linking_uri(msg, Some(id)).await
                        .map(|response| SignaldTypes::LinkingURIV1(response))
                } else {
                    Err(SocketError::General("Incorrect message type"))
                }
            },
            "get_all_identities" => {
                if let SignaldTypes::GetAllIdentitiesV1(msg) = msg {
                    self.get_all_identities(msg, Some(id)).await
                        .map(|response| SignaldTypes::AllIdentityKeyListV1(response))
                } else {
                    Err(SocketError::General("Incorrect message type"))
                }
            },
            "get_group" => {
                if let SignaldTypes::GetGroupRequestV1(msg) = msg {
                    self.get_group(msg, Some(id)).await
                        .map(|response| SignaldTypes::JsonGroupV2InfoV1(response))
                } else {
                    Err(SocketError::General("Incorrect message type"))
                }
            },
            "get_identities" => {
                if let SignaldTypes::GetIdentitiesRequestV1(msg) = msg {
                    self.get_identities(msg, Some(id)).await
                        .map(|response| SignaldTypes::IdentityKeyListV1(response))
                } else {
                    Err(SocketError::General("Incorrect message type"))
                }
            },
            "get_linked_devices" => {
                if let SignaldTypes::GetLinkedDevicesRequestV1(msg) = msg {
                    self.get_linked_devices(msg, Some(id)).await
                        .map(|response| SignaldTypes::LinkedDevicesV1(response))
                } else {
                    Err(SocketError::General("Incorrect message type"))
                }
            },
            "get_profile" => {
                if let SignaldTypes::GetProfileRequestV1(msg) = msg {
                    self.get_profile(msg, Some(id)).await
                        .map(|response| SignaldTypes::ProfileV1(response))
                } else {
                    Err(SocketError::General("Incorrect message type"))
                }
            },
            "get_servers" => {
                if let SignaldTypes::GetServersRequestV1(msg) = msg {
                    self.get_servers(msg, Some(id)).await
                        .map(|response| SignaldTypes::ServerListV1(response))
                } else {
                    Err(SocketError::General("Incorrect message type"))
                }
            },
            "group_link_info" => {
                if let SignaldTypes::GroupLinkInfoRequestV1(msg) = msg {
                    self.group_link_info(msg, Some(id)).await
                        .map(|response| SignaldTypes::JsonGroupJoinInfoV1(response))
                } else {
                    Err(SocketError::General("Incorrect message type"))
                }
            },
            "join_group" => {
                if let SignaldTypes::JoinGroupRequestV1(msg) = msg {
                    self.join_group(msg, Some(id)).await
                        .map(|response| SignaldTypes::JsonGroupJoinInfoV1(response))
                } else {
                    Err(SocketError::General("Incorrect message type"))
                }
            },
            "leave_group" => {
                if let SignaldTypes::LeaveGroupRequestV1(msg) = msg {
                    self.leave_group(msg, Some(id)).await
                        .map(|response| SignaldTypes::GroupInfoV1(response))
                } else {
                    Err(SocketError::General("Incorrect message type"))
                }
            },
            "list_accounts" => {
                if let SignaldTypes::ListAccountsRequestV1(msg) = msg {
                    self.list_accounts(msg, Some(id)).await
                        .map(|response| SignaldTypes::AccountListV1(response))
                } else {
                    Err(SocketError::General("Incorrect message type"))
                }
            },
            "list_contacts" => {
                if let SignaldTypes::ListContactsRequestV1(msg) = msg {
                    self.list_contacts(msg, Some(id)).await
                        .map(|response| SignaldTypes::ProfileListV1(response))
                } else {
                    Err(SocketError::General("Incorrect message type"))
                }
            },
            "list_groups" => {
                if let SignaldTypes::ListGroupsRequestV1(msg) = msg {
                    self.list_groups(msg, Some(id)).await
                        .map(|response| SignaldTypes::GroupListV1(response))
                } else {
                    Err(SocketError::General("Incorrect message type"))
                }
            },
            "mark_read" => {
                if let SignaldTypes::MarkReadRequestV1(msg) = msg {
                    self.mark_read(msg, Some(id)).await
                        .map(|_| SignaldTypes::NoResponse)
                } else {
                    Err(SocketError::General("Incorrect message type"))
                }
            },
            "react" => {
                if let SignaldTypes::ReactRequestV1(msg) = msg {
                    self.react(msg, Some(id)).await
                        .map(|response| SignaldTypes::SendResponseV1(response))
                } else {
                    Err(SocketError::General("Incorrect message type"))
                }
            },
            "register" => {
                if let SignaldTypes::RegisterRequestV1(msg) = msg {
                    self.register(msg, Some(id)).await
                        .map(|response| SignaldTypes::AccountV1(response))
                } else {
                    Err(SocketError::General("Incorrect message type"))
                }
            },
            "remote_delete" => {
                if let SignaldTypes::RemoteDeleteRequestV1(msg) = msg {
                    self.remote_delete(msg, Some(id)).await
                        .map(|response| SignaldTypes::SendResponseV1(response))
                } else {
                    Err(SocketError::General("Incorrect message type"))
                }
            },
            "remove_linked_device" => {
                if let SignaldTypes::RemoveLinkedDeviceRequestV1(msg) = msg {
                    self.remove_linked_device(msg, Some(id)).await
                        .map(|_| SignaldTypes::NoResponse)
                } else {
                    Err(SocketError::General("Incorrect message type"))
                }
            },
            "request_sync" => {
                if let SignaldTypes::RequestSyncRequestV1(msg) = msg {
                    self.request_sync(msg, Some(id)).await
                        .map(|_| SignaldTypes::NoResponse)
                } else {
                    Err(SocketError::General("Incorrect message type"))
                }
            },
            "reset_session" => {
                if let SignaldTypes::ResetSessionRequestV1(msg) = msg {
                    self.reset_session(msg, Some(id)).await
                        .map(|response| SignaldTypes::SendResponseV1(response))
                } else {
                    Err(SocketError::General("Incorrect message type"))
                }
            },
            "resolve_address" => {
                if let SignaldTypes::ResolveAddressRequestV1(msg) = msg {
                    self.resolve_address(msg, Some(id)).await
                        .map(|response| SignaldTypes::JsonAddressV1(response))
                } else {
                    Err(SocketError::General("Incorrect message type"))
                }
            },
            "send" => {
                if let SignaldTypes::SendRequestV1(msg) = msg {
                    self.send(msg, Some(id)).await
                        .map(|response| SignaldTypes::SendResponseV1(response))
                } else {
                    Err(SocketError::General("Incorrect message type"))
                }
            },
            "send_payment" => {
                if let SignaldTypes::SendPaymentRequestV1(msg) = msg {
                    self.send_payment(msg, Some(id)).await
                        .map(|response| SignaldTypes::SendResponseV1(response))
                } else {
                    Err(SocketError::General("Incorrect message type"))
                }
            },
            "set_device_name" => {
                if let SignaldTypes::SetDeviceNameRequestV1(msg) = msg {
                    self.set_device_name(msg, Some(id)).await
                        .map(|_| SignaldTypes::NoResponse)
                } else {
                    Err(SocketError::General("Incorrect message type"))
                }
            },
            "set_expiration" => {
                if let SignaldTypes::SetExpirationRequestV1(msg) = msg {
                    self.set_expiration(msg, Some(id)).await
                        .map(|response| SignaldTypes::SendResponseV1(response))
                } else {
                    Err(SocketError::General("Incorrect message type"))
                }
            },
            "set_profile" => {
                if let SignaldTypes::SetProfileV1(msg) = msg {
                    self.set_profile(msg, Some(id)).await
                        .map(|_| SignaldTypes::NoResponse)
                } else {
                    Err(SocketError::General("Incorrect message type"))
                }
            },
            "subscribe" => {
                if let SignaldTypes::SubscribeRequestV1(msg) = msg {
                    self.subscribe(msg, Some(id)).await
                        .map(|_| SignaldTypes::NoResponse)
                } else {
                    Err(SocketError::General("Incorrect message type"))
                }
            },
            "trust" => {
                if let SignaldTypes::TrustRequestV1(msg) = msg {
                    self.trust(msg, Some(id)).await
                        .map(|_| SignaldTypes::NoResponse)
                } else {
                    Err(SocketError::General("Incorrect message type"))
                }
            },
            "typing" => {
                if let SignaldTypes::TypingRequestV1(msg) = msg {
                    self.typing(msg, Some(id)).await
                        .map(|_| SignaldTypes::NoResponse)
                } else {
                    Err(SocketError::General("Incorrect message type"))
                }
            },
            "unsubscribe" => {
                if let SignaldTypes::UnsubscribeRequestV1(msg) = msg {
                    self.unsubscribe(msg, Some(id)).await
                        .map(|_| SignaldTypes::NoResponse)
                } else {
                    Err(SocketError::General("Incorrect message type"))
                }
            },
            "update_contact" => {
                if let SignaldTypes::UpdateContactRequestV1(msg) = msg {
                    self.update_contact(msg, Some(id)).await
                        .map(|response| SignaldTypes::ProfileV1(response))
                } else {
                    Err(SocketError::General("Incorrect message type"))
                }
            },
            "update_group" => {
                if let SignaldTypes::UpdateGroupRequestV1(msg) = msg {
                    self.update_group(msg, Some(id)).await
                        .map(|response| SignaldTypes::GroupInfoV1(response))
                } else {
                    Err(SocketError::General("Incorrect message type"))
                }
            },
            "verify" => {
                if let SignaldTypes::VerifyRequestV1(msg) = msg {
                    self.verify(msg, Some(id)).await
                        .map(|response| SignaldTypes::AccountV1(response))
                } else {
                    Err(SocketError::General("Incorrect message type"))
                }
            },
            "version" => {
                if let SignaldTypes::VersionRequestV1(msg) = msg {
                    self.version(msg, Some(id)).await
                        .map(|response| SignaldTypes::JsonVersionMessageV1(response))
                } else {
                    Err(SocketError::General("Incorrect message type"))
                }
            },
            _ => Err(SocketError::General("Unknown api function"))
        }
    }
}

pub struct SocketWrapper<T> {
    pub socket: T,
}