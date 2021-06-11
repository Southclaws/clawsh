use nom::{
    branch::{alt, permutation},
    bytes::complete::{tag, take_till, take_till1, take_until, take_while1},
    character::complete::{anychar, char, multispace1},
    combinator::opt,
    error::{context, ParseError},
    multi::{fold_many1, many0, many1},
    sequence::{terminated, tuple},
    IResult,
};

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

fn procedure(i: &str) -> IResult<&str, Vec<(&str, &str)>> {
    context(
        "procedure",
        //
        many1(alt((
            //
            // tuple((tag("|{"), js_pipe)), //
            // tuple((tag("|<"), jq_pipe)), //
            tuple((take_until("|{"), js_pipe)),
            tuple((take_until("|<"), jq_pipe)),
            // tuple((take_until("|("), js_pipe)),
            // tuple((take_until("|["), js_pipe)),
            // tuple((take_until("|"), js_pipe)),
            // tuple((take_until("&&"), js_pipe)),
        ))),
    )(i)
}

#[test]
fn test_procedure() {
    let r = procedure("curl |{ js code }| manager |< .jq.expr >|");
    println!("{:?}", r);
}

/// JavaScript pipe
///
fn js_pipe(i: &str) -> IResult<&str, &str> {
    context(
        "js_pipe",
        //
        take_until("}|"),
    )(i)
}

#[test]
fn test_js_pipe() {
    let (_, r) = js_pipe("|{ .test.obj }|").unwrap();
    assert_eq!(r, "|{ .test.obj");
}

/// JQ expression pipe
///
fn jq_pipe(i: &str) -> IResult<&str, &str> {
    context(
        "jq_pipe",
        //
        take_until(">|"),
    )(i)
}

#[test]
fn test_jq_pipe() {
    let (_, r) = jq_pipe("|< .test.obj >|").unwrap();
    assert_eq!(r, "|< .test.obj");
}
