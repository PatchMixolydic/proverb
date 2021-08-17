fn main() {
    println!("cargo:rerun-if-env-changed=PARCEL_INSTALL_PREFIX");
}
