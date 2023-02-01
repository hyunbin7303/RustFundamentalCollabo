
//Code from https://github.com/NishantJoshi00/cli_parse
//Testing purpose.

use std::env::args;

pub enum ArgType { // This is for storing the place holders Retrived after Parsing
    Int(i32),
    Str(String),
    None,
}

enum ArgType_ { // Private variable for templating
    Int,
    Str,
    None
}

struct Argument_ { // Private variable for templating
    name: String,
    nickname: Option<String>,
    value: ArgType_
}

pub struct Argument { // Struct containing arguments that are parsed and resolved
    pub name: String,
    pub value: ArgType
}

pub struct Parsed { // Final output of the CLI_PARSER
    pub binary: String,
    pub arguments: Vec<Argument>,
    pub command: Option<String>
}

fn get_type(name: String, template: &Vec<Argument_>, nick: bool) -> (Option<ArgType_>, String) {
    // This is a hidden function used to parse the arguments and return Them in the form of tuple
    for i in template {
        if nick {
            match &i.nickname {
                Some(val) => if val.clone() == name {
                    match &i.value {
                        ArgType_::Int => return (Some(ArgType_::Int), i.name.clone()),
                        ArgType_::Str => return (Some(ArgType_::Str), i.name.clone()),
                        ArgType_::None => return (Some(ArgType_::None), i.name.clone())
                    }
                },
                None => {}
            }
        } else {
            if i.name.clone() == name {
                match &i.value {
                    ArgType_::Int => return (Some(ArgType_::Int), i.name.clone()),
                    ArgType_::Str => return (Some(ArgType_::Str), i.name.clone()),
                    ArgType_::None => return (Some(ArgType_::None), i.name.clone())
                }
            }
        }
    }

    (None, name.clone())
}


fn from_tuple(template: Vec<(&str, Option<&str>, &str)>) -> Vec<Argument_> {
    // This for making templating easier for end developer
    let mut output: Vec<Argument_> = Vec::new();
    for i in template {
        output.push(Argument_ {
            name: i.0.to_owned(),
            nickname: match i.1 {
                Some(v) => Some(v.to_owned()),
                None => None
            },
            value: if i.2.to_lowercase() == "int" {
                ArgType_::Int
            } else if i.2.to_lowercase() == "str" {
                ArgType_::Str
            } else {
                ArgType_::None
            }
        })
    }
    output
}

pub fn parse_arguments(argv: Vec<String>, template: Vec<(&str, Option<&str>, &str)>) -> Parsed {
    let template = from_tuple(template);
    let binary = argv[0].clone();
    let mut arguments: Vec<Argument> = Vec::new();
    let mut skip: bool = false;
    let mut command: Option<String> = Option::None;
    let mut argtype: (Option<ArgType_>, String);
    for i in 1..argv.len() {
        if skip {
            skip = false;
            continue;
        }
        if argv[i].starts_with("--") && argv[i].len() > 2 {
            argtype = get_type(argv[i].clone(), &template, false);

        } else if  argv[i].starts_with("-") && argv[i].len() > 1 {
            argtype = get_type(argv[i].clone(), &template, true);

        } else {
            command = Some(argv[i].clone());
            continue;

        }
        match argtype.0 {
            Some(ty) => {
                let temp_arg = match ty {
                    ArgType_::Int => {
                        let val: i32 = match argv[i + 1].clone().trim().parse() {
                            Ok(val) => val,
                            Err(k) => panic!("{} while parsing {}", k, argv[i])
                        };
                        skip = true;
                        ArgType::Int(val)
                    },
                    ArgType_::Str => {
                        if argv[i + 1].clone().starts_with("-") || argv[i + 1].clone().starts_with("--") {
                            panic!("String expected but provided with : {}", argv[i + 1]);
                        } else {
                            skip = true;
                            ArgType::Str(argv[i + 1].clone())
                        }
                        
                    },
                    ArgType_::None => ArgType::None
                };
                arguments.push(Argument {
                    name: argtype.1.clone(),
                    value: temp_arg
                })
            }
            None => panic!("Invalid argument : {} was passed", i)
        }
    }
    Parsed {
        binary: binary,
        arguments: arguments,
        command: command
    }

}


fn main() {
    let args_vec : Vec<String> = args().collect();
    let mut list = vec![("bird", 10), ("cat", 20)];

   // parse_arguments(args_vec);
}
