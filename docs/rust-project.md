### Creating Project
```
cargo new your_porject_name
```

### resolve missing autocomplete,diagnostics
```
cargo check
```

### Cargo Check Packages
```
cargo tree
```

### Add necessary packages
- add tokio standard packages
```
cargo add tokio --features full
```
### sqlx installation
- add following under [dependencies]
```
sqlx = { version = "0.7", features = [
    "postgres",               # or "mysql", "sqlite"
    "runtime-tokio-rustls",   # or "runtime-async-std-native-tls"
    "macros",                 # enables compile-time query checking
    "offline"                 # optional: for offline builds
], default-features = false }
```