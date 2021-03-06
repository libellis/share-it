use crate::waitlist::{Waitlist, DJ};
use crate::repositories::abstractions::Repository;
use crate::user::{UserID, User, Username};
use rusty_ulid::Ulid;
use std::collections::VecDeque;
use crate::Song;

#[derive(Clone, PartialEq)]
pub struct ChatUser(pub UserID, pub Username);

pub(crate) struct Chatroom<T> where
    T: Repository<u32, User>,
{
    id: Ulid,
    name: String,
    moderator: UserID,
    waitlist: Waitlist<T>,
    current_users: Vec<ChatUser>,
}

impl<T> Chatroom<T> where
    T: Repository<u32, User>,
{
    pub fn new(user_repo: T, creating_user: UserID, chatroom_name: String) -> Chatroom<T> {
        Chatroom {
            id: Ulid::generate(),
            name: chatroom_name,
            moderator: creating_user,
            waitlist: Waitlist::new(user_repo),
            current_users: Vec::new(),
        }
    }

    pub fn moderator(&self) -> UserID {
        self.moderator
    }

    pub fn change_moderator(&mut self, new_moderator: UserID) {
        self.moderator = new_moderator;
    }

    pub fn join(&mut self, user: ChatUser) -> bool {
        // chatroom users list needs to be unique.
        if self.current_users.iter().any(|u| {
            u.0 == user.0
        }) {
            return false;
        }

        self.current_users.push(user);
        true
    }

    pub fn leave(&mut self, user_id: u32) -> bool {
        let pre_len = self.current_users.len();
        self.current_users.retain(|u| u.0 != user_id);
        self.current_users.len() != pre_len
    }

    pub fn join_waitlist(&mut self, user_id: u32) -> bool {
        // First make sure user is in chatroom.
        let dj_result : Vec<&ChatUser> = self.current_users.iter()
            .filter(|u| {
                u.0 == user_id
        }).collect();

        if dj_result.len() != 1 {
            // TODO: This should probably be an error.
            return false;
        }

        let dj = (dj_result[0].0, dj_result[0].1.clone());

        return self.waitlist.join(dj);
    }

    pub fn leave_waitlist(&mut self, user_id: u32) -> bool {
        let pre_len = self.waitlist.len();
        self.waitlist.leave(user_id);
        self.waitlist.len() != pre_len()
    }

    pub fn len(&self) -> usize {
        self.current_users.len()
    }

    pub fn waitlist_djs(&self) -> &VecDeque<DJ> {
        self.waitlist.djs()
    }

    pub fn play_next(&mut self) -> Result<Option<Song>, T::Error>{
        // TODO: We probably need to actually hand the song over for streaming somehow here.
        self.waitlist.play_next()
    }
}

impl<T> Clone for Chatroom<T> where
    T: Repository<u32, User> + Clone,
{
    fn clone(&self) -> Self {
        Chatroom {
            id: self.id.clone(),
            name: self.name.clone(),
            moderator: self.moderator.clone(),
            waitlist: self.waitlist.clone(),
            current_users: self.current_users.clone(),
        }
    }
}

#[cfg(test)]
mod tests {
    use crate::test_tools::factories::{TestChatroomSpec, new_test_chatroom, new_test_user};
    use std::collections::VecDeque;
    use crate::chatroom::ChatUser;

    #[test]
    #[allow(unused)]
    fn test_chatroom_leave() {
        let spec = TestChatroomSpec {
            chatroom_user_count: 4,
            playlist_per_user: 1,
            song_per_playlist: 2,
            // test user 1, and test user 3 joined the waitlist.
            which_joined_waitlist: vec![1, 3],
            moderator_user: 1,
            which_forgot_active: None,
        };
        let mut chatroom = new_test_chatroom(spec);
        assert_eq!(chatroom.len(), 4);

        chatroom.leave(0);
        assert_eq!(chatroom.len(), 3);
    }

    #[test]
    #[allow(unused)]
    fn test_chatroom_unique_users() {
        let spec = TestChatroomSpec {
            chatroom_user_count: 4,
            playlist_per_user: 1,
            song_per_playlist: 2,
            // test user 1, and test user 3 joined the waitlist.
            which_joined_waitlist: vec![1, 3],
            moderator_user: 1,
            which_forgot_active: None,
        };
        let mut chatroom = new_test_chatroom(spec);
        assert_eq!(chatroom.len(), 4);

        let test_user = new_test_user(0);
        chatroom.join(ChatUser(test_user.id(), test_user.username()));

        assert_eq!(chatroom.len(), 4)
    }

    #[test]
    #[allow(unused)]
    fn test_chatroom_moderator() {
        let spec = TestChatroomSpec {
            chatroom_user_count: 4,
            playlist_per_user: 1,
            song_per_playlist: 2,
            // test user 1, and test user 3 joined the waitlist.
            which_joined_waitlist: vec![1, 3],
            moderator_user: 1,
            which_forgot_active: None,
        };
        let mut chatroom = new_test_chatroom(spec);
        assert_eq!(chatroom.moderator(), 0);

        chatroom.change_moderator(2);
        assert_eq!(chatroom.moderator(), 2)
    }

    #[test]
    #[allow(unused)]
    fn test_waitlist_working() {
        let spec = TestChatroomSpec {
            chatroom_user_count: 4,
            playlist_per_user: 1,
            song_per_playlist: 2,
            // test user 1, and test user 3 joined the waitlist.
            which_joined_waitlist: vec![1, 3],
            moderator_user: 1,
            which_forgot_active: None,
        };
        let mut chatroom = new_test_chatroom(spec);
        let want = VecDeque::from(vec![
            (0, "test_username".to_string()),
            (2, "test_username".to_string())
        ]);
        let got = chatroom.waitlist_djs();

        assert_eq!(got, &want)
    }
}
