use std::env;

fn main() {
  println!("cargo:rustc-env=SECRET={}", env!("SECRET"));
}