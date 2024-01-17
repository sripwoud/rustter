# Deploy
Need to set the bind address to `0.0.0.0` instead of `127.0.0.1`  
(TODO: check if it is possible to detect we are in a CI environment and set it automatically)
```commandline
flyctl secrets -a rustter-api set API_BIND="0.0.0.0:8070"
```
```commandline
just deploy-api
```

Live at:
https://rustter-api.fly.dev/

# CLI
`just serve-api -- --help`

## Generate private key
`just genkey`