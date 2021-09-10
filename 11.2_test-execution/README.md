# Run tests in Parallel or consecutively

Default: tests will run in parallel.

To run sequentially use `cargo test -- --test-threads=1`.

# Show output regardless of the execution

To show used `println`, please run `cargo test -- --show-output`.

# Run tests by filtering

`cargo test one_hundred`
or
`cargo test add`
or
`cargo test -- --ignored`