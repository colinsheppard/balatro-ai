fn main() {
    // Tell Cargo to re-run this build script if pyproject.toml or Cargo.toml change
    println!("cargo:rerun-if-changed=pyproject.toml");
    println!("cargo:rerun-if-changed=Cargo.toml");
    
    // pyo3-build-config automatically handles Python detection when the python feature is enabled
    // It will automatically configure the linker when pyo3-build-config is in build-dependencies
    #[cfg(feature = "python")]
    pyo3_build_config::add_extension_module_link_args();
}
