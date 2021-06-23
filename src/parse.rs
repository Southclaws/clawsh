#![allow(dead_code)]

use nom::{
    branch::{alt, permutation},
    bytes::complete::{is_a, tag, take_till, take_till1, take_until, take_while},
    character::complete::{anychar, char, multispace1},
    combinator::{eof, opt, value},
    error::{context, convert_error, ContextError, ErrorKind, ParseError, VerboseError},
    multi::{fold_many1, many0, many1},
    sequence::{preceded, terminated, tuple},
    take_while, Err, IResult,
};

use crate::procedure::{Filter, Procedure, Script};

// curl -XPOST -H'Content-Type: application/json' -d'{\"ip\":\"91.121.87.14:2182\"}' https://api.open.mp/server/

// fn invocation(input: &str) -> IResult<&str, &str> {
//     context(
//         "invocation",
//         tuple((
//             many0(terminated(
//                 procedure,
//                 permutation((
//                     stream_pipe,
//                     conditional_chain,
//                     js_open_pipe,
//                     js_close_pipe,
//                     jq_open_pipe,
//                     jq_close_pipe,
//                     foreach_open_pipe,
//                     foreach_close_pipe,
//                 )),
//             )),
//             opt(procedure),
//         )),
//     )(input)
// }

// fn pipe_command(input: &str) -> IResult<&str, &str> {
//     context(
//         "pipe_command",
//         alt((
//             command,
//             //
//             terminated(
//                 command,
//                 //
//                 opt(stream_pipe),
//             ),
//         )),
//     )(input)
// }

// #[test]
// fn test_pipe_command() {
//     let r = pipe_command("curl xyz.com/123/xyz?q=s&x=y");
//     println!("{:#?}", r);
// }

/// A procedure is a:
/// - Command invocation
/// - JavaScript pipe
/// - jq Expression
/// - Foreach expression
/// Further parsing of JS, jq, foreach is not done at this stage so this
/// combinator just captures anything.
// fn procedure(input: &str) -> IResult<&str, &str> {
//     context(
//         "procedure",
//         //
//         many0(anychar),
//     )(input)
//     .map(|(_, v)| v)
// }

fn procedure<T: Procedure>(i: &str) -> IResult<&str, Vec<(&str, T)>> {
    context(
        "procedure",
        //
        many1(alt((
            //
            // tuple((tag("|{"), js_pipe)), //
            // tuple((tag("|<"), jq_pipe)), //
            tuple((take_until("|{"), js_pipe)),
            tuple((take_until("|<"), jql_pipe)),
            // tuple((take_until("|("), js_pipe)),
            // tuple((take_until("|["), js_pipe)),
            // tuple((take_until("|"), js_pipe)),
            // tuple((take_until("&&"), js_pipe)),
        ))),
    )(i)
}

// #[test]
// fn test_procedure() {
//     let r = procedure("curl |{ js code }| manager |< .jq.expr >|");
//     println!("{:?}", r);
// }

/// JavaScript pipe
///
fn js_pipe<'a, E: ParseError<&'a str> + ContextError<&'a str>>(
    i: &'a str,
) -> IResult<&'a str, Script, E> {
    match preceded(
        context("Script procedure opening", tag("|{")),
        alt((
            terminated(
                //
                take_until("}|"),
                tag("}|"),
            ),
            take_till(|c| c == '\0'),
        )),
    )(i)
    {
        Ok(v) => Ok((
            "",
            Script {
                raw: String::from(v.1),
            },
        )),
        Err(e) => Err(e),
    }
}

#[test]
fn test_js_pipe() {
    let (_, r) = js_pipe::<(&str, ErrorKind)>("|{ () => '' }|").unwrap();
    assert_eq!(r.raw, " () => '' ");
}

#[test]
fn test_js_pipe_err() {
    let data = "| () => '' }|";
    let result = js_pipe::<VerboseError<&str>>(data);
    let error = result.unwrap_err();
    error.map(|e| println!("{}", convert_error(data, e)));
}

#[test]
fn test_js_pipe_without_terminator() {
    let (_, r) = js_pipe::<(&str, ErrorKind)>("|{ () => ''").unwrap();
    assert_eq!(r.raw, " () => ''");
}

/// JQL pipe
///
fn jql_pipe<'a, E: ParseError<&'a str> + ContextError<&'a str>>(
    i: &'a str,
) -> IResult<&'a str, Filter, E> {
    match preceded(
        context("JQL procedure opening", tag("|<")),
        alt((
            terminated(
                //
                take_until(">|"),
                tag(">|"),
            ),
            take_till(|c| c == '\0'),
        )),
    )(i)
    {
        Ok(v) => Ok((
            "",
            Filter {
                query: String::from(v.1),
            },
        )),
        Err(e) => Err(e),
    }
}

#[test]
fn test_jql_pipe() {
    let (_, r) = jql_pipe::<(&str, ErrorKind)>("|< () => '' >|").unwrap();
    assert_eq!(r.query, " () => '' ");
}

#[test]
fn test_jql_pipe_err() {
    let data = "| () => '' >|";
    let result = jql_pipe::<VerboseError<&str>>(data);
    let error = result.unwrap_err();
    error.map(|e| println!("{}", convert_error(data, e)));
}

#[test]
fn test_jql_pipe_without_terminator() {
    let (_, r) = jql_pipe::<(&str, ErrorKind)>("|< () => ''").unwrap();
    assert_eq!(r.query, " () => ''");
}
