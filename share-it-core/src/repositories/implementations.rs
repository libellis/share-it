use crate::repositories::abstractions::Repository;
use crate::user::{User, UserID};
lazy_static! {
    static ref MYSQL_POOL: mysql::Pool = {
        let database_url = std::env::var("DATABASE_URL").expect("DATABASE_URL must be set");
        mysql::Pool::new(&database_url).unwrap()
    };
}

pub struct MysqlUsers {
    conn: mysql::PooledConn,
}

impl MysqlUsers {
    pub fn new() -> MysqlSurveyWriteRepository {
        let pool = super::MYSQL_POOL.clone();
        MysqlSurveyWriteRepository {
            conn: pool.get_conn().unwrap(),
        }
    }
}

impl Repository<UserID, User> for MysqlUsers {
    type Error = mysql::Error;

    fn insert(&mut self, user: &User) -> Result<Option<UserID>, Self::Error> {
        match self.conn.query(
            format!(r"INSERT INTO users (id, username, avatar_url, permalink_url)
            VALUES ('{}', '{}', '{}', '{}')", user.id(), user.username(), user.avatar_url(), user.permalink_url())
        ) {
            Ok(_) => Ok(Some(user.id())),
            Err(e) => Err(e),
        }
    }

    fn get(&mut self, key: &UserID) -> Result<Option<User>, Self::Error> {
        let user: Option<User> =
            match self.conn.query(
                format!(r"SELECT u.id, u.username, u.avatar_url, u.permalink_url
                FROM users AS u
                WHERE u.id = '{}'", key)
            ) {
                Ok(mut qr) => {
                    if let Some(row_result) = qr.next() {
                        let row = row_result?;
                        let (id, username, avatar_url, permalink_url) = mysql::from_row::<(UserID, String, String, String)>(row);
                        Some(
                            User::new(id, username, avatar_url, permalink_url)
                        )
                    } else {
                        None
                    }
                },

                // Underlying MySQL error type unrelated to existence of user in db.
                Err(e) => {
                    return Err(e);
                }
            };

        Ok(user)
    }

    fn update(&mut self, entity: &User) -> Result<Option<UserID>, Self::Error> {
        match self.conn.query(
            format!("UPDATE users SET username = '{}', avatar_url = '{}', permalink_url = '{}' WHERE id = '{}'",
            entity.username(), entity.avatar_url(), entity.permalink_url(), entity.id())
        ) {
            Ok(result) => {
                if result.affected_rows() == 0 {
                    return Ok(None);
                }
            },
            Err(e) => {
                return Err(e);
            }
        };

        // Success.  Return the PK back as is.
        Ok(Some(entity.id()))

        // Update playlists here.
    }

    fn remove(&mut self, key: &UserID) -> Result<Option<UserID>, Self::Error> {
        match self.conn.query(
            format!("DELETE FROM users WHERE id = '{}'", key),
        ) {
            Ok(result) => {
                if result.affected_rows() == 0 {
                    return Ok(None);
                }
            },
            Err(e) => {
                return Err(e);
            }
        };

        // Success.  Return the PK back as is.
        Ok(Some(key.clone()))
    }
}