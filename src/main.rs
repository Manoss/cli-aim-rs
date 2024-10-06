use clap::{Parser, Subcommand};
use aimapi::*;

#[derive(Parser)]
#[command(version, about, long_about = None)]
struct Cli {
    /// Optional name to operate on
    #[arg(long,)]
    cb: String,

    #[command(subcommand)]
    command: Option<Commands>,

}

#[derive(Subcommand)]
enum Commands {
    /// functions
    Sread {

    },
    Swrite {
        #[arg(short, long,)]
        text: String,
    },
    Uread {
        /// Type
        #[arg(short, long,)]
        cast: i32,
    },
    Uwrite {
        /// Type
        #[arg(short, long,)]
        cast: i32,

        /// number
        #[arg(short, long, allow_hyphen_values(true))]
        number: f32,
    },
}

fn main() {

    let cli = Cli::parse();

    //Commands
    match &cli.command {
        Some(Commands::Sread { .. }) => {
            let resultat = s_read(cli.cb);
            match resultat {
                Ok(value) => println!("Resultat s_read {:?}", value),
                Err(error) => println!("Error {:?}", error),
            }
      
        },
        Some(Commands::Swrite { text }) => {
            let resultat = s_write(cli.cb, text.clone());
            match resultat {
                Ok(value) => println!("Resultat s_write {:?}", value),
                Err(error) => println!("Error {:?}", error),
            }
        },
        Some(Commands::Uread { cast }) => {
            let resultat = u_read(cli.cb, *cast);
            match resultat {
                Ok(value) => println!("Resultat u_read {:?}", value),
                Err(error) => println!("Error {:?}", error),
            }
        },
        Some(Commands::Uwrite { cast, number }) => {
            let resultat = u_write(cli.cb, *cast, *number);
            match resultat {
                Ok(value) => println!("Resultat Mode 4 {:?}", value),
                Err(error) => println!("Error {:?}", error),
            }
        },
        None => {}
    }
}