use crate::song::Song;
use rusty_ulid::Ulid;
use std::collections::VecDeque;

pub(crate) struct Playlist {
    id: Ulid,
    name: String,
    songs: VecDeque<Song>
}

impl Playlist {
    pub fn new(name: String) -> Playlist {
        Playlist {
            id: Ulid::generate(),
            name: name,
            songs: VecDeque::new(),
        }
    }

    pub fn add_song(&mut self, song: Song) {
        if self.contains_song(&song) { return }
        self.songs.push_back(song);
    }

    pub fn remove_song(&mut self, song_id: u32) {
        for (i, song) in self.songs.iter().enumerate() {
            if song.id() == song_id {
                self.songs.remove(i);
                return;
            }
        }
    }

    pub fn top_song(&self) -> Option<Song> {
        if let Some(song) = self.songs.front() {
            Some(song.clone())
        } else {
            None
        }
    }

    pub fn cycle_playlist(&mut self) {
        if let Some(first) = self.songs.pop_front() {
            self.songs.push_back(first);
        }
    }

    pub fn len(&self) -> usize {
        self.songs.len()
    }

    fn contains_song(&self, song: &Song) -> bool {
        self.songs.iter().any(|s| {
            song.id() == s.id()
        })
    }
}

#[cfg(test)]
mod tests {
    use super::Playlist;
    use crate::song::Song;

    #[test]
    fn test_playlist() {
        let mut playlist: Playlist = Playlist::new("Test Playlist".to_string());
        let song1: Song = Song::new(
            11,
            22,
            3333,
            "test user".to_string(),
            "test song".to_string(),
            "public".to_string(),
            "test-song".to_string(),
            "https://www.soundcloud.com/test-user/test-song".to_string(),
            Some("https://www.artwork.com/test-art".to_string()),
            "https://api.soundcloud.com/tracks/22".to_string()
        );

        playlist.add_song(song1);
        let top_song = playlist.top_song();

        assert_eq!(playlist.name, "Test Playlist");
        assert!(playlist.len() == 1);
    }

    #[test]
    fn test_top_song() {
        let mut playlist: Playlist = Playlist::new("Test Playlist".to_string());
        let song1: Song = Song::new(
            1,
            11,
            111,
            "test user".to_string(),
            "test song".to_string(),
            "public".to_string(),
            "test-song".to_string(),
            "https://www.soundcloud.com/test-user/test-song".to_string(),
            Some("https://www.artwork.com/test-art".to_string()),
            "https://api.soundcloud.com/tracks/22".to_string()
        );
        let song2: Song = Song::new(
            2,
            22,
            222,
            "test user".to_string(),
            "test song".to_string(),
            "public".to_string(),
            "test-song".to_string(),
            "https://www.soundcloud.com/test-user/test-song".to_string(),
            Some("https://www.artwork.com/test-art".to_string()),
            "https://api.soundcloud.com/tracks/22".to_string()
        );

        playlist.add_song(song1.clone());
        playlist.add_song(song2.clone());

        let top = playlist.top_song().unwrap();
        assert_eq!(top, song1);
    }

    #[test]
    fn test_cycle_playlist() {
        let mut playlist: Playlist = Playlist::new("Test Playlist".to_string());
        let song1: Song = Song::new(
            1,
            11,
            111,
            "test user".to_string(),
            "test song".to_string(),
            "public".to_string(),
            "test-song".to_string(),
            "https://www.soundcloud.com/test-user/test-song".to_string(),
            Some("https://www.artwork.com/test-art".to_string()),
            "https://api.soundcloud.com/tracks/22".to_string()
        );
        let song2: Song = Song::new(
            2,
            22,
            222,
            "test user".to_string(),
            "test song".to_string(),
            "public".to_string(),
            "test-song".to_string(),
            "https://www.soundcloud.com/test-user/test-song".to_string(),
            Some("https://www.artwork.com/test-art".to_string()),
            "https://api.soundcloud.com/tracks/22".to_string()
        );

        playlist.add_song(song1.clone());
        playlist.add_song(song2.clone());
        
        playlist.cycle_playlist();

        assert_eq!(playlist.top_song(), Some(song2));
    }

    #[test]
    fn test_add_duplicate_song() {
        let mut playlist: Playlist = Playlist::new("Test Playlist".to_string());
        let song1: Song = Song::new(
            1,
            11,
            111,
            "test user".to_string(),
            "test song".to_string(),
            "public".to_string(),
            "test-song".to_string(),
            "https://www.soundcloud.com/test-user/test-song".to_string(),
            Some("https://www.artwork.com/test-art".to_string()),
            "https://api.soundcloud.com/tracks/22".to_string()
        );
        
        playlist.add_song(song1.clone());
        playlist.add_song(song1.clone());

        assert_eq!(playlist.len(), 1);
    }

    #[test]
    fn test_add_remove_song() {
        let mut playlist: Playlist = Playlist::new("Test Playlist".to_string());
        let song1: Song = Song::new(
            1,
            11,
            111,
            "test user".to_string(),
            "test song".to_string(),
            "public".to_string(),
            "test-song".to_string(),
            "https://www.soundcloud.com/test-user/test-song".to_string(),
            Some("https://www.artwork.com/test-art".to_string()),
            "https://api.soundcloud.com/tracks/22".to_string()
        );
        
        playlist.add_song(song1.clone());
        assert_eq!(playlist.len(), 1);

        playlist.remove_song(song1.id());
        assert_eq!(playlist.len(), 0);
    }
}
