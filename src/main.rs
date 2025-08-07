use ::clap::*;

#[derive(Parser, Debug)]
#[command(
    name = "bpx",
    version,
    about = "Run bash scripts from GitHub with ease"
)]

struct Args {
    #[command(subcommand)]
    command: Commands,
}

#[derive(Subcommand, Debug)]
enum Commands {
    Get {
        url: String,
    },
    List,
    Clean,
    #[command(about = "Run a script from the cache")]
    Run {
        name: String,
    },
}

fn main() {
    let args = Args::parse();

    match &args.command {
        Commands::Get { url } => {
            if url.is_empty() {
                println!("Error: URL cannot be empty.");
                std::process::exit(1);
            }

            let clone_dir_path = dirs::home_dir()
                .unwrap_or_default()
                .join(".bpx")
                .join("cache");

            let clone_dir = clone_dir_path.to_str().unwrap_or_default();

            println!("Cloning repository from {} into {}", url, clone_dir);

            let output = std::process::Command::new("git")
                .arg("clone")
                .arg(url)
                .arg(clone_dir)
                .output();

            match output {
                Ok(output) => {
                    if output.status.success() {
                        println!("Repository cloned successfully.");
                    } else {
                        eprintln!(
                            "Error cloning repository: {}",
                            String::from_utf8_lossy(&output.stderr)
                        );
                        std::process::exit(1);
                    }
                }
                Err(e) => {
                    eprintln!("Failed to execute git command: {}", e);
                    std::process::exit(1);
                }
            }
        }
        Commands::List => {
            //Here we would list all the available scripts in the bpx dir
            let scripts_dir = dirs::home_dir()
                .unwrap_or_default()
                .join(".bpx")
                .join("cache");
            if !scripts_dir.exists() {
                println!(
                    "You have no scripts available. Please run `bpx get <url>` to add a script."
                );
                std::process::exit(0);
            }

            // only display .sh files in the cache directory
            let scripts = std::fs::read_dir(&scripts_dir);

            match scripts {
                Ok(entries) => {
                    let mut script_count = 0;
                    for entry in entries {
                        let path = entry.unwrap().path();
                        if path.extension().and_then(|s| s.to_str()) == Some("sh") {
                            println!("found {} scripts {}", script_count, path.display());
                            script_count += 1;
                        } else {
                            println!("No Bash Scripts found.");
                            std::process::exit(0);
                        }
                    }
                }
                Err(e) => {
                    eprintln!("Error reading scripts directory: {}", e);
                    std::process::exit(1);
                }
            }
        }
        Commands::Clean => {
            println!("Cleaning up scripts...");
            let cache_dir = dirs::home_dir()
                .unwrap_or_default()
                .join(".bpx")
                .join("cache");
            if cache_dir.exists() {
                match std::fs::remove_dir_all(&cache_dir) {
                    Ok(_) => println!("Cache directory cleaned successfully."),
                    Err(e) => {
                        eprintln!("Error cleaning cache directory: {}", e);
                        std::process::exit(1);
                    }
                }
            } else {
                println!("No cache directory found to clean.");
            }
        }
        _ => {
            println!("Unknown command. Use --help for more information.");
            std::process::exit(1);
        }
        Commands::Run { name } => {
            //run the script
        }
    }
}
