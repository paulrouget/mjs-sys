use std::env;
use std::path::PathBuf;

fn main() {
  println!("cargo:rerun-if-changed=build.rs");
  println!("cargo:rerun-if-changed=mjs/mjs.c");
  println!("cargo:rerun-if-changed=mjs/mjs.h");

  // From mjs.h

  // #define CS_P_CUSTOM 0
  #[cfg(feature = "platform-custom")]
  let platform = Some("0");

  // #define CS_P_UNIX 1
  #[cfg(feature = "platform-unix")]
  let platform = Some("1");

  // #define CS_P_WINDOWS 2
  #[cfg(feature = "platform-windows")]
  let platform = Some("2");

  // #define CS_P_ESP32 15
  #[cfg(feature = "platform-esp32")]
  let platform = Some("15");

  // #define CS_P_ESP8266 3
  #[cfg(feature = "platform-esp8266")]
  let platform = Some("3");

  // #define CS_P_CC3100 6
  #[cfg(feature = "platform-cc3100")]
  let platform = Some("6");

  // #define CS_P_CC3200 4
  #[cfg(feature = "platform-cc3200")]
  let platform = Some("4");

  // #define CS_P_CC3220 17
  #[cfg(feature = "platform-cc3220")]
  let platform = Some("17");

  // #define CS_P_MSP432 5
  #[cfg(feature = "platform-msp432")]
  let platform = Some("5");

  // #define CS_P_TM4C129 14
  #[cfg(feature = "platform-tm4c129")]
  let platform = Some("14");

  // #define CS_P_MBED 7
  #[cfg(feature = "platform-mbed")]
  let platform = Some("7");

  // #define CS_P_WINCE 8
  #[cfg(feature = "platform-wince")]
  let platform = Some("8");

  // #define CS_P_NXP_LPC 13
  #[cfg(feature = "platform-nxp_lpc")]
  let platform = Some("13");

  // #define CS_P_NXP_KINETIS 9
  #[cfg(feature = "platform-nxp_kinetis")]
  let platform = Some("9");

  // #define CS_P_NRF51 12
  #[cfg(feature = "platform-nrf51")]
  let platform = Some("12");

  // #define CS_P_NRF52 10
  #[cfg(feature = "platform-nrf52")]
  let platform = Some("10");

  // #define CS_P_PIC32 11
  #[cfg(feature = "platform-pic32")]
  let platform = Some("11");

  // #define CS_P_RS14100 18
  #[cfg(feature = "platform-rs14100")]
  let platform = Some("18");

  // #define CS_P_STM32 16
  #[cfg(feature = "platform-stm32")]
  let platform = Some("16");

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
