```
cargo sqlx prepare -- --lib // Update sqlx-data.json
```

- Needed
- [ ] - Signup and email confirmation
- [ ] - Login (2FA? (Yubikeys?))
- [ ] - Forgot password 
- [ ] - Reset Password
- [ ] - Dispatch event on sign
- [ ] - A macro to parse environment variables directly into structs


Update sqlx-data.json from the base dir
```
SQLX_OFFLINE=true && cargo sqlx prepare --merged 
```