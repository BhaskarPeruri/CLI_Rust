use clap::Parser;
use maud::{DOCTYPE, Markup, html};
use pulldown_cmark::{Options, Parser as MarkdownParser, html};
use std::{fs, path::PathBuf};



#[derive(Parser, Debug)]
struct Args{
    //Input markdown file path
    #[arg(long, short)]
    input:String,


    //Output HTML file path
    #[arg(long, short)]
    output: Option<String>,
    
}

fn render_html_page(content: &str) -> Markup{
    html!{
        (DOCTYPE)
        html{
            head{
                meta charset = "utf-8";
                title{
                    "Markdown to HTML Converter"
                }
            }
            body{
                (maud::PreEscaped(content.to_string()))
            }
        }
    }
}
fn main() {

    let args = Args::parse();
    let mardown_input = fs::read_to_string(args.input).expect("Unable to read file");

    let mut options = Options::empty();
    options.insert(Options::ENABLE_STRIKETHROUGH);

    let parser = MarkdownParser::new_ext(&mardown_input, options);

    let mut html_output = String::new();
    html::push_html(&mut html_output, parser);//get the html output

    let full_html_output = render_html_page(&html_output).into_string();

    match &args.output{
       Some(path)=>{
            fs::write(path, full_html_output).expect("Unable to write")
        },
        None => println!("Path not provided"),
    }
}




//cargo run  -- -i sample.md -o hmtlFile.html