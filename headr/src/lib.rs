use clap::{ App, Arg };
use std::error::Error;
use std::io;

type MyResult<T> = Result<T, Box<dyn Error>>;

#[derive(Debug)]
pub struct Config {
    files: Vec<String>,
    lines: usize,
    bytes: Option<usize>,
}

pub fn get_args() -> MyResult<Config> {
    let matches = App::new("headr")
        .version("0.1.0")
        .author("Ken Youens-Clark <kyclark@gmail.com>")
        .about("Rust head")
        .arg(
            Arg::with_name("files")
                .value_name("FILE")
                .default_value("-")
                .multiple(true),
        )
        .arg(
            Arg::with_name("lines")
                .short("n")
                .long("lines")
                .value_name("LINES")
                .takes_value(true),
        )
        .arg(
            Arg::with_name("bytes")
                .short("c")
                .long("bytes")
                .takes_value(true)
                .value_name("BYTES"),
        )
        .get_matches();

    // linesとbytesが指定されたら弾く処理
    if matches.is_present("lines") && matches.is_present("bytes") {
        return Err(From::from(
            "the argument '--lines <LINES>' cannot be used with '--bytes <BYTES>'",
        ));
    }

    let files = matches
        .values_of("files")
        .map(|vals| vals.map(String::from).collect())
        .unwrap_or_else(|| vec!["-".to_string()]);

    let lines_val = matches.value_of("lines").unwrap_or("10");
    let lines = parse_positive_int(lines_val).map_err(|_| {
        Box::new(io::Error::new(
            io::ErrorKind::InvalidInput,
            format!(
                "error: invalid value '{lines_val}' for '--lines <LINES>': \
invalid digit found in string"
            ),
        )) as Box<dyn Error>
    })?;

    let bytes = matches
        .value_of("bytes")
        .map(|val| {
            parse_positive_int(val).map_err(|_| {
                Box::new(io::Error::new(
                    io::ErrorKind::InvalidInput,
                    format!(
                        "error: invalid value '{val}' for '--bytes <BYTES>': \
invalid digit found in string"
                    ),
                )) as Box<dyn Error>
            })
        })
        .transpose()?;
    
    Ok(Config { files, lines, bytes })
}

pub fn run(config: Config) -> MyResult<()> {
    println!("{:#?}", config);
    Ok(())
}

// 自作
fn parse_positive_int_self(val: &str) -> MyResult<usize> {
    match val.parse::<usize>() {
        Ok(0) => Err(Box::new(io::Error::new(io::ErrorKind::InvalidInput, "0"))),
        Ok(n) => Ok(n),
        Err(_) => Err(Box::new(io::Error::new(io::ErrorKind::InvalidInput, val.to_string()))),
    }
}

// テキスト
fn parse_positive_int(val: &str) -> MyResult<usize> {
    match val.parse() {
        Ok(n) if n > 0 => Ok(n),
        _ => Err(From::from(val)),
    }
}

#[test]
fn test_parse_positive_int() {
    let res = parse_positive_int("3");
    assert!(res.is_ok());

    assert_eq!(res.unwrap(), 3);

    let res = parse_positive_int("foo");
    assert!(res.is_err());
    assert_eq!(res.unwrap_err().to_string(), "foo".to_string());

    let res = parse_positive_int("0");
    assert!(res.is_err());
    assert_eq!(res.unwrap_err().to_string(), "0".to_string());
}
