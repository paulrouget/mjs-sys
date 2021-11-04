fn main() {
  let mut vm = mjs_sys::VM::create();

  let val = vm.exec(b"1 / 2\0").unwrap();
  if val.is_number() {
    println!("Result: {}", val.as_double().unwrap());
  }

  let val2 = vm.exec(b"\"foobar\"\0").unwrap();
  if val2.is_string() {
    println!("Result: {}", val2.as_str().unwrap().unwrap());
  }

  // Call JS function from Rust
  let mut js_function = vm
    .exec(
      b"
  function foobar(x) {
  return 42 + x;
  }
  foobar
  \0",
    )
    .unwrap();

  if js_function.is_function() {
    let this = None;
    let x = vm.make_number(10.);
    let args = &[&x];
    let res = js_function.call(this, args).unwrap();
    if res.is_number() {
      println!("Result: {}", res.as_double().unwrap());
    }
  }

  // Call Rust function from JS
  fn rust_function(mjs: *mut mjs_sys::mjs) {
    let mut vm = mjs_sys::VM::from_inner(mjs);
    let x = vm.arg(0).unwrap().as_int().unwrap();
    println!("JS -> Rust: {}", x);
  }

  let js_function = vm.make_foreign(rust_function as _);
  vm.global().set(b"rust", js_function).unwrap();
  vm.exec(b"rust(42)\0").unwrap();

  vm.destroy();
}
