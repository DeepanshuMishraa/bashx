mod cache;
mod command_executor;
use ::clap::*;
use cache::{clean_cache, get_cache_dir};
use command_executor::{run_bash_script, run_git_clone, set_executable_permission};
use walkdir::WalkDir;

#[derive(Parser, Debug)]
#[command(
    name = "bashx",
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

            let clone_dir_path = get_cache_dir();

            let clone_dir = clone_dir_path.to_str().unwrap_or_default();

            println!("Cloning repository from {} into {}", url, clone_dir);

            if let Err(e) = run_git_clone(url, clone_dir) {
                eprintln!("Error cloning repository: {}", e);
                std::process::exit(1);
            }
            println!("Repository cloned successfully.");
        }
        Commands::List => {
            let scripts_dir = get_cache_dir();
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
            if let Err(e) = clean_cache() {
                eprintln!("Error cleaning cache: {}", e);
                std::process::exit(1);
            }
            println!("Cache cleaned successfully.");
            std::process::exit(0);
        }
        Commands::Run { name } => {
            //run the script
            let cache_dir = get_cache_dir();

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
            if let Err(e) = set_executable_permission(&script_file) {
                eprintln!("Error setting permissions: {}", e);
                std::process::exit(1);
            }
            // run the script from its directory
            println!("Running script: {}", script_file.display());

            if let Err(e) = run_bash_script(&script_file, script_dir) {
                eprintln!("Error running script: {}", e);
                std::process::exit(1);
            } else {
                println!("Script executed successfully.");
            }
            std::process::exit(0);
        }
    }
}
