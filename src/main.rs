// rcli csv -i input.csv -o output.json -- header -d ',

use clap::Parser;
//use anyhow::Ok;

use rcli::{process_csv, Opts, SubCommand};

fn main() -> anyhow::Result<()> {
    let opts: Opts = Opts::parse();
    match opts.cmd {
        SubCommand::Csv(opts) => process_csv(&opts.input, &opts.output)?,
    }
    Ok(())
}
