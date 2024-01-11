#### How to building tools for multiple applications with cargo [cargo-floki]
(You can have multiple outputs of multiple applications at the same time by executing a command.)

Building a tool for managing the build process of multiple applications can be quite complex,
and writing build tools in Rust involves creating a program that automates various tasks related to building,
testing, and packaging your Rust projects.

#### How to install cargo :
```bash
$ cargo install --path .
```

##### Usage:
```bash
$  cargo floki --help
Usage: cargo-floki [OPTIONS] <COMMAND>

Commands:
  init    Adds a default floki.toml file to current directory
  build   Compile the client and server
  clean   Remove the target directories (in app, client and server)
  test    Run the cargo tests for app, client and server
  update  Run the cargo update for app, client and server
  run     Run app
  doc     Docs
  help    Print this message or the help of the given subcommand(s)

Options:
  -r, --release          Build artifacts in release mode, with optimizations
  -v, --verbose...       Verbosity (none: errors & warnings, -v: verbose, --vv: very verbose, --vvv: output everything)
  -c, --config <CONFIG>  Path to configuration file (defaults to './floki.toml')
  -h, --help             Print help

```
Below is a basic outline of how you might approach building such a tool:

#### Project Structure:
Create a standardized project structure to ensure consistency across different applications. This can simplify the build process by having a predictable layout.

We have 2 project for my example `app` and `client`:

```bash
$ tree
.
├── app (Project 1)
│   ├── Cargo.toml
│   └── src
│       └── lib.rs  
└── client (Project 2)
    ├── Cargo.toml
    └── src
        └── main.rs
```
#### Configuration:
Allow for configuration options so users can customize the build process for their specific needs.
Consider using configuration files, environment variables, or a configuration management system.

I define project path `app` and `client` and generate `floki.toml` :

```bash
$ cargo floki init
$ cat floki.toml 
[floki]
# # Optional. Defaults to 'app'
 main_service = "floki_projects/app"

# # Optional. Defaults to 'client'
 client_service = "floki_projects/client"

```
#### Logging and Reporting:
Implement logging and reporting features to provide detailed feedback during the build process. This helps developers identify and fix issues more efficiently.

```rust
fn setup_logging(verbose: u8) {
    let log_level = match verbose {
        0 => LevelFilter::Warn,
        1 => LevelFilter::Info,
        2 => LevelFilter::Debug,
        _ => LevelFilter::Trace,
    };
    let config = ConfigBuilder::default()
        .set_time_level(LevelFilter::Off)
        .build();
    TermLogger::init(log_level, config, TerminalMode::Stderr, ColorChoice::Auto)
        .expect("Failed to start logger");
    log::info!("Log level set to: {log_level}");
}

```

#### Testing:
Implement testing for your build tool itself to ensure reliability and stability.

I write a test for `app` project:
```bash
$ cargo test
    Finished test [unoptimized + debuginfo] target(s) in 0.02s
     Running unittests src/main.rs (cargo-floki/target/debug/deps/client-5bd14cdeb41d4264)

running 0 tests

test result: ok. 0 passed; 0 failed; 0 ignored; 0 measured; 0 filtered out; finished in 0.00s

$ cargo test
    Finished test [unoptimized + debuginfo] target(s) in 0.01s
     Running unittests src/lib.rs (cargo-floki/target/debug/deps/app-f55b4b7fb682729e)

running 1 test
test tests::test_add ... ok

test result: ok. 1 passed; 0 failed; 0 ignored; 0 measured; 0 filtered out; finished in 0.00s

   Doc-tests app

running 0 tests

test result: ok. 0 passed; 0 failed; 0 ignored; 0 measured; 0 filtered out; finished in 0.00s


```
#### Documentation:
Provide comprehensive documentation to help users understand how to configure and use the build tool effectively.

Generate documentation for all projects:

```bash
$ cargo floki doc
```
#### Continuous Integration (CI) Integration:
Ensure compatibility with popular CI/CD systems like Jenkins, Travis CI, GitLab CI, or GitHub Actions. This facilitates automated builds and deployments.

Build all projects that cleans, build, testing the project directory.
```bash
$ cargo floki build
$ cargo floki clean
$ cargo floki update
```

All these steps are for ease of work and speed of application development.