use winresource::WindowsResource;

fn main() {
    println!("cargo::rerun-if-changed=assets/vnc.ico");
    let mut resource = WindowsResource::new();
    resource
        .set_icon("assets/vnc.ico")
        .set("FileDescription", "VNC")
        .set("ProductName", "VNC");
    if let Err(error) = resource.compile_for("vnc") {
        println!("cargo::warning=the app icon is not embedded in vnc.exe: {error}");
    }
}
