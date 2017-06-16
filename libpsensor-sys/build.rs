extern crate gcc;

static LIBPSENSOR_SRC: &str = "psensor-1.2.0/src/lib/";
static LIBPSENSOR_SOURCES: [&str; 14] = [
    "color.c",
    "hdd_hddtemp.c",
    "measure.c",
    "plog.c",
    "pmutex.c",
    "psensor.c",
    "ptime.c",
    "pio.c",
    "slog.c",
    "temperature.c",
    "url.c",
    // OPTION
    "lmsensor.c",
    "nvidia.c",
    "pudisks2.c",
];

fn main() {
    let mut config = gcc::Config::new();
    for source in &LIBPSENSOR_SOURCES {
        config.file(format!("{}{}", LIBPSENSOR_SRC, source));
    }
    // pudisks2.c
    config
        .include("/usr/include/udisks2")
        .include("/usr/include/glib-2.0")
        .include("/usr/lib/x86_64-linux-gnu/glib-2.0/include");
    config
        .file("wrapper.c")
        .include(".")
        .include(LIBPSENSOR_SRC)
        .compile("libpsensor.a");
    // lmsensor.c
    println!("cargo:rustc-link-lib=sensors");
    // nvidia.c
    println!("cargo:rustc-link-lib=XNVCtrl");
    println!("cargo:rustc-link-lib=X11");
    println!("cargo:rustc-link-lib=Xext");
    // pudisks2.c
    println!("cargo:rustc-link-lib=udisks2");
    println!("cargo:rustc-link-lib=gio-2.0");
    println!("cargo:rustc-link-lib=gobject-2.0");
    println!("cargo:rustc-link-lib=glib-2.0");
}
