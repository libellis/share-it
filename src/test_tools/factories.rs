use crate::{User, Song, Playlist, Waitlist, MockUserRepository, UserRepository};

pub(crate) fn new_test_user(user_id: u32) -> User {
    User::new(
        user_id,
        "test_username".to_string(),
        "test_avatar_url".to_string(),
        "permalink_url".to_string(),
    )
}

pub(crate) fn new_test_song(song_id: u32, user_id: u32) -> Song {
    Song::new(
        song_id,
        user_id,
        111,
        "test username".to_string(),
        "test song".to_string(),
        "public".to_string(),
        "test-song".to_string(),
        "https://www.soundcloud.com/test-user/test-song".to_string(),
        Some("https://www.artwork.com/test-art.jpg".to_string()),
        "https://api.soundcloud.com/tracks/22".to_string()
    )
}

pub(crate) fn new_test_playlist(user_id: u32, first_song_idx: u32) -> Playlist {
    let mut playlist = Playlist::new("Test Playlist".to_string());
    let song1 = new_test_song(first_song_idx, user_id);
    let song2 = new_test_song(first_song_idx + 1, user_id);
    playlist.add_song(song1);
    playlist.add_song(song2);
    playlist
}

// TODO: Take in # of users, songs, etc. to specify how much to generate.
pub(crate) fn new_test_waitlist() -> Waitlist<MockUserRepository> {
    let mut user1 = new_test_user(0);
    let playlist1 = new_test_playlist(user1.id(), 0);
    user1.set_active_playlist(&playlist1.id());
    user1.add_playlist(playlist1);

    let mut user2 = new_test_user(1);
    let playlist2 = new_test_playlist(user2.id(), 2);
    user2.set_active_playlist(&playlist2.id());
    user2.add_playlist(playlist2);

    // This user forgot to set an active playlist.
    let mut user3 = new_test_user(2);
    let playlist3 = new_test_playlist(user3.id(), 4);
    user3.add_playlist(playlist3);

    let mut user4 = new_test_user(3);
    let playlist4 = new_test_playlist(user4.id(), 6);
    user4.set_active_playlist(&playlist4.id());
    user4.add_playlist(playlist4);

    let mut user_repo = MockUserRepository::new();
    user_repo.insert(&user1);
    user_repo.insert(&user2);
    user_repo.insert(&user3);
    user_repo.insert(&user4);

    let mut waitlist = Waitlist::new(user_repo);
    waitlist.join(user1.id());
    waitlist.join(user2.id());
    waitlist.join(user3.id());
    waitlist.join(user4.id());

    waitlist
}