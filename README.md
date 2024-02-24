# pasty-rs

A low level API wrapper for [pasty](https://github.com/lus/pasty).

```
cargo add pasty-rs
```

Because this SDK currently only allows async requests, you might want
to install an async runtime like [tokio](https://crates.io/crates/tokio), [async-std](https://crates.io/crates/async-std) or [smol](https://crates.io/crates/smol).

## Example Usage

The following example uses tokio as async runtime.

```
cargo add tokio --features all
```

```rust
use pasty_rs::client::UnauthenticatedClient;

#[tokio::main]
async fn main() -> anyhow::Result<()> {
    // Create an API client for unauthenticated requests.
    let client = UnauthenticatedClient::new("https://pasty.lus.pm")?;

    // Create a paste.
    let paste = client.create_paste("hello pasty!", None).await?;
    dbg!(&paste);

    // Store the modification_token of that paste.
    let modification_token = paste.modification_token;

    // Get a paste by id (the one we've created).
    let paste = client.paste(&paste.paste.id).await?;
    dbg!(&paste);

    // Transform the unauthenticated client into an authenticated client
    // using the modification_token of the created paste.
    let client = client.authenticate(&modification_token);

    // Update the previously created paste with the authenticated client.
    client
        .update_paste(&paste.id, "new hello world", None)
        .await?;

    // Retireve the updated posts content.
    let paste = client.inner().paste(&paste.id).await?;
    dbg!(&paste);

    // Delete the created post.
    client.delete_paste(&paste.id).await?;

    Ok(())
}
```

## Limitations

Because this is a somewhat quick and dirty implementation I need for another project, this crate currently has some limitations.

- Currently only supports async requests.
- Currently does not support the [report paste](https://github.com/lus/pasty/blob/master/API.md#unsecured-report-a-paste) endpoint.

## License

This crate is licensed under the [MIT License](LICENSE).

pasty is licensed under the [MIT License](https://github.com/lus/pasty/blob/master/LICENSE), (c) 2020 Lukas SP.