# axum-route

```rust
#[route(get, "/")]
pub async fn index() -> String {
    "Hello World!".into_string()
}
```