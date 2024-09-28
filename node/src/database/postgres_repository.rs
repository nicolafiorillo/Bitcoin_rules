use std::env;

use core::block::header::Header;
use core::std_lib::std_result::StdResult;
use diesel::{pg::PgConnection, Connection, QueryResult, RunQueryDsl};

use super::{models::NewHeader, repository::Repository, schema::headers};

pub struct PostgresRepository {
    pub conn: PgConnection,
}

impl Repository for PostgresRepository {
    type Output = PostgresRepository;

    fn connect() -> StdResult<Self::Output> {
        let database_url = env::var("DATABASE_URL").expect("DATABASE_URL must be set");
        let conn =
            PgConnection::establish(&database_url).unwrap_or_else(|_| panic!("Error connecting to {}", database_url));

        Ok(PostgresRepository { conn })
    }

    async fn create_headers(&mut self, br_headers: &[Header]) -> QueryResult<usize> {
        let new_headers: Vec<NewHeader> = br_headers.iter().map(|h| h.into()).collect();
        diesel::insert_into(headers::table)
            .values(new_headers)
            .execute(&mut self.conn)
    }
}
