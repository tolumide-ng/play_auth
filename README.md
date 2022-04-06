```
cargo sqlx prepare -- --lib // Update sqlx-data.json
```

- Needed
- [x] - Signup and email confirmation
- [ ] - Login (2FA? (Yubikeys?))
- [x] - Forgot password 
- [x] - Reset Password
- [ ] - Dispatch event on sign


```
SQLX_OFFLINE=true && cargo sqlx prepare --merged 
```

### 1. Run the application in a development environment 
```
docker compose up # to startup the application
docker compose down # to shutdown the application
docker compose down -v # to shutdown the application and it's volumes
```
OR 
```
make play_dev # to start the application
```



### 2. Run the application in a production environment
```
docker compose up -f docker-compose.yml -f docker-compose.prod.yml up -d # to start the application
docker compose down -f docker-compose.yml -f docker-compose.prod.yml down
```
