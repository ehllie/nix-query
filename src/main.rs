use std::io::stdout;
use std::path::{Path, PathBuf};
use std::process::Command;

use anyhow::Result;
use lazy_static::lazy_static;
use ratatui::{
    backend::CrosstermBackend,
    crossterm::{
        event::{self, Event, KeyCode},
        terminal::{disable_raw_mode, enable_raw_mode, EnterAlternateScreen, LeaveAlternateScreen},
        ExecutableCommand,
    },
    widgets::{Block, Paragraph},
    Frame, Terminal,
};
use serde::{Deserialize, Serialize};

mod cli;
use cli::{Args, Parser};

use nquery::FlakeInfo;

/*
TODO:
1. Determine flake sources: nixpkgs, home-manager, nix-darwin etc.
2. Extract modules and packages from flakes.
3. Store them in a database.
3a. Database schema: modules, packages, flake_sources.
3b: Migrations, sqlx and sqlite.
4. Query the database to get the modules and packages.
5. Display the modules and packages in the terminal.
6. Allow for configuration of keybinds and sources list.
*/

fn main() -> Result<()> {
    let args = Args::parse();
    if args.index {
        index_flakes()
    } else {
        tui()
    }
}

lazy_static! {
    static ref NIX_LIB_DIR: PathBuf = match option_env!("NXQ_LIB") {
        Some(lib) => Path::new(lib).to_path_buf(),
        None => Path::new(env!("CARGO_MANIFEST_DIR")).join("nix_lib"),
    };
    static ref EXTRACT_PATH: PathBuf = NIX_LIB_DIR.join("extract.nix");
    static ref EXTRACT_ARGS: [&'static str; 4] = [
        "eval",
        "--json",
        "-f",
        EXTRACT_PATH
            .to_str()
            .expect("Could not convert path to str"),
    ];
}

const META_ARGS: [&str; 4] = ["flake", "metadata", "--json", "--no-write-lock-file"];

fn index_flakes() -> Result<()> {
    let nixpkgs_info = {
        let mut cmd = Command::new("nix");
        cmd.args(META_ARGS);
        cmd.arg("github:nixos/nixpkgs/nixos-unstable");
        let output = cmd.output()?;
        let output = String::from_utf8(output.stdout)?;
        serde_json::from_str::<FlakeInfo>(&output)?
    };

    let sources = vec![(
        "github:nix-community/home-manager",
        NIX_LIB_DIR.join("hm.nix"),
    )];
    for (source, extractor) in sources {
        let mut cmd = Command::new("nix");
        cmd.args(META_ARGS);
        cmd.arg(source);
        let output = cmd.output()?;
        let output = String::from_utf8(output.stdout)?;
        let flake_info = serde_json::from_str::<FlakeInfo>(&output)?;

        let mut cmd = Command::new("nix");
        cmd.args(EXTRACT_ARGS.iter());
        cmd.args([
            "--argstr",
            "nixpkgs-flake",
            &nixpkgs_info.path.to_string_lossy(),
            "--argstr",
            "input-flake",
            &flake_info.path.to_string_lossy(),
            "--argstr",
            "extractor",
            &extractor.to_string_lossy(),
            "results",
        ]);
        // We will not be building any packages, but generating default values might fail if
        // a default value is a package that is marked as broken on current platform.
        cmd.env("NIXPKGS_ALLOW_UNSUPPORTED_SYSTEM", "1");
        cmd.env("NIXPKGS_ALLOW_BROKEN", "1");
        let output = cmd.output()?;
        if !output.stderr.is_empty() {
            eprintln!("Error: {}", String::from_utf8(output.stderr)?);
        }
        let output = String::from_utf8(output.stdout)?;
        println!("{}", output);
    }

    Ok(())
}

fn tui() -> Result<()> {
    // Magic to setup terminal and crossterm
    enable_raw_mode()?;
    stdout().execute(EnterAlternateScreen)?;

    let mut terminal = Terminal::new(CrosstermBackend::new(stdout()))?;

    let mut should_quit = false;
    while !should_quit {
        terminal.draw(ui)?;
        should_quit = handle_events()?;
    }

    // Magic to clean up terminal and crossterm
    disable_raw_mode()?;
    stdout().execute(LeaveAlternateScreen)?;
    Ok(())
}

fn handle_events() -> Result<bool> {
    if event::poll(std::time::Duration::from_millis(50))? {
        if let Event::Key(key) = event::read()? {
            if key.kind == event::KeyEventKind::Press && key.code == KeyCode::Char('q') {
                return Ok(true);
            }
        }
    }
    Ok(false)
}

fn ui(frame: &mut Frame) {
    frame.render_widget(
        Paragraph::new("Hello Tui!").block(Block::bordered().title("Greeting")),
        frame.area(),
    );
}
