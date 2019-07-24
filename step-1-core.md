# Step 1 - Basic web project
-----

### The `diesel` model:

```rust
// models.rs
use diesel::{r2d2::ConnectionManager, PgConnection};

// type alias to use in multiple places
pub type Pool = r2d2::Pool<ConnectionManager<PgConnection>>;
```

Above file define the Pool alias for a very long name of diesel's type.
Then we setup a new `actix-web` server in `src/main.rs`.

Setting up diesel quite easy. First, install `diesel_cli` using cargo.
Then run `diesel setup`, this will create our database if didn't exist
and setup a migration directory etc.

Create some SQL, we will insert migrations by run `diesel migration generate users`
and similar with invitations. Open up.sql, down.sql files in migrations folders and
and sql respectively.

Command `diesel migration run` will create the tables in the DB and a file src/schema.rs.
This is all we should know about diesel for now. For more details we can
read diesel documentation.

So we already have tables in DB, now we create a representation of user and invitation
in rust (models module). We created `user.rs` and `invitation.rs` and implement helper
function that can quickly create new instance of User and Invitation struct.

### Our own Error response type
Convert all other error types into our own ServiceError for management. For doing that,
we impl the `From` trait from `std::convert` of other error types for our ServiceError.

### Invitation Handler
We created the invitation handlers for handling posting new invitation to server.
Sending invitation email was ignored and I'll back to it in future.

### Helper in utils
hashing password, and verifying it will be separate into utils (I'll move them around later)
We also add SlimUser that is a simple struct that will hide all sensitivy information
in User (like password), impl trait From<User> for SlimUser will help us create new instance
from original User easily.

### Registering User
