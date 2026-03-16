fn main() {
	if std::env::var("CARGO_CFG_TARGET_OS").unwrap_or_default() == "windows" {
		let vcpkg = std::env::var("VCPKG_ROOT")
			.unwrap_or_else(|_| "C:/vcpkg".to_string());
		println!("cargo:rustc-link-search=native={vcpkg}/installed/x64-windows/lib");
	}
}
