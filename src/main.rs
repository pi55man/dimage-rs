use std::{fs, path::PathBuf};

use clap::*;
use serde::Deserialize;
use toml;

#[derive(Parser, Debug, Clone)]
struct Args {
    #[arg(short, long, default_value = ".")]
    path: String,
    #[arg(short, long, default_value = "Dockerfile")]
    output: String,
    #[arg(long, required = false)]
    port: Option<i32>,
}

#[derive(Deserialize, Debug)]
struct CargoToml {
    package: Package,
}

#[derive(Deserialize, Debug)]
struct Package {
    name: String,
}

fn main() {
    let args = Args::parse();
    let toml_path = PathBuf::from(&args.path).join("Cargo.toml");
    let content =
        fs::read_to_string(&toml_path).expect("[!] Failed to open cargo.toml for this directory");
    let parsed: CargoToml =
        toml::from_str(&content).expect("[!] Failed to parse cargo.toml for this directory");

    let dockerfile = generate_dockerfile(&parsed.package.name, args.clone());
    fs::write(&args.output, dockerfile).expect("[!] Failed to write Dockerfile");
    println!("[+] Dockerfile created at {}", args.output);
}

fn generate_dockerfile(name: &String, args: Args) -> String {
    let mut content = String::from("");
    content.push_str("FROM rust:latest AS builder\n");
    content.push_str("WORKDIR /app\n");
    content.push_str("COPY . .\n");
    content.push_str("RUN rustup target add x86_64-unknown-linux-musl\n");
    content.push_str("RUN cargo build --release --target x86_64-unknown-linux-musl\n");
    content.push_str("FROM alpine:latest\n");
    content.push_str("RUN apk add --no-cache ca-certificates\n");

    if let Some(port) = args.port {
        content.push_str(format!("EXPOSE {}\n", port).as_str());
    }

    content.push_str(format!("COPY --from=builder /app/target/x86_64-unknown-linux-musl/release/{} usr/local/bin/app\n",name).as_str());
    content.push_str(r#"CMD ["app"]"#);
    content
}
