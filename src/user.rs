use crate::{SoundcloudUser, Playlist};
use std::collections::HashMap;
use rusty_ulid::Ulid;

pub(crate) type PlaylistID = Ulid;

#[derive(Debug, Clone, PartialEq)]
pub(crate) struct User {
    id: u32,
    username: String,
    avatar_url: String,
    permalink_url: String,
    active_playlist: Option<PlaylistID>,
    // TODO: Add users playlists here once we have a playlist model.
    playlists: HashMap<PlaylistID, Playlist>
}

impl User {
    pub fn id(&self) -> u32 {
        self.id
    }

    pub fn new(user_id: u32,
               username: String,
               avatar_url: String,
               permalink_url: String) -> User {
        User {
            id: user_id,
            username,
            avatar_url,
            permalink_url,
            active_playlist: None,
            playlists: HashMap::new(),
        }
    }

    pub fn active_playlist(&self) -> Option<&Ulid> {
        if let Some(playlist_id) = &self.active_playlist {
            Some(playlist_id)
        } else {
            None
        }
    }

    pub fn set_active_playlist(&mut self, playlist_id: &PlaylistID) {
        self.active_playlist = Some(playlist_id.clone())
    }

    fn clear_active_playlist(&mut self) {
        self.active_playlist = None;
    }

    pub fn get_playlist(&self, playlist_id: &PlaylistID) -> Option<&Playlist> {
        self.playlists.get(&playlist_id)
    }

    pub fn add_playlist(&mut self, playlist: Playlist) {
        self.playlists.insert(playlist.id(), playlist);
    }

    pub fn remove_playlist(&mut self, playlist_id: &PlaylistID) {
        // We need to first ensure this playlist is not the active playlist.
        if let Some(p_id) = &self.active_playlist {
            if p_id == playlist_id { self.clear_active_playlist() }
        }

        self.playlists.remove(playlist_id);
    }

    pub fn cycle_playlist(&mut self, playlist_id: &PlaylistID) {
        if let Some(playlist) = self.playlists.get_mut(playlist_id) {
            playlist.cycle_playlist()
        }
    }

    pub fn playlist_count(&self) -> usize {
        self.playlists.len()
    }
}

impl From<SoundcloudUser> for User {
    fn from(s_user: SoundcloudUser) -> Self {
        User {
            id: s_user.id,
            username: s_user.username,
            avatar_url: s_user.avatar_url,
            permalink_url: s_user.permalink_url,
            active_playlist: None,
            playlists: HashMap::new(),
        }
    }
}

#[cfg(test)]
mod tests {
    use super::User;
    use crate::SoundcloudUser;

    #[test]
    fn mapping_from_soundcloud_user_works() {
        let mock_api_response = r#"
        {
          "id": 3207,
          "permalink": "jwagener",
          "username": "Johannes Wagener",
          "uri": "https://api.soundcloud.com/users/3207",
          "permalink_url": "https://soundcloud.com/jwagener",
          "avatar_url": "https://i1.sndcdn.com/avatars-000001552142-pbw8yd-large.jpg?142a848",
          "country": "Germany",
          "full_name": "Johannes Wagener",
          "city": "Berlin",
          "description": "<b>Hacker at SoundCloud</b>\r\n\r\nSome of my recent Hacks:\r\n\r\nsoundiverse.com \r\nbrowse recordings with the FiRe app by artwork\r\n\r\ntopbillin.com \r\nfind people to follow on SoundCloud\r\n\r\nchatter.fm \r\nget your account hooked up with a voicebox\r\n\r\nrecbutton.com \r\nrecord straight to your soundcloud account",
          "discogs_name": null,
          "myspace_name": null,
          "website": "http://johannes.wagener.cc",
          "website_title": "johannes.wagener.cc",
          "online": true,
          "track_count": 12,
          "playlist_count": 1,
          "followers_count": 416,
          "followings_count": 174,
          "public_favorites_count": 26,
          "plan": "Pro Plus",
          "private_tracks_count": 63,
          "private_playlists_count": 3,
          "primary_email_confirmed": true
        }"#;

        let s_user: SoundcloudUser = serde_json::from_str(mock_api_response).unwrap();
        let u = User::from(s_user.clone());

        assert_eq!(u.id, s_user.id);
    }
}
