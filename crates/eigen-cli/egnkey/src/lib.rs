use args::{Args, Commands};
use generate::generate;
pub mod args;
mod generate;

pub fn execute_egnkey(args: Args) {
    match args.command {
        Commands::Generate {
            key_type,
            num_keys,
            output_dir,
        } => generate(key_type, num_keys, output_dir),
        Commands::Convert {
            private_key,
            output_file,
            password,
        } => todo!(),
        Commands::DeriveOperatorId { private_key } => todo!(),
    };
}
