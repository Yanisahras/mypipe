extern crate clap;
use clap::{Arg, App};
use std::process::Command;

fn main() {
    // recuperation of  --in and --out
    let matches = App::new("mypipe")
                          .version("1.0")
                          .author("Yanis AHRAS  <yanisahras96@gmail.com>")
                          .about("Pipeline")
                          .arg(Arg::with_name("input")
                               .short("i")
                               .long("in")
                               .value_name("in")
                               .help("The input of the pipe")
                               .takes_value(true)
                               
                            )
                          .arg(Arg::with_name("output")
                               .short("o")
                               .long("out")
                               .value_name("out")
                               .help("The output of the pipe")
                               .required(true)
                               .takes_value(true)
                            )
                          .get_matches();

    let arginput = matches.value_of("input").unwrap( );

    
    let argoutput = matches.value_of("output").unwrap( );
                
        
    let command_input_result = Command::new(arginput)
                            .stdout(Stdio::piped())
                            .spawn()
                            .expect("error to exec the arg");
                               

    let command_output_result = Command::new(argoutput)
                            .stdin(arg1_result.stdout.unwrap())
                            .output()
                            .expect("error to exec the arg");
                      
    println!("{}", String::from_utf8_lossy(&command_output.stdout).to_string());
    
}