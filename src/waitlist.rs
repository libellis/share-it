use crate::{UserRepository, Song, User, PlaylistID};
use std::collections::VecDeque;

// TODO: We probably need also have the username for display purposes in the waitlist.
type UserID = u32;

pub(crate) struct Waitlist<T> where
    T: UserRepository
{
    users: T,
    current_dj: Option<User>,
    current_playlist: Option<PlaylistID>,
    queue: VecDeque<UserID>
}

impl<T> Waitlist<T> where
    T: UserRepository
{
    pub fn new(user_repository: T) -> Waitlist<T> {
        Waitlist {
            users: user_repository,
            current_dj: None,
            current_playlist: None,
            queue: VecDeque::new(),
        }
    }

    pub fn join(&mut self, user_id: UserID) {
        // The queue needs to only be unique user_ids. A user shouldn't be able to have multiple spots
        // in the queue, so we need to first make sure they aren't already in the queue.
        if self.contains_user(user_id) { return; }

        self.queue.push_back(user_id);
    }

    pub fn leave(&mut self, user_id: UserID) {
        for (i, u_id) in self.queue.iter().enumerate() {
            if user_id == *u_id {
                self.queue.remove(i);
                // we have a unique constraint on the queue, so we should bail
                // because we just found our only match.
                return;
            }
        }
    }

    fn contains_user(&self, user_id: UserID) -> bool {
        self.queue.iter().any(|u_id| { user_id == *u_id })
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
            // First we remove the current user from the top of the queue if we have a current_dj.
            // Otherwise they are the first to ever show up so they should be played.
            if let Some(dj) = &self.current_dj {
                self.queue.pop_front();
            }

            // Now let's make sure we cycle their playlist for them before moving to the next DJ.
            if let Some(dj) = &mut self.current_dj {
                if let Some(current_playlist) = &self.current_playlist {
                    dj.cycle_playlist(current_playlist);
                    // Must persist dj back now that we cycled their playlist.
                    self.users.update(dj);
                }
            }

            // Check if queue is now empty and bail if so.
            if self.queue.is_empty() { return Ok(None); }

            // Now we fetch the full user from the top of the queue, based on the given user_id.
            let u_id = self.queue.front().unwrap();
            let maybe_user = self.users.get(*u_id)?;
            if maybe_user.is_none() {
                // TODO: This is very odd, somehow we got a user_id for a user that doesn't exist in our system.
                // This seems like a very big mess up and we might want to do something other than skip them,
                // Like a re-fetch from SC to our DB.
                continue
            }
            let user = maybe_user.unwrap();

            // Found a valid user, let's see if they have an active playlist, and if that playlist is non-empty.
            // If so, we have a match and should return the top song for playback.
            // If not, we must remove them from the wait-list and emit a skip event.
            let active_playlist_id = user.active_playlist();
            if active_playlist_id.is_none() {
                // No active playlist set, so let's skip this DJ.
                continue;
            }
            let maybe_playlist = user.get_playlist(active_playlist_id.unwrap());
            if maybe_playlist.is_none() {
                // Didn't find the active playlist in the users playlists.
                // This is very odd and we should never hit this. Let's skip for now.
                continue;
            }
            // Found the playlist!
            let playlist = maybe_playlist.unwrap();

            // Let's set the current dj and return the top song.
            // We need to store their playlist id as well in case they change their active playlist
            // During the middle of their turn, so we always cycle the correct playlist next time
            // play_next() gets called.
            self.current_dj = Some(user.clone());
            self.current_playlist = Some(playlist.id());
            return Ok(playlist.top_song());
        }
    }
}

#[cfg(test)]
mod tests {
    use crate::{new_test_user, MockUserRepository, UserRepository, new_test_waitlist};

    #[allow(unused)]
    fn test_waitlist_play_next() {
        let mut waitlist = new_test_waitlist();
        let maybe_song = waitlist.play_next().unwrap();

        assert!(maybe_song.is_some());
        let song = maybe_song.unwrap();

        assert_eq!(song.id(), 0);
        // Verify that the we didn't cycle the first DJ as they aren't done playing yet.
        assert_eq!(waitlist.len(), 4);

        waitlist.play_next();
        assert_eq!(waitlist.len(), 3)
    }

    #[allow(unused)]
    fn test_waitlist_skipping_works() {
        let mut waitlist = new_test_waitlist();
        waitlist.play_next().unwrap();
        waitlist.play_next().unwrap();
        // This person forgot to set an active playlist.
        waitlist.play_next().unwrap();

        // Therefore we expect to have skipped them, and be on the last dj.
        assert_eq!(waitlist.len(), 1);
    }
}
