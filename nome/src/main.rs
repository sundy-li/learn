extern crate nom;

use std::error::Error;

use nom::{
    branch::alt,
    bytes::complete::{is_not, tag, take_while1},
    character::{
        complete::{alpha1, char, digit1, multispace0, multispace1, one_of},
        is_alphabetic, is_digit,
    },
    combinator::{cut, map, map_res, opt},
    error::{context, VerboseError},
    multi::many0,
    sequence::{delimited, preceded, terminated, tuple},
    IResult, Parser,
};

#[derive(Debug)]
pub struct StatusLine {
    //状态行
    pub version: String,
    pub status: u16, //状态码
    pub msg: String, //状态消息
}

#[derive(Debug)]
pub struct HttpResponse {
    pub status_line: StatusLine,
    pub headers: Vec<(String, String)>,
    pub body: Vec<u8>,
}

impl HttpResponse {
    pub fn get_header(&self, name: &str) -> Option<&str> {
        self.headers
            .iter()
            .find(|header| header.0.eq(name))
            .map(|v| v.1.as_ref())
    }
}

fn ws(input: &str) -> IResult<&str, char> {
    one_of(" \t")(input)
}

pub fn parse_status_line(input: &str) -> IResult<&str, StatusLine> {
    match tuple((
        preceded(tag("HTTP/"), tuple((digit1, tag("."), digit1))), // 1.23
        preceded(ws, digit1),                                      // 200
        preceded(ws, is_not("\r\n")),                              // 200
    ))(input)
    {
        Err(e) => Err(e),
        Ok((rest, ((a, b, c), status, msg))) => {
            let status = status.to_string();
            let status = status.parse::<u16>().unwrap();

            let mut version_r: Vec<u8> = a.into();
            version_r.extend_from_slice(b.as_bytes());
            version_r.extend_from_slice(c.as_bytes());

            Ok((
                rest,
                StatusLine {
                    version: format!("{}{}{}", a, b, c),
                    status,
                    msg: msg.to_string(),
                },
            ))
        }
    }
}

// headers
// Bdpagetype: 2
// Bdqid: 0xc5fcbcd300117410
// Cache-Control: private
pub fn parse_headers(input: &str) -> IResult<&str, Vec<(String, String)>> {
    let name = terminated(is_not(":"), char(':'));
    let value = terminated(preceded(ws, is_not("\r\n")), tag("\r\n"));
    let kv = tuple((name, value));
    let headers: IResult<&str, Vec<(&str, &str)>> = many0(kv)(input);

    match headers {
        Err(e) => Err(e),
        Ok((rest, v)) => {
            let vec: Vec<(String, String)> = v
                .iter()
                .map(|(a, b)| (a.to_string(), b.to_string()))
                .collect();

            Ok((rest, vec))
        }
    }
}

fn main() {
    let line = "HTTP/1.2 200 OK\r\n";
    let ((msg, line)) = parse_status_line(line).unwrap();
    println!("{:?}", line);

    let headers = "a: b\r\nc: d\r\ne: f\r\n";
    let ((_, hs)) = parse_headers(headers).unwrap();
    println!("{:?}", hs);
}
