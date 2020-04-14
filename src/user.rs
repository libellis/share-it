use crate::SoundcloudUser;
use domain_patterns::models::Entity;

#[derive(Clone, Entity)]
pub struct User {
    id: u32,
    username: String,
    avatar_url: String,
    permalink_url: String,
    // TODO: Add users playlists here once we have a playlist model.
}

impl User {
    pub fn id(&self) -> u32 {
        self.id
    }

    pub fn new(id: u32, username: String, avatar_url: String, permalink_url: String) -> User {
        User {
            id,
            username,
            avatar_url,
            permalink_url,
        }
    }
}

impl From<SoundcloudUser> for User {
    fn from(s_user: SoundcloudUser) -> Self {
        User {
            id: s_user.id,
            username: s_user.username,
            avatar_url: s_user.avatar_url,
            permalink_url: s_user.permalink_url,
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
