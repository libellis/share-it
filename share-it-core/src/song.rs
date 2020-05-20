use crate::SoundcloudTrack;

#[derive(Debug, Clone, PartialEq)]
pub struct Song {
    id: u32,
    user_id: u32,
    duration_ms: u32,
    username: String,
    title: String,
    // TODO: add an enum for Sharing
    sharing: String,
    permalink: String,
    permalink_url: String,
    artwork_url: Option<String>,
    stream_url: String,
}

impl Song {
    pub fn new(id: u32,
               user_id: u32,
               duration_ms: u32,
               username: String,
               title: String,
               sharing: String,
               permalink: String,
               permalink_url: String,
               artwork_url: Option<String>,
               stream_url: String) -> Song {
        Song {
            id: id,
            user_id: user_id,
            username: username,
            title: title,
            duration_ms: duration_ms,
            sharing: sharing,
            permalink: permalink,
            permalink_url: permalink_url,
            artwork_url: artwork_url,
            stream_url: stream_url
        }
    }

    pub fn id(&self) -> u32 {
        self.id
    }
}

impl From<SoundcloudTrack> for Song {
    fn from(s_track: SoundcloudTrack) -> Self {
        Song {
            id: s_track.id,
            user_id: s_track.user_id,
            username: s_track.user.username,
            duration_ms: s_track.duration_ms,
            sharing: s_track.sharing,
            title: s_track.title,
            artwork_url: s_track.artwork_url,
            stream_url: s_track.stream_url,
            permalink_url: s_track.permalink_url,
            permalink: s_track.permalink,
        }
    }
}


#[cfg(test)]
mod tests {
    use super::Song;
    use crate::SoundcloudTrack;

    #[test]
    fn mapping_from_soundcloud_track_works() {
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

        
        let s_track: SoundcloudTrack = serde_json::from_str(mock_api_response).unwrap();
        let s = Song::from(s_track.clone());

        assert_eq!(s.id, 13158665);
    }
    
}
