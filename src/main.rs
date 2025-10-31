use iscrolib::candle_script::{
    Token,
    // Ast,
};

fn main() {
    // READ FILE AS Vec<char> (UTF_32!)
    let source_path = std::env::args().nth(1).unwrap_or_else(|| {
        // eprintln!("Please specify the path. - Dmitriy & Iscra (^_^)");
        // std::process::exit(-1);
        "candle-script-samples/index.cdl".to_string()
    });
    let candle_source = std::fs::read_to_string(
        source_path
    ).unwrap_or_else(|e| {
        eprintln!("Comptime error while reading file: {e}. - Dmitriy & Iscra (>_<)");
        std::process::exit(-1);
    })
        .chars()
        .collect::<Vec<_>>();

    // LEXING
    // println!("{0:?}", candle_source);
    println!("{0:#?}", Token::get_vec(&candle_source));
}
