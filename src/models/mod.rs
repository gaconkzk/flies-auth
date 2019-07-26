use diesel::{r2d2::{ConnectionManager}, PgConnection};
/// Simple name for a long long diesel type
///   this is the Pool of ConnectionManager for manage PgConnection.
pub type Pool = r2d2::Pool<ConnectionManager<PgConnection>>;

pub mod user;
pub mod invitation;

// Re export here for simpler `use`
pub use invitation::Invitation;
pub use user::User;
