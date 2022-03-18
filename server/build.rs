use std::path::PathBuf;

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let out_dir = PathBuf::from(concat!(env!("CARGO_MANIFEST_DIR"), "/static"));
    // println!("cargo:warning={:?}", out_dir);

    tonic_build::configure()
        .build_client(false)
        .build_server(false)
        // .file_descriptor_set_path(out_dir.join("order_descriptor.bin"))
        .out_dir(out_dir)
        .compile(
            &[
                "proto/order.proto",
                "proto/trade.proto",
                "proto/request.proto",
                "proto/orderbook.proto",
            ],
            &["proto"],
        )?;

    // println!("cargo:warning=built");

    Ok(())
}
