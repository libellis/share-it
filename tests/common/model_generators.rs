use share_it::User;

pub fn create_test_user(user_id: u32) -> User {
    User {
        id: user_id,
        username: "test_username".to_string(),
        avatar_url: "test_avatar_url.jpg".to_string(),
        permalink_url: "test_permalink_url.com".to_string()
    }
}