use std::env;

fn main() {
    let sdk = match env::var("TARGET").unwrap().as_ref() {
        "aarch64-apple-darwin" | "x86_64-apple-darwin" => Some("macosx"),
        "aarch64-apple-ios" => Some("iphoneos"),
        "x86_64-apple-ios" | "aarch64-apple-ios-sim" => Some("iphonesimulator"),
        _ => None,
    };

    let framework = match sdk {
        Some("macosx") => None, //"ios-arm64_x86_64-maccatalyst",
        Some("iphoneos") => Some("ios-arm64"),
        Some("iphonesimulator") => Some("ios-arm64_x86_64-simulator"),
        _ => None,
    };

    if let Some(framework) = framework {
        println!("cargo:rustc-link-searich=framework=./xcframeworks/ios_system.xcframework/{}", framework);
    }
}
