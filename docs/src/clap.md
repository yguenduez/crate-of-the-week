# Clap - 6th of August

`cargo add clap -F derive`

Clap is useful, if you want to write a CLI program, that accepts arguments and options.

Clap offers two ways of defining arguments and options for your program:
 - Using the `derive` feature of `clap`: [reference](https://docs.rs/clap/latest/clap/_derive/index.html)
 - Using the builder method, which is more flexible and allows for more complex configurations: [reference](https://docs.rs/clap/latest/clap/struct.Command.html)

For simplicity, we will look at the `derive` feature of `clap`.

## How to use Clap

For a simple program, you define a struct, and it's fields become the arguments and options of your program.
If you use the `derive` feature of `clap`, you can use the `Parser` derive macro to automatically generate the clap parser for your struct
and you can instantly access the arguments and options.

An example from their [docs](https://docs.rs/clap/latest/clap/):

```rust
use clap::Parser;

/// Simple program to greet a person
#[derive(Parser, Debug)]
#[command(version, about, long_about = None)]
struct Args {
    /// Name of the person to greet
    #[arg(short, long)]
    name: String,

    /// Number of times to greet
    #[arg(short, long, default_value_t = 1)]
    count: u8,
}

fn main() {
    let args = Args::parse();

    for _ in 0..args.count {
        println!("Hello {}!", args.name);
    }
}
```

If you now run this program with no arguments, `Args::parse()` will panic, as it expects an argument with `--name`, which is not an optional, therefore required argument.

But how can you give args to your Rust program:

- Via your binary (usually in your `target` directory):

```sh
./<you_binary_name> --name Bernd
```

- Via cargo

```sh
cargo run -- --name Bernd
```

- Installing your program (will be installed in `~/.cargo/bin`, which is in your `PATH`) - then use it:

```sh
cargo install --path .
<your-binary-name> --name Bernd
```

## Try it out

Generate a new rust project:

```sh
cargo new clap_project
```

Add clap as a dependency:

```sh
cargo add clap -F derive
```

Play around with it :). You can have a look at the [clap examples](https://github.com/clap-rs/clap/tree/master/examples).
