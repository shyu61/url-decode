use anyhow::{Context, Result};
use atty::Stream;
use log::*;
use std::io::{self, Read};
use structopt::StructOpt;
use url::percent_encoding::percent_decode;

#[derive(StructOpt, Debug)]
struct Opt {
    #[structopt(name = "INPUT")]
    input: Option<String>,
}

fn main() -> Result<()> {
    pretty_env_logger::init();
    let opt = Opt::from_args();

    debug!("opt: {:?}", opt);

    if opt.input.is_none() && !is_stdin(opt.input.as_ref()) {
        Opt::clap().print_help()?;
        std::process::exit(1);
    }

    let input = match opt.input {
        Some(i) => i,
        None => read_from_stdin()?,
    };
    if input.is_empty() {
        Opt::clap().get_matches().usage();
    }

    #[allow(clippy::unit_arg)]
    Ok(println!(
        "{}",
        decode(&input).with_context(|| String::from("Failed decode"))?
    ))
}

fn is_stdin(input: Option<&String>) -> bool {
    let is_request = matches!(input, Some(i) if i == "-");

    let is_pipe = !atty::is(Stream::Stdin);

    is_request || is_pipe
}

fn read_from_stdin() -> Result<String> {
    let mut buf = String::new();
    let stdin = io::stdin();
    let mut handle = stdin.lock();
    handle.read_to_string(&mut buf)?;

    Ok(buf)
}

fn decode(input: &str) -> Result<String> {
    let decoded = percent_decode(input.as_bytes()).decode_utf8()?;
    Ok(decoded.to_string())
}

#[cfg(test)]
mod tests {
    use crate::decode;

    #[test]
    fn decode_space_ok() {
        let expected = "foo bar";
        let input = "foo%20bar";
        let actual = decode(input).unwrap();
        assert_eq!(actual, expected);
    }

    #[test]
    #[should_panic]
    fn decode_invalid_utf8_ng() {
        let input = "%93%FA%96%7B%8C%EA%0D%0A";
        decode(input).unwrap();
    }
}
