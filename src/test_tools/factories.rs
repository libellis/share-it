use crate::{Song, MockUserRepository};
use crate::repositories::abstractions::UserRepository;
use crate::user::User;
use crate::playlist::Playlist;
use crate::waitlist::Waitlist;

pub(crate) struct TestWaitlistSpec {
    pub(crate) user_count: u32,
    pub(crate) playlist_per_user: u32,
    pub(crate) song_per_playlist: u32,

    // which_forgot_active specifies which user (1, 2, 3, 4 etc.) forgot to set their
    // active playlist.
    pub(crate) which_forgot_active: Option<u32>,
}

enum WaitlistOut<'a> {
    WaitlistOwned(Waitlist<MockUserRepository>),
    WaitlistBorrowed(Waitlist<&'a mut MockUserRepository>),
}

#[allow(unused)]
pub(crate) fn new_test_user(user_id: u32) -> User {
    User::new(
        user_id,
        "test_username".to_string(),
        "test_avatar_url".to_string(),
        "permalink_url".to_string(),
    )
}

#[allow(unused)]
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

#[allow(unused)]
pub(crate) fn new_test_playlist(user_id: u32, song_count: u32) -> Playlist {
    let mut playlist = Playlist::new("Test Playlist".to_string());
    for i in 0..song_count {
        let song= new_test_song(i, user_id);
        playlist.add_song(song);
    }
    playlist
}

#[allow(unused)]
pub(crate) fn new_test_waitlist(spec: TestWaitlistSpec) -> Waitlist<MockUserRepository> {
    let mut user_repo = MockUserRepository::new();
    let mut users: Vec<User> = (0..spec.user_count).map(|i| {
        new_test_user(i)
    }).collect();

    for (i, user) in users.iter_mut().enumerate() {
        let playlist = new_test_playlist(user.id(), spec.song_per_playlist);
        if let Some(user_num) = spec.which_forgot_active {
            if user_num == (i + 1) as u32 {
                user.add_playlist(playlist);
                user_repo.insert(&user);
                continue;
            }
        }
        user.set_active_playlist(&playlist.id());
        user.add_playlist(playlist);
        user_repo.insert(&user);
    }
    // Unfortunately this can't happen earlier because the waitlist takes ownership of the repo,
    // so we need to insert users first before passing ownership over.
    // This also requires us to loop one more time to join users to the waitlist.
    // If you can think of a cleaner way to do this, please refactor.
    let mut waitlist = Waitlist::new(user_repo);

    for user in &users {
        waitlist.join((user.id(), user.username()));
    }

    waitlist
}

#[allow(unused)]
pub(crate) fn new_test_waitlist_with_repo(spec: TestWaitlistSpec, repo: &mut MockUserRepository) -> Waitlist<&mut MockUserRepository> {
    let mut users: Vec<User> = (0..spec.user_count).map(|i| {
        new_test_user(i)
    }).collect();

    for (i, user) in users.iter_mut().enumerate() {
        let playlist = new_test_playlist(user.id(), spec.song_per_playlist);
        if let Some(user_num) = spec.which_forgot_active {
            if user_num == (i + 1) as u32 {
                user.add_playlist(playlist);
                repo.insert(&user);
                continue;
            }
        }
        user.set_active_playlist(&playlist.id());
        user.add_playlist(playlist);
        repo.insert(&user);
    }
    // Unfortunately this can't happen earlier because the waitlist takes ownership of the repo,
    // so we need to insert users first before passing ownership over.
    // This also requires us to loop one more time to join users to the waitlist.
    // If you can think of a cleaner way to do this, please refactor.
    let mut waitlist = Waitlist::new(repo);

    for user in &users {
        waitlist.join((user.id(), user.username()));
    }

    waitlist
}
