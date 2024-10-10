use anyhow::Context;
use anyhow::Result;
use clap::{Arg, ArgAction, Command};
use std::fs::File;
use std::io::{self, BufRead, BufReader, Write};
use url::Url;

enum Mode {
    Defang,
    Fang,
}

fn defang(s: &str) -> Result<String> {
    // check the url is follow the standard: https://url.spec.whatwg.org/
    let _ = Url::parse(s).context("url is not valid")?;

    let mut ret = String::new();
    let mut count = 0;

    for c in s.replacen("http", "hxxp", 1).chars() {
        if c == '/' {
            count += 1;
        }

        if count < 3 && c == '.' {
            ret.push_str("[.]");
        } else {
            ret.push(c);
        }
    }
    Ok(ret)
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
        writeln!(writer, "{res}")?;
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

    if let Some(filename) = matches.get_one::<String>("input") {
        convert_in_out(mode, &mut BufReader::new(File::open(filename)?), &mut w)
    } else {
        convert_in_out(mode, &mut io::stdin().lock(), &mut w)
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use pretty_assertions::assert_eq;

    #[test]
    fn defang_normal() {
        let got = defang("http://google.com").unwrap();
        let want = "hxxp://google[.]com";
        assert_eq!(want, &got);
    }

    #[test]
    fn defang_url_in_url() {
        let got = defang("http://google.com/?u=http://www.google.com").unwrap();
        let want = "hxxp://google[.]com/?u=http://www.google.com";
        assert_eq!(want, &got);
    }

    #[test]
    fn fang_normal() {
        let got = fang("hxxp://google[.]com").unwrap();
        let want = "http://google.com";
        assert_eq!(want, &got)
    }

    #[test]
    fn fang_url_in_url() {
        let got = fang("hxxp://google[.]com/?u=http://www.google.com").unwrap();
        let want = "http://google.com/?u=http://www.google.com";
        assert_eq!(want, &got);
    }
}
