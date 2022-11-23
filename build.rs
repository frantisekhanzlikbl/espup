fn main() {
    #[cfg(feature = "gui")]
    slint_build::compile("espup.slint").unwrap();
}
