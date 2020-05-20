use crate::Song;
use std::collections::VecDeque;
use std::fmt::Display;
use serde::export::Formatter;
use crate::user::{UserID, Username, User, PlaylistID};
use crate::repositories::abstractions::Repository;
use rusty_ulid::Ulid;

pub type DJ = (UserID, Username);

#[derive(Debug, PartialEq)]
pub(crate) struct Waitlist<T> where
    T: Repository<u32, User>,
{
    id: Ulid,
    users: T,
    current_dj: Option<User>,
    current_playlist: Option<PlaylistID>,
    queue: VecDeque<DJ>
}

impl<T> Waitlist<T> where
    T: Repository<u32, User>,
{
    pub fn new(user_repository: T) -> Waitlist<T> {
        Waitlist {
            id: Ulid::generate(),
            users: user_repository,
            current_dj: None,
            current_playlist: None,
            queue: VecDeque::new(),
        }
    }

    pub fn id(&self) -> Ulid {
        self.id.clone()
    }

    pub fn join(&mut self, dj: DJ) {
        // The queue needs to only be unique user_ids. A user shouldn't be able to have multiple spots
        // in the queue, so we need to first make sure they aren't already in the queue.
        if self.contains_user(dj.0) { return; }

        self.queue.push_back(dj);
    }

    pub fn leave(&mut self, user_id: UserID) {
        for (i, (u_id, _)) in self.queue.iter().enumerate() {
            if user_id == *u_id {
                self.queue.remove(i);
                // we have a unique constraint on the queue, so we should bail
                // because we just found our only match.
                return;
            }
        }
    }

    fn contains_user(&self, user_id: u32) -> bool {
        self.queue.iter().any(|(u_id, _)| { user_id == *u_id })
    }

    pub fn len(&self) -> usize {
        self.queue.len()
    }

    // play_next will return a song if the waitlist is non-empty, and the next DJ has
    // at least one song in their active playlist. Otherwise, we skip that DJ, and keep going.
    // If we find no valid songs in active playlists from any of the DJs in the queue, then we
    // return None.
    // If there was a problem communicating with underlying DB, then we return the error type
    // defined by the repository implementation.
    pub fn play_next(&mut self) -> Result<Option<Song>, T::Error> {
        loop {
            // Base case.
            if self.queue.is_empty() {
                return Ok(None)
            }

            // First we remove the current user from the top of the queue if we have a current_dj.
            // Otherwise they are the first to ever show up so they should be played.
            if let Some(_) = &self.current_dj {
                self.queue.pop_front();
            }

            // Now let's make sure we cycle their playlist for them before moving to the next DJ.
            if let Some(user) = &mut self.current_dj {
                if let Some(current_playlist) = &self.current_playlist {
                    user.cycle_playlist(current_playlist);
                    // Must persist dj back now that we cycled their playlist.
                    // TODO: If we get an underlying database error of some kind, we will
                    // bail here, which means we fail to play next. Is this really what we want?
                    self.users.update(user)?;
                }
            }

            // Check if queue is now empty and bail if so.
            if self.queue.is_empty() { return Ok(None); }

            // Now we fetch the full user from the top of the queue, based on the given user_id.
            let (u_id, _) = self.queue.front().unwrap();
            let maybe_user = self.users.get(u_id)?;
            if maybe_user.is_none() {
                // TODO: This is very odd, somehow we got a user_id for a user that doesn't exist in our system.
                // This seems like a very big mess up and we might want to do something other than skip them,
                // Like a re-fetch from SC to our DB.
                continue
            }
            let user = maybe_user.unwrap();

            // Found a valid user, let's see if they have an active playlist, and if that playlist is non-empty.
            // If so, we have a match and should return the top song for playback.
            // If not, we must remove them from the wait-list and skip them.
            let active_playlist_id = user.active_playlist();
            if active_playlist_id.is_none() {
                // No active playlist set, so let's skip this DJ.
                // TODO: Emit a skip event here.
                continue;
            }
            let maybe_playlist = user.get_playlist(active_playlist_id.unwrap());
            if maybe_playlist.is_none() {
                // Didn't find the active playlist in the users playlists.
                // This is very odd and we should never hit this. Let's skip for now.
                // TODO: Emit a skip event here.
                continue;
            }
            // Found the playlist!
            let playlist = maybe_playlist.unwrap();

            // Let's set the current dj and return the top song.
            // We need to store their playlist id as well in case they change their active playlist
            // during the middle of their turn, so we always cycle the correct playlist next time
            // play_next() gets called.
            self.current_dj = Some(user.clone());
            self.current_playlist = Some(playlist.id());
            return Ok(playlist.top_song());
        }
    }

    pub fn djs(&self) -> &VecDeque<DJ> {
        &self.queue
    }
}

impl<T> Display for Waitlist<T>
    where T: Repository<u32, User> + Clone
{
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        writeln!(f, "Waitlist:")?;
        for (i, (_, username)) in self.queue.iter().enumerate() {
            if i == self.queue.len() - 1 {
                return write!(f, "{}. {}", i+1, username);
            }
            writeln!(f, "{}. {}", i+1, username)?;
        }
        Ok(())
    }
}

// Implementing Clone manually, only for cases where the repository is also clonable.
// All other cases can still get the generic methods implemented above, but would then
// have a non-clonable waitlist.
impl<T> Clone for Waitlist<T>
    where T: Repository<u32, User> + Clone,
{
    fn clone(&self) -> Self {
        Waitlist {
            id: self.id.clone(),
            users: self.users.clone(),
            current_dj: self.current_dj.clone(),
            current_playlist: self.current_playlist.clone(),
            queue: self.queue.clone(),
        }
    }
}

#[cfg(test)]
mod tests {
    use crate::MockUserRepository;
    use crate::repositories::abstractions::Repository;
    use crate::test_tools::factories::{new_test_waitlist_with_repo, TestWaitlistSpec, new_test_waitlist};

    #[test]
    #[allow(unused)]
    fn test_waitlist_play_next() {
        let spec = TestWaitlistSpec {
            user_count: 4,
            playlist_per_user: 1,
            song_per_playlist: 2,
            which_forgot_active: Some(3),
        };
        let mut waitlist = new_test_waitlist(spec);
        let maybe_song = waitlist.play_next().unwrap();

        assert!(maybe_song.is_some());
        let song = maybe_song.unwrap();

        assert_eq!(song.id(), 0);
        // Verify that the we didn't cycle the first DJ as they aren't done playing yet.
        assert_eq!(waitlist.len(), 4);

        waitlist.play_next();
        assert_eq!(waitlist.len(), 3)
    }

    #[test]
    #[allow(unused)]
    fn test_waitlist_skipping_works() {
        let spec = TestWaitlistSpec {
            user_count: 4,
            playlist_per_user: 1,
            song_per_playlist: 2,
            which_forgot_active: Some(3),
        };
        let mut waitlist = new_test_waitlist(spec);
        waitlist.play_next().unwrap();
        waitlist.play_next().unwrap();
        // This person forgot to set an active playlist.
        waitlist.play_next().unwrap();

        // Therefore we expect to have skipped them, and be on the last dj.
        assert_eq!(waitlist.len(), 1);
    }

    #[test]
    #[allow(unused)]
    fn test_waitlist_play_next_saves_users() {
        let mut repo = MockUserRepository::new();
        let spec = TestWaitlistSpec {
            user_count: 4,
            playlist_per_user: 1,
            song_per_playlist: 2,
            which_forgot_active: None,
        };
        let mut waitlist = new_test_waitlist_with_repo(spec, &mut repo);
        waitlist.play_next().unwrap();
        waitlist.play_next().unwrap();

        // We know we have cycled our first user now. Let's get them from the repo and check their playlist length.
        let user = repo.get(&0).unwrap().unwrap();
        assert_eq!(user.playlist_count(), 1);
    }

    #[test]
    #[allow(unused)]
    fn test_waitlist_string_repr() {
        let spec = TestWaitlistSpec {
            user_count: 4,
            playlist_per_user: 1,
            song_per_playlist: 2,
            which_forgot_active: None,
        };
        let mut waitlist = new_test_waitlist(spec);
        let want = "Waitlist:\n1. test_username\n2. test_username\n3. test_username\n4. test_username";
        let got = waitlist.to_string();

        assert_eq!(got, want);
    }

    #[test]
    #[allow(unused)]
    fn empty_playlist_returns_none() {
        let spec = TestWaitlistSpec {
            user_count: 4,
            playlist_per_user: 1,
            song_per_playlist: 2,
            which_forgot_active: None,
        };
        let mut waitlist = new_test_waitlist(spec);
        waitlist.play_next().unwrap();
        waitlist.play_next().unwrap();
        waitlist.play_next().unwrap();
        waitlist.play_next().unwrap();

        // Now we've cycled through all 4 test users. Next one should return None.
        let song = waitlist.play_next().unwrap();
        assert_eq!(song, None);
    }
}
