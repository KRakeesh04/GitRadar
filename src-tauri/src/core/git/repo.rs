use crate::db::repositories::get_all_repositories;
use crate::models::Repository;
use rusqlite::Connection;

pub fn get_repos_from_db(conn: &Connection) -> Vec<Repository> {
    let repos = get_all_repositories(conn).unwrap_or_default();
    repos
}
