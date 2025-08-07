use ::clap::*;
use walkdir::WalkDir;

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

            // only display .sh files in the cache directory and subdirectories
            let mut script_count = 0;

            for entry in WalkDir::new(&scripts_dir)
                .into_iter()
                .filter_map(|e| e.ok())
                .filter(|e| e.path().extension().and_then(|s| s.to_str()) == Some("sh"))
            {
                println!("Script {}: {}", script_count + 1, entry.path().display());
                script_count += 1;
            }

            if script_count == 0 {
                println!("No Bash Scripts found.");
            } else {
                println!("Found {} script(s) total.", script_count);
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
        Commands::Run { name } => {
            //run the script
            let cache_dir = dirs::home_dir()
                .unwrap_or_default()
                .join(".bpx")
                .join("cache");

            // recursively search for the script name in all directories
            let mut found_script: Option<std::path::PathBuf> = None;

            for entry in WalkDir::new(&cache_dir)
                .into_iter()
                .filter_map(|e| e.ok())
                .filter(|e| e.path().extension().and_then(|s| s.to_str()) == Some("sh"))
            {
                if let Some(file_name) = entry.path().file_stem().and_then(|s| s.to_str()) {
                    if file_name == name {
                        found_script = Some(entry.path().to_path_buf());
                        break;
                    }
                }
            }

            let script_file = match found_script {
                Some(path) => path,
                None => {
                    eprintln!("Script '{}' not found in cache.", name);
                    std::process::exit(1);
                }
            };

            println!(
                "The file you are about to run may contain malicious code. Please review the script before running it."
            );

            let choice = dialoguer::Confirm::new()
                .with_prompt("Do you want to run this script?")
                .default(false)
                .interact()
                .unwrap_or(false);

            if !choice {
                println!("Script execution cancelled.");
                std::process::exit(0);
            }

            // get the directory containing the script
            let script_dir = script_file.parent().unwrap_or_else(|| {
                eprintln!("Could not determine script directory");
                std::process::exit(1);
            });

            // give chmod permissions to the script
            let perm = std::process::Command::new("chmod")
                .arg("+x")
                .arg(&script_file)
                .output();

            match perm {
                Ok(output) => {
                    if output.status.success() {
                        println!("Permissions set successfully. Running script...");
                    } else {
                        eprintln!("Error setting permissions");
                        std::process::exit(1);
                    }
                }
                Err(_) => {
                    eprintln!("Failed to set permissions for the script.");
                    std::process::exit(1);
                }
            }

            // run the script from its directory
            println!("Running script: {}", script_file.display());

            let status = std::process::Command::new("bash")
                .arg(&script_file)
                .current_dir(script_dir)
                .status();

            match status {
                Ok(status) => {
                    if status.success() {
                        println!("Script executed successfully.");
                    } else {
                        eprintln!("Script failed with exit code: {:?}", status.code());
                        std::process::exit(1);
                    }
                }
                Err(e) => {
                    eprintln!("Failed to execute script: {}", e);
                    std::process::exit(1);
                }
            }
        }
    }
}
