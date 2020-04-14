mod common;
use common::*;
use share_it::UserRepository;

#[test]
#[allow(unused)]
fn test_add_user() {
    let user_id = 0;
    let test_user = common::create_test_user(user_id);
    let mut user_repo = MockUserRepository::new();
    user_repo.insert(&test_user);
    let success_result = user_repo.get(user_id).unwrap();

    assert_eq!(&success_result.unwrap().username, &test_user.username)
}

#[test]
#[allow(unused)]
fn test_cant_add_duplicate() {
    let user_id = 0;
    let test_user = common::create_test_user(user_id);
    let mut user_repo = MockUserRepository::new();
    let returned_entity = user_repo.insert(&test_user).unwrap();
    assert!(returned_entity.is_some());

    let success_result = user_repo.get(user_id).unwrap();
    assert_eq!(&success_result.unwrap().username, &test_user.username);

    let failure_result = user_repo.insert(&test_user).unwrap();
    assert!(failure_result.is_none());
}

// TODO: Test this once we add playlist support. Check that we can update a users playlists and it persists.
// #[test]
// #[allow(unused)]
// fn test_update_user() {
//     let user_id = 0;
//     let mut test_user = common::create_test_user(user_id);
//     let mut user_repo = MockUserRepository::new();
//     let returned_entity = user_repo.insert(&test_user).unwrap();
//     assert!(returned_entity.is_some());
//
//     // TODO: Update users playlists here.
//     user_repo.update(&test_user).unwrap();
//     // check that we get back Some() which implies updating worked.
//     assert!(returned_entity.is_some());
//
//     let updated_user = user_repo.get(user_id).unwrap();
//     assert_eq!(updated_user.unwrap().first_name(), &updated_name);
// }

#[test]
#[allow(unused)]
fn test_remove_user() {
    let user_id = 0;
    let test_user = common::create_test_user(user_id);
    let mut user_repo = MockUserRepository::new();
    user_repo.insert(&test_user);

    // we first check that user is in repo
    assert!(user_repo.contains(user_id).unwrap());

    user_repo.remove(user_id);
    assert!(!user_repo.contains(user_id).unwrap())
}
