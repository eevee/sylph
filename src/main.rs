#[macro_use]
extern crate nom;

use std::io::{self, Read};
use std::str;

use nom::{IResult, alphanumeric};

struct StringLiteral<'a> {
    value: &'a str,
}

struct FunctionCall<'a> {
    function_name: &'a str,
    argument: StringLiteral<'a>,
}


// Parser stuff
named!(string_literal<&[u8], StringLiteral>,
    do_parse!(
        value: map_res!(
            delimited!(tag!("\""), take_until!("\""), tag!("\"")),
            str::from_utf8
        ) >>
        (StringLiteral{value: value})
    )
);

named!(function_call<&[u8], FunctionCall>,
    do_parse!(
        name: map_res!(alphanumeric, str::from_utf8) >>
        tag!("(") >>
        arg: string_literal >>
        tag!(")") >>
        (FunctionCall{ function_name: name, argument: arg })
    )
);

fn print(argument: &StringLiteral) {
    println!("{}", argument.value);
}

fn main() {
    let mut buffer = String::new();
    io::stdin().read_to_string(&mut buffer);
    let result = function_call(buffer.as_bytes());
    match result {
        IResult::Done(_leftovers, call) => {
            match call.function_name {
                "print" => {
                    print(&call.argument);
                }
                _ => panic!("oh no no such function {}", call.function_name),
            }
        }
        IResult::Incomplete(_needed) => {
            println!("early termination what");
        }
        IResult::Error(err) => {
            println!("boom!  {:?}", err);
        }
    }
}
