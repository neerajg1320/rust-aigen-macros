cargo new --lib my_proc_macro
cd my_proc_macro


# Add macros in src/lib.rs

# Add [lib] tag in Cargo.toml

# Add dependences
cargo add syn 
cargo add quote
cargo add proc-macro2

# Add full features for syn
syn={ version="2.0.52", features=["full"] }
