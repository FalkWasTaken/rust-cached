# rust-cached

A Procedural macro for caching functions in Rust.

A function like:

    foo(x: T) -> O {
        ...
    }

can be cached by adding the `#[cached]` tag:

    #[cached]
    foo(x: T) -> O {
        ...
    }
