#![allow(dead_code)]

use std::io::{self, BufReader, ErrorKind, Read, Write};
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
use serde_json::Value;
use tokio::io::AsyncReadExt;
use tokio::io::AsyncWriteExt;

#[derive(Debug)]
pub enum Procedure {
    Script(Script),
    Filter(Filter),
    Command(Command),
}

impl Procedure {
    pub fn process<R: Read, W: Write>(&self, r: R, w: W) -> Result<(), ()> {
        match self {
            Procedure::Script(p) => return p.process(r, w),
            Procedure::Filter(p) => return p.process(r, w),
            Procedure::Command(p) => return p.process(r, w),
        }
    }

    pub fn get_string(&self) -> String {
        match self {
            Procedure::Script(p) => return p.get_string(),
            Procedure::Filter(p) => return p.get_string(),
            Procedure::Command(p) => return p.get_string(),
        }
    }
}

#[derive(Debug)]
pub struct Script {
    pub raw: String,
}
impl Script {
    fn process<R: Read, W: Write>(&self, mut r: R, mut w: W) -> Result<(), ()> {
        // let mut buf = String::new();
        // if r.read_to_string(&mut buf).is_err() {
        //     return Some(ErrorKind::Other);
        // }

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

        return Ok(());
    }
    fn get_string(&self) -> String {
        return self.raw.clone();
    }
}

#[test]
fn test_js_eval() {
    let s = Script {
        raw: String::from("console.log('hi')"),
    };
    let r = BufReader::new("Hello!".as_bytes());
    s.process(r, io::stdout());
}

#[derive(Debug)]
pub struct Filter {
    pub query: String,
}
impl Filter {
    fn process<R: Read, W: Write>(&self, r: R, mut w: W) -> Result<(), ()> {
        let input: Value = serde_json::from_reader(r).unwrap();

        match jql::walker(&input, Some(&*self.query)) {
            Ok(result) => match w.write_all(&*result.to_string().as_bytes()) {
                Ok(_) => Ok(()),
                Err(_) => Err(()),
            },
            Err(e) => {
                println!("Oh no! {}", e);
                return Err(());
            }
        }
    }
    fn get_string(&self) -> String {
        return self.query.clone();
    }
}

#[test]
fn test_jql_eval() {
    let s = Filter {
        query: String::from("\"name\""),
    };
    let r = BufReader::new("{\"name\": \"Southclaws\", \"ignore\": \"me\"}".as_bytes());
    s.process(r, io::stdout());
}

#[derive(Debug)]
pub struct Command {
    pub raw: String,
}
impl Command {
    fn process<R: Read, W: Write>(&self, r: R, w: W) -> Result<(), ()> {
        return Ok(());
    }
    fn get_string(&self) -> String {
        return self.raw.clone();
    }
}
