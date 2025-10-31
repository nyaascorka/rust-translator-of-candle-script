use iscrolib::candle_script::Token;

fn main() {
    // READ FILE AS Vec<char> (UTF_32!)
    let candle_source = std::fs::read_to_string(
        "candle-script-samples/index.cdl"
    ).unwrap_or_else(|e| {
        eprintln!("Comptime error while reading file: {e}.");
        std::process::exit(-1);
    })
        .chars()
        .collect::<Vec<_>>();

    // LEXING
    // println!("{0:?}", candle_source);
    println!("{0:#?}", Token::get_vec(&candle_source));
}
