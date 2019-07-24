# Step 1 - Basic web project
-----

The `diesel` model:

```rust
// models.rs
use diesel::{r2d2::ConnectionManager, PgConnection};

// type alias to use in multiple places
pub type Pool = r2d2::Pool<ConnectionManager<PgConnection>>;
```

Above file define the Pool alias for a very long name of diesel's type.
Then we setup a new `actix-web` server in `src/main.rs`.
