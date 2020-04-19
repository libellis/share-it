use crate::waitlist::{Waitlist, DJ};
use crate::repositories::abstractions::Repository;
use crate::user::{UserID, User, Username};
use rusty_ulid::Ulid;
use std::collections::VecDeque;

#[derive(Clone)]
pub struct ChatUser(pub UserID, pub Username);

pub(crate) struct Chatroom<T> where
    T: Repository<u32, User>,
{
    id: Ulid,
    moderator: UserID,
    waitlist: Waitlist<T>,
    current_users: Vec<ChatUser>,
}

impl<T> Chatroom<T> where
    T: Repository<u32, User>,
{
    pub fn new(user_repo: T, creating_user: UserID) -> Chatroom<T> {
        Chatroom {
            id: Ulid::generate(),
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

    pub fn enter(&mut self, user: &ChatUser) {
        self.current_users.push(user.clone());
    }

    pub fn leave(&mut self, user_id: u32) {
        let maybe_user_idx = self.current_users.iter().position(|u| {
            u.0 == user_id
        });

        if let Some(user_idx) = maybe_user_idx {
            self.current_users.remove(user_idx);
        }
    }

    pub fn join_waitlist(&mut self, user_id: u32) {
        // First make sure user is in chatroom.
        let dj_result : Vec<&ChatUser> = self.current_users.iter()
            .filter(|u| {
                u.0 == user_id
        }).collect();

        if dj_result.len() != 1 {
            // TODO: This should probably be an error.
            return;
        }

        let dj = (dj_result[0].0, dj_result[0].1.clone());

        self.waitlist.join(dj);
    }

    pub fn leave_waitlist(&mut self, user_id: u32) {
        self.waitlist.leave(user_id);
    }

    pub fn len(&self) -> usize {
        self.current_users.len()
    }

    pub fn djs(&self) -> &VecDeque<DJ> {
        self.waitlist.djs()
    }
}

impl<T> Clone for Chatroom<T> where
    T: Repository<u32, User> + Clone,
{
    fn clone(&self) -> Self {
        Chatroom {
            id: self.id.clone(),
            moderator: self.moderator.clone(),
            waitlist: self.waitlist.clone(),
            current_users: self.current_users.clone(),
        }
    }
}

#[cfg(test)]
mod tests {
    use crate::test_tools::factories::{TestChatroomSpec, new_test_chatroom};
    use std::collections::VecDeque;

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
        let got = chatroom.djs();

        assert_eq!(got, &want)
    }
}
