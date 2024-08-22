### Add a dependency to Cargo.toml file
> `$ cargo add library_name`

Or directly add `library_name = "version_number"` under `[dependencies]` in Cargo.toml file

### Update dependencies
> `$ cargo update`

### Shadowing
Shadowing lets us reuse the guess variable name rather than forcing us to create two unique variables, such as guess_str and guess, for example. This feature is often used when you want to **convert a value from one type to another type**.

### Match
Use `match` to handle `Ok` or `Err` of `Result` type.
