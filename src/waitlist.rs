use crate::{UserRepository, Song, User};
use std::collections::VecDeque;

// TODO: We probably need also have the username for display purposes in the waitlist.
type UserID = u32;

pub struct Waitlist<T> where
    T: UserRepository
{
    users: T,
    queue: VecDeque<UserID>
}

impl<T> Waitlist<T> where
    T: UserRepository
{
    pub fn new(user_repository: T) -> Waitlist<T> {
        Waitlist {
            users: user_repository,
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
        for (i, u_id) in self.queue.into_iter().enumerate() {
            if user_id == u_id {
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

    // play_next will return a song if the waitlist is non-empty, and the next DJ has
    // at least one song in their active playlist. Otherwise, we skip that DJ, and keep going.
    // If we find no valid songs in active playlists from any of the DJs in the queue, then we
    // return None.
    // If there was a problem communicating with underlying DB, then we return the error type
    // defined by the repository implementation.
    pub fn play_next(&mut self) -> Result<Option<Song>, T::Error> {
        loop {
            // First we remove the current user from the top of the queue.
            self.queue.pop_front();

            // Check if queue is now empty and bail if so.
            if self.queue.is_empty() { return Ok(None); }

            // Now we fetch the full user from the top of the queue, based on the given user_id.
            let u_id = self.queue.front().unwrap();
            let maybe_user = self.users.get(*u_id)?;
            if maybe_user.is_none() {
                // TODO: This is very odd, somehow we got a user_id for a user that doesn't exist in our system.
                // This seems like a very big fuck up and we might want to do something other than skip them,
                // Like a re-fetch from SC to our DB.
                continue
            }
            let user = maybe_user.unwrap();

            // Found a valid user, let's see if they have an active playlist, and if that playlist is non-empty.
            // If so, we have a match and should return the top song for playback.
            // If not, we must remove them from the wait-list and emit a skip event.
            if Some(playlist_id) = user.active_playlist() {
                // TODO: Find playlist with given id in users playlists, and bail if it's empty.
                // Otherwise, get the top song and return it.
            } else {
                continue
            }
        }
    }
}
