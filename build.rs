use manage_define;

fn main() -> Result<(), Box<dyn std::error::Error>> {
    tonic_build::configure()
        .out_dir("src")
        .extern_path(".cashmere", "::manage_define::cashmere")
        .extern_path(".knitter_module", "::knitter_module::protocols")
        .build_client(false)
        .build_server(true)
        .type_attribute(
            "Reference",
            "#[derive(serde::Serialize, serde::Deserialize)]",
        )
        .compile_well_known_types(true)
        .compile(
            &["protocols/knitter.proto"],
            &[
                "protocols",
                "../cashmere_core/protocols",
                "../knitter_module/protocols",
            ],
        )?;

    // manage_define::utils::generate_manage_defines(
    //     &vec!["manage_defines"],
    //     "src/ids_codes",
    //     Some("dart_packges/knitter_id_codes/lib/src"),
    // );

    Ok(())
}
