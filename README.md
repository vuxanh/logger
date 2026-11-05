# Docs

Logger crate. Features:

- Log to file
- Thread-safe logging
- Log levels
- Log rotation

## Example

```rust
fn main() {
    let config = LogServiceConfig::new()
        .path("./logs")
        .file_name("app.log")
        .max_buffer_size(4)
        .max_file_size(1000000);
    let service = LogService::new(&config);
    let tx = service.stream();

    service.register();
}
```
