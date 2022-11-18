pub mod project;
pub mod task;
pub mod comment;

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
