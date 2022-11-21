pub mod comment;
pub mod project;
pub mod section;
pub mod task;

#[macro_export]
macro_rules! endpoint_group {
    () => {
        #[derive(Clone, Copy)]
        pub struct Client<'a> {
            api_client: &'a crate::Client,
        }

        pub fn new<'a>(api_client: &'a crate::Client) -> Client<'a> {
            Client { api_client }
        }
    };
}
