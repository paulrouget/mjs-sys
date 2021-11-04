use std::env;
use std::path::PathBuf;

fn main() {
  println!("cargo:rerun-if-changed=build.rs");
  println!("cargo:rerun-if-changed=mjs/mjs.c");
  println!("cargo:rerun-if-changed=mjs/mjs.h");

  // From mjs.h

  let platform = if cfg!(feature = "platform-custom") {
    // #define CS_P_CUSTOM 0
    Some("0")
  } else if cfg!(feature = "platform-unix") {
    // #define CS_P_UNIX 1
    Some("1")
  } else if cfg!(feature = "platform-windows") {
    // #define CS_P_WINDOWS 2
    Some("2")
  } else if cfg!(feature = "platform-esp32") {
    // #define CS_P_ESP32 15
    Some("15")
  } else if cfg!(feature = "platform-esp8266") {
    // #define CS_P_ESP8266 3
    Some("3")
  } else if cfg!(feature = "platform-cc3100") {
    // #define CS_P_CC3100 6
    Some("6")
  } else if cfg!(feature = "platform-cc3200") {
    // #define CS_P_CC3200 4
    Some("4")
  } else if cfg!(feature = "platform-cc3220") {
    // #define CS_P_CC3220 17
    Some("17")
  } else if cfg!(feature = "platform-msp432") {
    // #define CS_P_MSP432 5
    Some("5")
  } else if cfg!(feature = "platform-tm4c129") {
    // #define CS_P_TM4C129 14
    Some("14")
  } else if cfg!(feature = "platform-mbed") {
    // #define CS_P_MBED 7
    Some("7")
  } else if cfg!(feature = "platform-wince") {
    // #define CS_P_WINCE 8
    Some("8")
  } else if cfg!(feature = "platform-nxp_lpc") {
    // #define CS_P_NXP_LPC 13
    Some("13")
  } else if cfg!(feature = "platform-nxp_kinetis") {
    // #define CS_P_NXP_KINETIS 9
    Some("9")
  } else if cfg!(feature = "platform-nrf51") {
    // #define CS_P_NRF51 12
    Some("12")
  } else if cfg!(feature = "platform-nrf52") {
    // #define CS_P_NRF52 10
    Some("10")
  } else if cfg!(feature = "platform-pic32") {
    // #define CS_P_PIC32 11
    Some("11")
  } else if cfg!(feature = "platform-rs14100") {
    // #define CS_P_RS14100 18
    Some("18")
  } else if cfg!(feature = "platform-stm32") {
    // #define CS_P_STM32 16
    Some("16")
  } else {
    None
  };

  cc::Build::new()
    .file("mjs/mjs.c")
    .include("mjs/")
    .define("CS_PLATFORM", platform)
    .define("CS_ENABLE_STDIO", Some("0")) // Disable logs
    .compile("mjs");

  let bindings = bindgen::Builder::default()
    .header("mjs/mjs.h")
    .use_core()
    .ctypes_prefix("cty")
    .parse_callbacks(Box::new(bindgen::CargoCallbacks))
    .generate()
    .expect("Unable to generate bindings");

  let out_path = PathBuf::from(env::var("OUT_DIR").unwrap());
  bindings
    .write_to_file(out_path.join("mjs.rs"))
    .expect("Couldn't write bindings");
}
