

#### How to building tools for cargo [cargo-floki] (Floki the boat builder) ?

Building a tool for managing the build process of multiple applications can be quite complex,
and writing build tools in Rust involves creating a program that automates various tasks related to building,
testing, and packaging your Rust projects. 
Below is a basic outline of how you might approach building such a tool:

#### Define Project Structure:
Create a standardized project structure to ensure consistency across different applications. This can simplify the build process by having a predictable layout.
```bash
$ tree
.
├── app
│   ├── Cargo.toml
│   └── src
│       └── lib.rs
└── client
    ├── Cargo.toml
    └── src
        └── main.rs

4 directories, 4 files

```
#### Configuration:
Allow for configuration options so users can customize the build process for their specific needs.
Consider using configuration files, environment variables, or a configuration management system.
```bash
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

```bash
$ cargo floki test
```
#### Documentation:
Provide comprehensive documentation to help users understand how to configure and use the build tool effectively.

```bash
$ cargo floki doc
```
#### Continuous Integration (CI) Integration:
Ensure compatibility with popular CI/CD systems like Jenkins, Travis CI, GitLab CI, or GitHub Actions. This facilitates automated builds and deployments.

Let's create a simple build tool that cleans, build, testing the project directory.
```bash
$ cargo floki build
```
##### Usage:
You can have multiple outputs at the same time by executing a command.

Install with: `cargo install --path .`

Get help: `cargo floki --help`