use clap::Parser as _;
use hyprland::keyword::*;

#[derive(clap::Parser)]
#[command(author, version, about, long_about = None)]
#[command(propagate_version = true)]
struct Cli {
    #[command(subcommand)]
    command: Commands,
}

#[derive(clap::Subcommand)]
enum Commands {
    CursorZoom(CursorZoomCommand),
}

#[derive(clap::Args)]
struct CursorZoomCommand {
    #[command(subcommand)]
    command: CursorZoomCommands,
}

#[derive(clap::Subcommand)]
enum CursorZoomCommands {
    In,
    Out,
    Reset,
}

fn cursor_zoom(command: CursorZoomCommand) -> hyprland::Result<()> {
    let mut cursor_zoom_factor = match Keyword::get("misc:cursor_zoom_factor")?.value {
        OptionValue::Float(i) => i,
        _ => panic!("border size can only be a int"),
    };

    match command.command {
        CursorZoomCommands::In => cursor_zoom_factor += 0.1 * cursor_zoom_factor,
        CursorZoomCommands::Out => cursor_zoom_factor -= 0.1 * cursor_zoom_factor,
        CursorZoomCommands::Reset => cursor_zoom_factor = 1.0,
    }

    cursor_zoom_factor = (cursor_zoom_factor * 10.0).round() / 10.0;
    if cursor_zoom_factor < 1.0 {
        cursor_zoom_factor = 1.0;
    }

    println!("new zoom factor: {cursor_zoom_factor}");

    Keyword::set("misc:cursor_zoom_factor", cursor_zoom_factor)?;

    Ok(())
}

fn main() -> hyprland::Result<()> {
    let cli = Cli::parse();

    match cli.command {
        Commands::CursorZoom(command) => cursor_zoom(command),
    }
}
