use std::str::FromStr;

use deno_core::error::bad_resource_id;
use deno_core::error::AnyError;
use deno_core::AsyncRefCell;
use deno_core::BufVec;
use deno_core::JsRuntime;
use deno_core::OpState;
use deno_core::RcRef;
use deno_core::Resource;
use deno_core::ZeroCopyBuf;
use tokio::io::AsyncReadExt;
use tokio::io::AsyncWriteExt;

#[derive(Debug)]
pub enum Procedure {
    Command(Command),
    Script(Script),
}

#[derive(Debug)]
pub enum Span {
    Text(String),
    Script(Script),
}

#[derive(Debug)]
pub struct Command {
    spans: Vec<Span>,
}

#[derive(Debug)]
pub struct CommandParseError;

impl FromStr for Command {
    type Err = CommandParseError;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let mut in_script_block = false;
        let mut escaped = false;
        let mut spans = Vec::new();
        let mut anchor = 0;
        for (i, c) in s.chars().enumerate() {
            if c == '\\' {
                escaped = true;
            }

            if !in_script_block {
                if c == '{' {
                    in_script_block = true;
                    spans.push(Span::Text(s[anchor..i].to_owned()));
                    anchor = i + 1;
                }
            } else {
                if c == '}' {
                    in_script_block = false;
                    spans.push(Span::Script(Script::new(s[anchor..i].to_owned())));
                    anchor = i + 1;
                }
            }
        }

        spans.push(Span::Text(s[anchor..s.len()].to_owned()));

        Ok(Command { spans })
    }
}

#[test]
fn test_parse_command() {
    let cmd = Command::from_str("curl https://some.site/{ (in) => in * 2 }/sub/route");
    println!("{:#?}", cmd);
}

#[derive(Debug)]
pub struct Script {
    raw: String,
}

impl Script {
    fn new(raw: String) -> Self {
        Script { raw }
    }

    fn evaluate(self) {
        let mut js_runtime = JsRuntime::new(Default::default());

        let runtime = tokio::runtime::Builder::new_current_thread()
            .enable_all()
            .build()
            .unwrap();

        let future = async {
            js_runtime.execute("clawsh_script", &self.raw).unwrap();
            js_runtime.run_event_loop().await
        };

        runtime.block_on(future).unwrap();
    }
} 

#[test]
fn test_eval() {
    let s = Script::new("console.log('hi')".into());
    s.evaluate();
}