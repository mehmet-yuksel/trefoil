use clap::{Parser, Subcommand};
use std::error::Error;
use std::path::Path;
use trefoil::ast::Ast;
use trefoil::diff::diff_ast;
use trefoil::parser::{parse, tokenize};
use trefoil::vc::Commit;
use trefoil::vc::{
    get_commit_chain, get_current_commit_id, reconstruct_ast, save_commit, set_current_commit_id,
};

#[derive(Parser)]
#[command(name = "trefoil")]
struct Cli {
    #[command(subcommand)]
    command: Commands,
}

#[derive(Subcommand)]
enum Commands {
    Init,
    Commit,
    Log,
    Checkout { id: u64 },
    Debug { id: u64 },
}

fn main() -> Result<(), Box<dyn Error>> {
    let cli = Cli::parse();
    let vcdir = Path::new(".trefoil");
    let commits_dir = vcdir.join("commits");
    match cli.command {
        Commands::Init => {
            if vcdir.exists() {
                println!("Repository already initialized.");
            } else {
                std::fs::create_dir_all(&commits_dir)?;
                let root_commit = Commit {
                    id: 0,
                    parent_id: None,
                    instructions: vec![],
                    timestamp: 0, // TODO: use real timestamp
                };
                save_commit(&root_commit, &commits_dir)?;
                set_current_commit_id(0, vcdir)?;

                let code_file = Path::new("code.lisp");
                if !code_file.exists() {
                    std::fs::write(code_file, "")?;
                    println!("Initialized empty repository and created empty 'code.lisp'.");
                } else {
                    println!("Initialized empty repository. 'code.lisp' already exists.");
                }
            }
        }
        Commands::Commit => {
            let code = std::fs::read_to_string("code.lisp")?;
            let tokens = tokenize(&code);
            let new_ast = parse(&tokens)?;

            let current_id = get_current_commit_id(vcdir)?;
            let current_ast = reconstruct_ast(current_id, &commits_dir)?;

            let mut path = vec![];
            let instructions = diff_ast(&current_ast, &new_ast, &mut path);

            if instructions.is_empty() {
                println!("No changes detected in 'code.lisp'. Nothing to commit.");
            } else {
                let commit_files = std::fs::read_dir(&commits_dir)?
                    .filter_map(|entry| entry.ok())
                    .filter_map(|entry| entry.path().file_stem()?.to_str()?.parse::<u64>().ok())
                    .collect::<Vec<u64>>();
                let next_id = commit_files.iter().max().map_or(0, |max_id| max_id + 1);

                let new_commit = Commit {
                    id: next_id,
                    parent_id: Some(current_id),
                    instructions,
                    timestamp: 0, // TODO: use real timestamp
                };
                save_commit(&new_commit, &commits_dir)?;
                set_current_commit_id(next_id, vcdir)?;
                println!("Committed changes as commit {}", next_id);
            }
        }
        Commands::Log => {
            let current_id = get_current_commit_id(vcdir)?;
            let chain = get_commit_chain(current_id, &commits_dir)?;
            if chain.is_empty() {
                println!("No commits found.");
            } else {
                println!("Commit History (newest first):");
                for commit in &chain {
                    print!(
                        "* commit {} (parent: {:?})",
                        commit.id,
                        commit
                            .parent_id
                            .map(|id| id.to_string())
                            .unwrap_or_else(|| "None".to_string())
                    );
                    if commit.id == current_id {
                        print!(" (HEAD)");
                    }
                    println!();
                }
            }
        }
        Commands::Checkout { id } => {
            let commit_path = commits_dir.join(format!("{}.json", id));
            if !commit_path.exists() {
                return Err(format!("Commit with id '{}' not found.", id).into());
            }

            let ast = reconstruct_ast(id, &commits_dir)?;

            let code_to_write = match ast {
                Ast::List(nodes) => nodes
                    .iter()
                    .map(|node| node.to_string())
                    .collect::<Vec<String>>()
                    .join("\n"),
                _ => ast.to_string(),
            };

            std::fs::write("code.lisp", code_to_write)?;
            set_current_commit_id(id, vcdir)?;
            println!("Checked out commit {}. 'code.lisp' updated.", id);
        }
        Commands::Debug { id } => {
            let commits_dir = vcdir.join("commits");
            let chain = get_commit_chain(id, &commits_dir)?;
            let commit = chain
                .iter()
                .find(|c| c.id == id)
                .ok_or("Commit not found")?;
            println!("Instructions for commit {}:", id);
            for (i, instruction) in commit.instructions.iter().enumerate() {
                println!(
                    "{}. {}",
                    i + 1,
                    instruction.to_string().replace("[", "(").replace("]", ")")
                );
            }
        }
    }
    Ok(())
}
