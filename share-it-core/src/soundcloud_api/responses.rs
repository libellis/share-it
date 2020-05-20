#[derive(Deserialize, Clone)]
pub struct SoundcloudUser {
    pub id: u32,
    pub username: String,
    pub uri: String,
    pub permalink_url: String,
    pub avatar_url: String,
}

#[derive(Deserialize, Clone)]
pub struct SoundcloudTrack {
    pub id: u32,
    pub user_id: u32,
    #[serde(rename = "duration")]
    pub duration_ms: u32,
    pub sharing: String,
    pub title: String,
    pub permalink: String,
    pub permalink_url: String,
    pub artwork_url: Option<String>,
    pub stream_url: String,
    pub user: SoundcloudUser,
}

#[cfg(test)]
mod tests {
    use super::SoundcloudUser;
    use super::SoundcloudTrack;

    #[test]
    fn mapping_from_user_api_response_works() {
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

        let u: SoundcloudUser = serde_json::from_str(mock_api_response).unwrap();

        assert_eq!(u.id, 3207);
    }

    #[test]
    fn mapping_from_track_api_response_works() {
        let mock_api_response = r#"
        {
            "id": 13158665,
            "created_at": "2011/04/06 15:37:43 +0000",
            "user_id": 3699101,
            "duration": 18109,
            "commentable": true,
            "state": "finished",
            "sharing": "public",
            "tag_list": "soundcloud:source=iphone-record",
            "permalink": "munching-at-tiannas-house",
            "description": null,
            "streamable": true,
            "downloadable": true,
            "genre": null,
            "release": null,
            "purchase_url": null,
            "label_id": null,
            "label_name": null,
            "isrc": null,
            "video_url": null,
            "track_type": "recording",
            "key_signature": null,
            "bpm": null,
            "title": "Munching at Tiannas house",
            "release_year": null,
            "release_month": null,
            "release_day": null,
            "original_format": "m4a",
            "original_content_size": 10211857,
            "license": "all-rights-reserved",
            "uri": "https://api.soundcloud.com/tracks/13158665",
            "permalink_url": "https://soundcloud.com/user2835985/munching-at-tiannas-house",
            "artwork_url": null,
            "waveform_url": "https://w1.sndcdn.com/fxguEjG4ax6B_m.png",
            "user": {
              "id": 3699101,
              "permalink": "user2835985",
              "username": "user2835985",
              "uri": "https://api.soundcloud.com/users/3699101",
              "permalink_url": "https://soundcloud.com/user2835985",
              "avatar_url": "https://a1.sndcdn.com/images/default_avatar_large.png?142a848"
            },
            "stream_url": "https://api.soundcloud.com/tracks/13158665/stream",
            "download_url": "https://api.soundcloud.com/tracks/13158665/download",
            "playback_count": 0,
            "download_count": 0,
            "favoritings_count": 0,
            "comment_count": 0,
            "attachments_uri": "https://api.soundcloud.com/tracks/13158665/attachments"
          }"#;

        let t: SoundcloudTrack = serde_json::from_str(mock_api_response).unwrap();

        assert_eq!(t.id, 13158665);
    }
}
