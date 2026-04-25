use cef::{args::Args, *};

#[cfg(all(target_os = "macos", feature = "sandbox"))]
fn cef_sandbox_library_available() -> bool {
  std::env::current_exe()
    .ok()
    .and_then(|exe| {
      exe.parent().map(|parent| {
        parent.join("../../../Chromium Embedded Framework.framework/Libraries/libcef_sandbox.dylib")
      })
    })
    .as_deref()
    .is_some_and(std::path::Path::is_file)
}

fn main() {
  let args = Args::new();

  #[cfg(all(target_os = "macos", feature = "sandbox"))]
  let _sandbox = cef_sandbox_library_available().then(|| {
    let mut sandbox = cef::sandbox::Sandbox::new();
    sandbox.initialize(args.as_main_args());
    sandbox
  });

  #[cfg(target_os = "macos")]
  let _loader = {
    let loader = library_loader::LibraryLoader::new(&std::env::current_exe().unwrap(), true);
    assert!(loader.load());
    loader
  };

  execute_process(
    Some(args.as_main_args()),
    None::<&mut App>,
    std::ptr::null_mut(),
  );
}

