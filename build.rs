use std::env;

fn main() {
    let sdk = match env::var("TARGET").unwrap().as_ref() {
        "aarch64-apple-darwin" | "x86_64-apple-darwin" => "macosx",
        "aarch64-apple-ios" => "iphoneos",
        "x86_64-apple-ios" | "aarch64-apple-ios-sim" => "iphonesimulator",
        x => panic!("unknown tripple: {}", x),
    };

    let framework = match sdk {
        "macosx" => "ios-arm64_x86_64-maccatalyst",
        "iphoneos" => "ios-arm64",
        "iphonesimulator" => "ios-arm64_x86_64-simulator",
        x => panic!("unknown sdk: {}", x),
    };

    println!("cargo:rustc-link-search=framework=./xcframeworks/ios_system.xcframework/{}", framework);
}
