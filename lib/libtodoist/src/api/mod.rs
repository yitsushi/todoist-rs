pub mod comment;
pub mod project;
pub mod section;
pub mod task;

#[macro_export]
macro_rules! endpoint_group {
    () => {
        #[derive(Clone, Copy)]
        pub struct Client<'a> {
            api_client: &'a $crate::Client,
        }

        pub fn new(api_client: &$crate::Client) -> Client<'_> {
            Client { api_client }
        }
    };
}
