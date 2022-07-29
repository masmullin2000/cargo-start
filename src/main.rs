use clap::Parser;
use git2::Repository;

#[derive(Parser)]
struct Args {
    #[clap(value_parser)]
    name: String,

    #[clap(short, long, action = clap::ArgAction::Count)]
    force: u8,
}

fn delete_current(force: u8, project_name: &str) -> bool {
    match force {
        0 => false,
        1 => {
            println!("!!WARNING!!");
            println!("This will delete the current directory {}", project_name);
            print!("Are you sure you wish to proceed? (type YES to proceed) : ");
            std::io::Write::flush(&mut std::io::stdout()).expect("error flush");
            
            let mut answer = String::new();
            if std::io::stdin().read_line(&mut answer).is_err() {
                eprintln!("error gathering information from commandline: quitting");
                std::process::exit(1);
            }
            answer.trim() == "YES" || answer.trim() == "yes"
        },
        _ => true,
    }
}

fn main() {
    let args = std::env::args();
    let args = args.filter(|a| a != "start");

    let args = Args::parse_from(args);

    let project_name = args.name;

    if std::path::Path::new(&project_name).exists() {
        if delete_current(args.force, &project_name) {
            if let Err(e) = std::fs::remove_dir_all(&project_name) {
                let k = e.kind();
                if k != std::io::ErrorKind::NotFound {
                    eprintln!("failed to remove existing project: {e}");
                    std::process::exit(1);
                }
            }
        } else {
            eprintln!("Directory {} exists", project_name);
            eprintln!("Use the -f flag for interactive delete, or -ff to force delete this directory");
            eprintln!("Exiting");
            std::process::exit(1);
        }
    }

    let url = "https://github.com/masmullin2000/rust_starter_project.git";
    if let Err(e) = Repository::clone(url, &project_name) {
        eprintln!("failed to clone template project: {}", e.message());
        std::process::exit(1);
    }

    let git_dir = format!("./{}/.git/", &project_name);

    if let Err(e) = std::fs::remove_dir_all(git_dir) {
        eprintln!("failed to remove .git directory: {e}");
        std::process::exit(1);
    }

    if let Err(e) = Repository::init(&project_name) {
        eprintln!("failed to initialize project: {}", e.message());
        std::process::exit(1);
    }

    let cargo_file = format!("./{}/Cargo.toml", project_name);

    match std::fs::read_to_string(&cargo_file) {
        Ok(mut cargo_file_data) => {
            cargo_file_data = cargo_file_data.replace("rust_starter_project", &project_name);
            let name_is = format!("name = \"{}\"", project_name);
            cargo_file_data = cargo_file_data.replace("name = \"bin\"", &name_is); 

            if let Err(e) = std::fs::write(cargo_file, cargo_file_data) {
                eprintln!("failed to write Cargo.toml file: {e}");
            }
        }
        Err(e) => eprintln!("failed to read Cargo.toml file: {e}"),
    }
}
