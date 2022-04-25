# heloworldr

Refer to https://doc.rust-lang.org/book/title-page.html if you need help

## Setup

Reference: https://doc.rust-lang.org/book/ch01-01-installation.html

### Linux & macOS

```bash
curl --proto '=https' --tlsv1.2 https://sh.rustup.rs -sSf | sh
```

### Windows

Download from https://www.rust-lang.org/tools/install

## Usage

You can build the project (also used to download new crates once you add them to `Cargo.toml`)

```bash
cargo build
```

Or you can just run directly, which will rebuild if necessary

```bash
cargo run
# or if you want to pass arguments
cargo run -- -h
```

### Dependencies

Dependencies (crates) are specified in `Cargo.toml`, you can find them through Google (e.g. `rust base64`) or directly from https://crates.io/crates/, which also gives you the latest version (making it easy to add to `Cargo.toml`).

You can download `cargo-edit` using

```bash
cargo install cargo-edit
```

which gives you access to `cargo add` to add new dependencies, e.g. `cargo add clap` adds the `clap` crates.

Note that crates can be added with `features` flags which enable/disable some features, this is usually documented on https://crates.io/.

### Static analysis

You can use `clippy` for linting and `fmt` for formatting

```bash
cargo clippy # checks
cargo clippy --fix # fixes

cargo fmt --all --check # checks
cargo fmt --all # fixes
```

## Features

The app is very basic, it only takes a name and prints Hello World with it. We would like to do much more.

### Choosing the language

_Themes:_ trait implementation, enums, matching, error handling

Currently, the app always replies in English. We already added a trait with one implementation for English but we would like to add another one.

Choose another language and add an extra implementation, mirroring the current module structure.

Then you will need to get the language from the command-line (maybe let's keep English as a default?) and use the factory to get the right implementation.

### Output to a file

_Themes:_ standard library usage, error handling

The app just prints to the standard output but we might want to save that result for later. Obviously we could use a simple shell redirection but it's much more fun to deal with the filesystem ourselves (isn't it?).

Let's take another command-line argument and output to that file. Maybe we should check if the file exists as well and ask the user if we should overwrite it?

Might be worth to add a force and interactive flag as well to allow the user to avoid that annoying prompt...

### Output an image to a file

_Themes:_ crate installation and usage, error handling

Text is all good but in a world where memes are supreme, we should really be working with images.

Using https://crates.io/crates/fontdue, we can make bitmap from text. You will need to specify the font yourself, so go wild and choose something funky from https://www.dafont.com/ (maybe you could add multiple ones and add a flag to choose between them, maybe make them themes and add color as part of it??).

You can then use https://crates.io/crates/image to save the image (and do some other manipulation, cropping, blur, etc... maybe include another image first and make an actual meme??? for reference you would be looking for the "Impact" font).

Tips: check https://github.com/mooman219/fontdue/blob/master/dev/examples/raster-print.rs#L22 and https://docs.rs/image/latest/image/enum.DynamicImage.html#method.put_pixel to learn how to transform data from `fontdue` to `image`.

### Abstracting the output mechanism

_Themes:_ trait definition & implementation, module creation & inclusion

Well, now we have **3** different ways to output our result! A file, an image or the console... Maybe we could abstract that? Taking inspiration from the `languages` module (or not, I mean it's a free country), create your own trait and its implementations which would abstract the creation of those artifacts so that adding a new method (e.g. posting it to https://pastebin.com, which you can add as well, go wild!) is trivial and separated from the main logic.

Note that module inclusion rules might be a bit peculiar/unexpected, use `super::` or `crate::` to keep your code readable.

### Verbose mode

_Themes:_ trait expansion, logging, matching

Sometimes it's nice to be able to see what the app is doing and why. Adding a verbose flag would be nice.

We could use the https://crates.io/crates/log crate (potentially paired with another crate for initialization like https://crates.io/crates/simplelog) to display the data... but what data?

Maybe adding more functions to our `Language` trait which will help diagnose what it is through polymorphism (e.g. show what language it is... maybe we could even return the enum used to create it??) so we can print it from the main without adding logging everywhere?

### Automatic language

_Themes:_ web integration, serialization, structure definition

What if we could add a new implementation to our `Language` trait which would automatically use an API to do the translation? More expensive (in terms of money and performance) but much more flexible! You can get a variety of crates which wrap Google Translate https://crates.io/keywords/translate, but you could even do it yourself using https://crates.io/crates/reqwest for HTTP and https://crates.io/crates/serde for JSON serialization.

In the case of Google Translate, you will need to create a project: https://console.cloud.google.com/projectcreate and enable the Google Translate API (be careful of cost and stick to the free-tier/development settings!).

### Server mode

_Themes:_ networking, multi-threading, serialization

Well, this is all great, but in the days of the Internet, I can't be bothered to use a command-line utility, everything must be networked and _cloud_. Let's create a server which can be queried to greet us. It must be able to handle many clients at the same time because we definitely will need the scale.

You can choose the protocol, but here are some ideas:
 - Rest with JSON: using https://crates.io/crates/serde for serialization and one of the plethora of high-level web framework (https://github.com/flosse/rust-web-framework-comparison#high-level-server-frameworks) or low-level if you are feeling up to it -> easy to test using cURL
 - gRPC: using https://crates.io/crates/tonic -> easy to test using grpcURL
 - Do something wild! Custom TCP protocol, UDP broadcasts, Gopher, really go for it!

Note that in order to parallelize client requests, you will need to use some kind of threading library... and they are a lot. You can use `std::thread` but based on the server library you choose, you might not have a choice. https://crates.io/crates/tokio is a safe choice (both `tonic` and `hyper` use it) and so is https://crates.io/crates/futures.
