pub mod requests;
pub mod teams;
pub mod games;
pub mod scores;
pub mod rounds;
pub mod fields;
pub mod start_times;

pub use requests::{
    request_delete, request_get, request_post, request_put, 
};