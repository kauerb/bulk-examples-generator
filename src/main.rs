use bulk_examples_generator::compile_grammar;
use bulk_examples_generator::config::{ExecutorConfig, GeneratorConfig};
use bulk_examples_generator::generate_examples;

use std::fs::File;
use std::io::prelude::*;
use std::io::Error;
use std::path::PathBuf;
use structopt::StructOpt;

#[derive(StructOpt, Debug)]
/// Generate massive amounts of random examples based in a PEST grammar, it can be used like a grammar fuzzer
///
/// Examples of use:
///
/// bulk-examples-generator --grammar my-grammar.pest --quantity 5 --start-rule myrule --out-type stdout
///
/// Shortened
///
/// bulk-examples-generator -g my-grammar.pest -q 5 -s myrule -o stdout
#[structopt(name = "bulk-examples-generator")]
pub struct Opt {
    /// Path of grammar for generate examples
    #[structopt(short, long, parse(from_os_str))]
    pub grammar: PathBuf,

    /// Quantity of examples to generate
    #[structopt(short, long)]
    pub quantity: u32,

    /// Rule to start generation of examples
    #[structopt(short, long)]
    pub start_rule: String,

    /// Where to write the examples (multiples values can be used) debug, stdout, text, bar, folder
    ///
    /// debug: Print results in stdout (vec form) for debugging purposes
    /// stdout: Print results in stdout
    /// text: Print "Example #n generated:" before print the example
    /// bar: Print progress bar
    /// folder: Create one file for each example (use template_name for personalize the filename and output_folder)
    ///
    #[structopt(short, long, verbatim_doc_comment)]
    pub out_type: Vec<String>,

    // TODO: is necessary implement this?
    // /// file: Save all examples in a single file
    // #[structopt(required_if("out_type", "file"), parse(from_os_str))]
    // pub output_file: Option<PathBuf>,
    /// Output folder to save the examples
    #[structopt(long, parse(from_os_str))]
    pub output_folder: Option<PathBuf>,

    /// Name of the files, e.g. html-test-{}.html, {} will be used for enumerating the example
    #[structopt(short, long, default_value = "example-{}.txt")]
    pub template_name: String,

    /// Config file for generate elements, for more details pleaser refer to README
    /// Default config available in src/config/default.toml
    #[structopt(short, long, parse(from_os_str))]
    pub config_file: Option<PathBuf>,

    #[structopt(long)]
    /// Disable parallel mode
    pub sequential: bool,
}

fn main() -> Result<(), Error> {
    let opt = Opt::from_args();

    let mut gen_config: GeneratorConfig = Default::default();
    if let Some(config_file) = &opt.config_file {
        gen_config = GeneratorConfig::new(config_file.to_str().unwrap()).unwrap();
    }

    let mut exe_config: ExecutorConfig = Default::default();
    exe_config.print_stdout = false;
    exe_config.parallel_mode = !opt.sequential;
    // if let Some(config_file) = &opt.config_file {
    //     config = GeneratorConfig::new(config_file.to_str().unwrap()).unwrap();
    // }

    // Load grammar file
    let mut grammar_string = String::new();
    let mut f = File::open(&opt.grammar)?;
    f.read_to_string(&mut grammar_string)?;

    if opt.out_type.contains(&"debug".to_string()) {
        // Print input parameters
        println!("{:?}", &opt);

        // print the vector
        exe_config.print_debug = true;
        exe_config.return_vec = true;

        // Print grammar
        let g = compile_grammar(grammar_string.clone());
        println!("{:?}", g);
    }

    if opt.out_type.contains(&"stdout".to_string()) {
        exe_config.print_stdout = true;
    }
    if opt.out_type.contains(&"text".to_string()) {
        exe_config.print_stdout = false;
        exe_config.print_progress_text = true;
    }
    if opt.out_type.contains(&"bar".to_string()) {
        exe_config.print_progress_bar = true;
    }
    if opt.out_type.contains(&"folder".to_string()) {
        // Output folder
        exe_config.print_folder = Some((opt.template_name, opt.output_folder.unwrap()));
    }

    // Generating examples
    let results = generate_examples(
        grammar_string,
        opt.quantity,
        opt.start_rule,
        &gen_config,
        &exe_config,
    );

    if opt.out_type.contains(&"debug".to_string()) {
        // Print vec
        println!("{:?}", results);
    }

    Ok(())
}
