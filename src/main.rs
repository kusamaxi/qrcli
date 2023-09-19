use qrcode_generator::QrCodeEcc;
use std::env;
use std::convert::TryInto;

fn generate_qr(content: &str, file_path: &str, size: u32) -> Result<(), Box<dyn std::error::Error>> {
    let size_usize = size.try_into().unwrap();
    qrcode_generator::to_png_to_file(content, QrCodeEcc::Low, size_usize, file_path)?;
    Ok(())
}

fn main() {
    let args: Vec<String> = env::args().collect();
    
    if args.len() != 3 {
        eprintln!("Usage: rust-qr-encoder <OUTPUT_FILE_PATH> <CONTENT>");
        return;
    }

    let output_file = &args[1];
    let content = &args[2];

    // Start with a reasonable size, and increase as needed.
    let mut size = 21;
    loop {
        match generate_qr(content, output_file, size) {
            Ok(_) => break,
            Err(e) if e.to_string().contains("too small") => {
                size += 1;
            },
            Err(e) => {
                eprintln!("Error generating QR code: {}", e);
                return;
            },
        }
    }
    println!("QR code generated with size: {}", size);
}
