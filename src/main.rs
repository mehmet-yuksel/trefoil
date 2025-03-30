use clap::{Parser, Subcommand};
use std::error::Error;
use std::path::Path;
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
            std::fs::create_dir_all(&commits_dir)?;
            let root_commit = Commit {
                id: 0,
                parent_id: None,
                instructions: vec![],
                timestamp: 0,
            };
            save_commit(&root_commit, &commits_dir)?;
            set_current_commit_id(0, vcdir)?;
            println!("Initialized empty repository");
        }
        Commands::Commit => {
            let code = std::fs::read_to_string("code.lisp")?;
            let tokens = tokenize(&code);
            let new_ast = parse(&tokens)?;
            let current_id = get_current_commit_id(vcdir)?;
            let current_ast = reconstruct_ast(current_id, &commits_dir)?;
            let mut path = vec![];
            let instructions = diff_ast(&current_ast, &new_ast, &mut path);
            let new_id = current_id + 1;
            let new_commit = Commit {
                id: new_id,
                parent_id: Some(current_id),
                instructions,
                timestamp: 0,
            };
            save_commit(&new_commit, &commits_dir)?;
            set_current_commit_id(new_id, vcdir)?;
            println!("Committed changes as commit {}", new_id);
        }
        Commands::Log => {
            let current_id = get_current_commit_id(vcdir)?;
            let chain = get_commit_chain(current_id, &commits_dir)?;
            for commit in chain.iter().rev() {
                println!("Commit {}", commit.id);
            }
        }
        Commands::Checkout { id } => {
            let ast = reconstruct_ast(id, &commits_dir)?;
            let code = ast.to_string();
            std::fs::write("code.lisp", code)?;
            set_current_commit_id(id, vcdir)?;
            println!("Checked out commit {}", id);
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
