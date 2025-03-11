fn main() {
    println!("cargo:rustc-link-lib=framework=AVFoundation");
    println!("cargo:rustc-link-lib=framework=CoreVideo");
    println!("cargo:rustc-link-lib=framework=CoreMedia");
    println!("cargo:rustc-link-lib=framework=Foundation");

    // Set macOS deployment target
    println!("cargo:rustc-env=MACOSX_DEPLOYMENT_TARGET=10.15");

    // Compile Objective-C helper
    cc::Build::new()
        .file("objc_helper.m")
        .flag("-fobjc-arc")  // Use Automatic Reference Counting
        .compile("objc_helper");
}
