use std::{path::PathBuf, process::Command};

#[cfg(all(feature = "static", feature = "dylib"))]
compile_error!("static and dylib are mutually exclusive and cannot be enabled together");

pub fn main() {
    println!("cargo:rerun-if-changed=DotnetLib/DotnetLib.csproj");
    println!("cargo:rerun-if-changed=DotnetLib/Library.cs");

    let dotnet_configuration = {
        let profile = std::env::var("PROFILE").unwrap();
        match profile.as_str() {
            "release" => "Release",
            _ => "Debug",
        }
    };

    let dotnet_out = PathBuf::from(std::env::var("OUT_DIR").unwrap()).join("DotnetLib");

    // TODO(Sirius902) Restore deps.
    let dotnet_publish = {
        let mut command = Command::new("dotnet");

        command
            .arg("publish")
            .arg("DotnetLib/DotnetLib.csproj")
            .arg("--use-current-runtime");

        #[cfg(feature = "dylib")]
        command.arg("-p:NativeLib=Shared");

        #[cfg(feature = "static")]
        command.arg("-p:NativeLib=Static");

        command
            .arg("-c")
            .arg(dotnet_configuration)
            .arg("-o")
            .arg(&dotnet_out);

        command.spawn().and_then(|mut child| child.wait())
    };

    if !dotnet_publish.is_ok_and(|status| status.success()) {
        panic!("failed to publish DotnetLib");
    }

    println!("cargo:rustc-link-search=native={}", dotnet_out.display());

    #[cfg(feature = "dylib")]
    link_dylib();

    #[cfg(feature = "static")]
    link_static();
}

#[cfg(feature = "dylib")]
fn link_dylib() {
    println!("cargo:rustc-link-lib=dylib=DotnetLib");
}

#[cfg(feature = "static")]
fn link_static() {
    let nuget_dir = dirs::home_dir()
        .expect("failed to find home dir")
        .join(".nuget");

    // NOTE(Sirius902) The version string must match the installed .NET compiler version.
    let ilcompiler_sdk = nuget_dir
        .join("packages")
        .join("runtime.win-arm64.microsoft.dotnet.ilcompiler")
        .join("9.0.0")
        .join("sdk");

    println!(
        "cargo:rustc-link-search=native={}",
        ilcompiler_sdk.display()
    );

    cc::Build::new()
        .object(ilcompiler_sdk.join("bootstrapperdll.obj"))
        .compile("bootstrapperdll");

    println!("cargo:rustc-link-lib=dylib=advapi32");
    println!("cargo:rustc-link-lib=dylib=bcrypt");
    println!("cargo:rustc-link-lib=dylib=ole32");
    println!("cargo:rustc-link-lib=dylib=oleaut32");

    println!("cargo:rustc-link-arg=-Wl,--whole-archive");
    println!("cargo:rustc-link-lib=static=bootstrapperdll");
    println!("cargo:rustc-link-arg=-Wl,--no-whole-archive");

    println!("cargo:rustc-link-lib=static=Runtime.WorkstationGC");
    println!("cargo:rustc-link-lib=static=System.Globalization.Native.Aot");
    println!("cargo:rustc-link-lib=static=System.IO.Compression.Native.Aot");
    println!("cargo:rustc-link-lib=static=eventpipe-disabled");
    println!("cargo:rustc-link-lib=static=zlibstatic");
    println!("cargo:rustc-link-lib=static=standalonegc-enabled");

    println!("cargo:rustc-link-lib=static=DotnetLib");
}
