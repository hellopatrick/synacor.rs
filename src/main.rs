mod errors;
mod ops;
mod vm;

use std::env;
use std::error::Error;
use std::fs::File;

fn main() -> Result<(), Box<Error>> {
    let args: Vec<String> = env::args().collect();

    let path = if args.len() > 1 {
        &args[1]
    } else {
        "./docs/challenge.bin"
    };

    let mut f = File::open(path).map_err(|_| "Unable to load file.")?;

    let mut vm = vm::VM::load(&mut f);

    vm.run()?;

    Ok(())
}
