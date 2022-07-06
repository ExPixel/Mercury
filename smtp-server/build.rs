fn main() {
    lalrpop::Configuration::new()
        .always_use_colors()
        .emit_rerun_directives(true)
        .set_out_dir(std::env::var("OUT_DIR").unwrap())
        .process_file("src/session/cmd_parser.lalrpop")
        .expect("failed to parser lalrpop files");
}
