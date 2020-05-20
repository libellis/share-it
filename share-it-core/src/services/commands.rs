use crate::user::UserID;
use rusty_ulid::Ulid;

pub struct CreateChatroomCmd {
    pub creating_user: UserID,
    pub chatroom_name: String,
}

pub struct JoinChatroomCmd {
    pub chatroom_id: Ulid,
    pub user_id: u32,
}

pub struct LeaveChatroomCmd {
    pub chatroom_id: Ulid,
    pub user_id: u32,
}

pub struct LoginCmd {
    // TODO: Fill in necessary info to log in a user.
}

pub struct JoinWaitlistCmd {
    pub chatroom_id: Ulid,
    pub user_id: u32,
}

pub struct LeaveWaitlistCmd {
    pub chatroom_id: Ulid,
    pub user_id: u32,
}

pub struct ListWaistlistDJs {
    pub chatroom_id: Ulid,
}

pub struct PlayNextCmd {
    // TODO: Fill in necessary info to play the next song, if you are the moderator.
}

pub struct UploadSongCmd {
    // TODO: Fill in necessary info to upload a song.
}