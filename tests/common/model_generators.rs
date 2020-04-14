use share_it::User;

pub fn create_test_user(user_id: u32) -> User {
    User::new(
        user_id,
        "test_username".to_string(),
        "test_avatar_url.jpg".to_string(),
        "test_permalink_url.com".to_string()
    )
}