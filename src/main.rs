use clap::Parser;

// Program to greet people
#[derive(Parser, Debug)]
#[clap(author, version, about, long_about = None)]
struct Args {
    /// Name of the person to greet
    #[clap(short, long)]
    name: Option<String>, // Option -> Some(thing) | None = std::optional

                          // TODO: would like to add --language?

                          // TODO: maybe a --output to put it into a specific file too?
}

mod languages;

fn greet(name: &str) {
    let lang = languages::LanguageFactory::new_language(languages::LanguageName::English);

    // TODO: what if we wanted to print to a file? maybe generate an image?
    println!("{}", lang.greet(name));
}

fn main() {
    let args = Args::parse();

    // Use match to unpack the values and set a default
    let person = match args.name {
        Some(name) => name,
        None => "world".to_string(),
    };
    // Same but use a chain
    // let person = args.name.unwrap_or("world".to_string());

    greet(person.as_str());
}
