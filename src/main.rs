use anyhow::Result;
use clap::{Arg, ArgAction, Command};
use std::fs::File;
use std::io::{self, BufRead, BufReader, Write};

enum Mode {
    Defang,
    Fang,
}

fn defang(s: &str) -> Result<String> {
    Ok(s.replace(".", "[.]").replacen("http", "hxxp", 1))
}

fn fang(s: &str) -> Result<String> {
    Ok(s.replacen("hxxp", "http", 1).replace(r"[.]", r"."))
}

fn convert_in_out<R: BufRead, W: Write>(mode: Mode, reader: &mut R, writer: &mut W) -> Result<()> {
    for line in reader.lines() {
        let line = line?;
        let res = match mode {
            Mode::Defang => defang(&line)?,
            Mode::Fang => fang(&line)?,
        };
        writeln!(writer, "{}", res)?;
    }
    writer.flush()?;
    Ok(())
}

fn main() -> Result<()> {
    let matches = Command::new("defang")
        .version("0.1")
        .author("Amikai Chuang")
        .about(
            "Defang defang or fang url from FILE or standard input line by line to standard output.",
        )
        .arg(
            Arg::new("decode")
                .short('d')
                .long("decode")
                .help("fang the url (reverse operation of defang)")
                .action(ArgAction::SetTrue),
        )
        .arg(
            Arg::new("input")
                .value_name("FILE")
                .help("input file. if not specify, the default is stdin"),
        )
        .get_matches();

    let mode = if matches.get_flag("decode") {
        Mode::Fang
    } else {
        Mode::Defang
    };

    let mut w = io::stdout().lock();

    let mut r: Box<dyn BufRead> = if let Some(filename) = matches.get_one::<String>("input") {
        Box::new(BufReader::new(File::open(filename)?))
    } else {
        Box::new(io::stdin().lock())
    };

    match convert_in_out(mode, &mut r, &mut w) {
        Ok(_) => Ok(()),
        Err(e) => Err(e),
    }
}
