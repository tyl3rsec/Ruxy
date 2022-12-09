fn main() {
    glib_build_tools::compile_resources(
        "src/resources",
        "src/resources/ruxy.gresource.xml",
        "ruxy.gresource"
    );
}
