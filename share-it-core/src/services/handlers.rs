use crate::repositories::abstractions::Repository;
use crate::user::User;
use crate::chatroom::{Chatroom, ChatUser};
use rusty_ulid::Ulid;
use crate::services::abstractions::Handles;
use crate::services::commands::{CreateChatroomCmd, JoinChatroomCmd, LeaveChatroomCmd, JoinWaitlistCmd, LeaveWaitlistCmd, ListWaistlistDJs};
use std::collections::VecDeque;
use crate::waitlist::DJ;


// PACKAGE TODOS: Handlers should only return serialized types.

// ChatroomHandler is a Handler that handles all chatroom related commands.
pub struct ChatroomHandler<T, U> where
    T: Repository<Ulid, Chatroom<U>>,
    U: Repository<u32, User> + Clone,
{
    chatrooms: T,
    users: U,
}

impl<T, U> ChatroomHandler<T, U> where
    T: Repository<Ulid, Chatroom<U>>,
    U: Repository<u32, User> + Clone,
{
    pub fn new(chatroom_repo: T, user_repo: U) -> ChatroomHandler<T, U> {
        ChatroomHandler {
            chatrooms: chatroom_repo,
            users: user_repo,
        }
    }
}

impl<T, U> Handles<CreateChatroomCmd> for ChatroomHandler<T, U> where
    T: Repository<Ulid, Chatroom<U>>,
    U: Repository<u32, User> + Clone,
{
    type Result = Result<Option<Ulid>, T::Error>;

    fn handle(&mut self, cmd: CreateChatroomCmd) -> Self::Result {
        let mut new_chatroom = Chatroom::new(self.users.clone(), cmd.creating_user, cmd.chatroom_name);
        self.chatrooms.insert(&new_chatroom)
    }
}

impl<T, U> Handles<JoinChatroomCmd> for ChatroomHandler<T, U> where
    T: Repository<Ulid, Chatroom<U>>,
    U: Repository<u32, User> + Clone,
{
    type Result = Result<Option<()>, T::Error>;

    fn handle(&mut self, cmd: JoinChatroomCmd) -> Self::Result {
        let maybe_chatroom = self.chatrooms.get(&cmd.chatroom_id)?;
        if let None = maybe_chatroom {
            return Ok(None);
        }
        let mut chatroom = maybe_chatroom.unwrap();

        // TODO: This will return a U::Error, so we need an error tree. After establishing, remove unwrap.
        let maybe_requesting_user = self.users.get(&cmd.user_id).unwrap();
        if let None = maybe_requesting_user {
            return Ok(None)
        }
        let User{ id, username, ..} = maybe_requesting_user.unwrap();

        let joined = chatroom.join(ChatUser(id, username));
        if !joined {
            // No work was necessary, so no need to persist.
            return Ok(None)
        }

        let result = self.chatrooms.update(&chatroom)?;
        if result.is_none() {
            return Ok(None);
        }

        Ok(Some(()))
    }
}

impl<T, U> Handles<LeaveChatroomCmd> for ChatroomHandler<T, U> where
    T: Repository<Ulid, Chatroom<U>>,
    U: Repository<u32, User> + Clone,
{
    type Result = Result<Option<()>, T::Error>;

    fn handle(&mut self, cmd: LeaveChatroomCmd) -> Self::Result {
        let maybe_chatroom = self.chatrooms.get(&cmd.chatroom_id)?;
        if let None = maybe_chatroom {
            return Ok(None);
        }
        let mut chatroom = maybe_chatroom.unwrap();

        let left = chatroom.leave(cmd.user_id);

        if !left {
            // No work was necessary, so no need to persist.
            return Ok(None)
        }

        let result = self.chatrooms.update(&chatroom)?;
        if result.is_none() {
            return Ok(None);
        }

        Ok(Some(()))
    }
}

impl<T, U> Handles<JoinWaitlistCmd> for ChatroomHandler<T, U> where
    T: Repository<Ulid, Chatroom<U>>,
    U: Repository<u32, User> + Clone,
{
    type Result = Result<Option<()>, T::Error>;

    fn handle(&mut self, cmd: JoinWaitlistCmd) -> Self::Result {
        let maybe_chatroom = self.chatrooms.get(&cmd.chatroom_id)?;
        if let None = maybe_chatroom {
            return Ok(None);
        }
        let mut chatroom = maybe_chatroom.unwrap();

        // TODO: This will return a U::Error, so we need an error tree. After establishing, remove unwrap.
        let maybe_requesting_user = self.users.get(&cmd.user_id).unwrap();
        if let None = maybe_requesting_user {
            return Ok(None)
        }
        let User{id, ..} = maybe_requesting_user.unwrap();

        let joined = chatroom.join_waitlist(id);
        if !joined {
            return Ok(None)
        }

        let result = self.chatrooms.update(&chatroom)?;
        if result.is_none() {
            return Ok(None);
        }

        Ok(Some(()))
    }
}

impl<T, U> Handles<LeaveWaitlistCmd> for ChatroomHandler<T, U> where
    T: Repository<Ulid, Chatroom<U>>,
    U: Repository<u32, User> + Clone,
{
    type Result = Result<Option<()>, T::Error>;

    fn handle(&mut self, cmd: LeaveWaitlistCmd) -> Self::Result {
        let maybe_chatroom = self.chatrooms.get(&cmd.chatroom_id)?;
        if let None = maybe_chatroom {
            return Ok(None);
        }
        let mut chatroom = maybe_chatroom.unwrap();

        // TODO: This will return a U::Error, so we need an error tree. After establishing, remove unwrap.
        let maybe_requesting_user = self.users.get(&cmd.user_id).unwrap();
        if let None = maybe_requesting_user {
            return Ok(None)
        }
        let User{id, ..} = maybe_requesting_user.unwrap();

        let left = chatroom.leave_waitlist(id);
        if !left {
            return Ok(None)
        }

        let result = self.chatrooms.update(&chatroom)?;
        if result.is_none() {
            return Ok(None);
        }

        Ok(Some(()))
    }
}

impl<T, U> Handles<ListWaistlistDJs> for ChatroomHandler<T, U> where
    T: Repository<Ulid, Chatroom<U>>,
    U: Repository<u32, User> + Clone,
{
    type Result = Result<Option<VecDeque<DJ>>, T::Error>;

    fn handle(&mut self, cmd: ListWaistlistDJs) -> Self::Result {
        let maybe_chatroom = self.chatrooms.get(&cmd.chatroom_id)?;
        if let None = maybe_chatroom {
            return Ok(None);
        }
        let chatroom = maybe_chatroom.unwrap();

        Ok(Some(chatroom.waitlist_djs().clone()))
    }
}
