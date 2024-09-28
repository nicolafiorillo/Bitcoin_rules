use core::{block::header::Header, std_lib::std_result::StdResult};
use diesel::QueryResult;

pub trait Repository {
    type Output: Repository;

    fn connect() -> StdResult<Self::Output>;
    async fn create_headers(&mut self, headers: &[Header]) -> QueryResult<usize>;
}
