fn main() {
    tonic_build::configure()
    .out_dir("src/server")
    .compile(
        &["protocols/knitter.proto", "protocols/account.proto"],
        &["protocols", "../cashmere_core/protocols"]
    ).unwrap();
}
