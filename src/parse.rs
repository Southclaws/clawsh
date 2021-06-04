use nom::{
    branch::{alt, permutation},
    bytes::complete::{tag, take_till, take_till1, take_while1},
    character::complete::{anychar, char, multispace1},
    combinator::opt,
    error::context,
    multi::{fold_many1, many0},
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

/// Open and close separators for pipe sections.

fn stream_pipe(input: &str) -> IResult<&str, char> {
    context(
        "stream_pipe",
        //
        char('|'),
    )(input)
}

fn conditional_chain(input: &str) -> IResult<&str, &str> {
    context(
        "conditional_chain",
        //
        tag("&&"),
    )(input)
}

fn js_pipe(i: &str) -> IResult<&str, &str> {
    let (i, _) = multispace1(i)?;
    let (i, _) = tag("|<")(i)?;

    let (i, elements) = fold_many1(
        tuple((multispace1, take_till(">|"), multispace1)),
        Vec::new(),
        |mut acc, (_, thing, _)| {
            acc.push(Box::new(thing));
            acc
        },
    )(i)?;
    let (i, _) = tuple((multispace1, tag(">|")))(i)?;

    // context(
    //     "js_pipe",
    //     //
    //     ,
    // )(input)
}

#[test]
fn test_js_pipe() {
    js_pipe("|< .test.obj >|")
}

// fn js_open_pipe(input: &str) -> IResult<&str, &str> {
//     context(
//         "js_open_pipe",
//         //
//         tag("|{"),
//     )(input)
// }

// fn js_close_pipe(input: &str) -> IResult<&str, bool> {
//     context(
//         "js_close_pipe",
//         //
//         tag("}|"),
//     )(input)
// }

// fn jq_open_pipe(input: &str) -> IResult<&str, bool> {
//     context(
//         "jq_open_pipe",
//         //
//         tag("|<"),
//     )(input)
// }

// fn jq_close_pipe(input: &str) -> IResult<&str, bool> {
//     context(
//         "jq_close_pipe",
//         //
//         tag(">|"),
//     )(input)
// }

// fn foreach_open_pipe(input: &str) -> IResult<&str, bool> {
//     context(
//         "foreach_open_pipe",
//         //
//         tag("|["),
//     )(input)
// }

// fn foreach_close_pipe(input: &str) -> IResult<&str, bool> {
//     let (input, o) = context(
//         "foreach_close_pipe",
//         //
//         tag("]|"),
//     )(input);
//     Ok((input, o == "]|"))
// }
