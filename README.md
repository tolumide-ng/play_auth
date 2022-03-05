```
cargo sqlx prepare -- --lib // Update sqlx-data.json
```

- Needed
- [ ] - Signup and email confirmation
- [ ] - Login (2FA? (Yubikeys?))
- [ ] - Forgot password 
- [ ] - Reset Password
- [ ] - Dispatch event on sign


Update sqlx-data.json from the base dir
```
SQLX_OFFLINE=true && cargo sqlx prepare --merged 
```