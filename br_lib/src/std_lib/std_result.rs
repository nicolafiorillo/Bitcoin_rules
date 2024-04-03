pub type StdResult<T> = Result<T, Box<dyn std::error::Error>>;

pub struct SendStdResult<T>(pub StdResult<T>);

unsafe impl<T> Send for SendStdResult<T> where T: Send {}
