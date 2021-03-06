use clap::Parser;

/// Search for a pattern in a file and display the lines that contain it
#[derive(Parser)]
struct Cli{
    /// The pattern to look for
    pattern: String,
    /// The path to the file to read
    #[clap(parse(from_os_str))]
    path: std::path::PathBuf,
}

fn main() {
    let args = Cli::parse();
    let mut found = false;

    let result = std::fs::read_to_string(&args.path);
    let content = match result{
        Ok(content) => { content },
        Err(error) => { panic!("Can't deal with {}, just exit here", error); }
    };

    for line in content.lines() {
        if line.contains(&args.pattern) {
            println!("{}", line);
            found = true;
        }
    }

    if !found {
        println!("Pattern not found!");
    }
}
