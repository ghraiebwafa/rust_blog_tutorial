pub mod header;
pub mod post_details;
pub mod post_form;
pub mod post_list;
 pub mod post_update;
mod login;
mod register;

pub use header::Header;
pub use post_details::PostDetails;
pub use post_form::PostForm;
pub use post_list::PostList;
pub use post_update::PostUpdate;
pub use login::LoginComponent;
pub use register::RegisterComponent;