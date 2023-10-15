# Deploy
Need to set the bind address to `0.0.0.0` instead of `127.0.0.1`  
(TODO: check if it possible to detect we are in a CI environment and set it automatically)
```commandline
flyctl secrets -a rustter-api set API_BIND="0.0.0.0:8070"
```
```commandline
just deploy-api
```

Live at:
https://rustter-api.fly.dev/