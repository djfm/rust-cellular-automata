use beryllium::*;

fn main() {
    let sdl = SDL::init(InitFlags::Everything).expect("couldn't start SDL");
    sdl.gl_set_attribute(SdlGlAttr::MajorVersion, 3).unwrap();
      sdl.gl_set_attribute(SdlGlAttr::MinorVersion, 3).unwrap();
      sdl.gl_set_attribute(SdlGlAttr::Profile, GlProfile::Core).unwrap();
      #[cfg(target_os = "macos")]
      {
        sdl
          .gl_set_attribute(SdlGlAttr::Flags, ContextFlag::ForwardCompatible)
          .unwrap();
      }

    let _win = sdl
    .create_gl_window(
      "Hello Window",
      WindowPosition::Centered,
      1600,
      900,
      WindowFlags::Shown,
    )
    .expect("couldn't make a window and context");

    'main_loop: loop {
        // handle events this frame
        while let Some(event) = sdl.poll_events().and_then(Result::ok) {
          match event {
            Event::Quit(_) => break 'main_loop,
            _ => (),
          }
        }
        // now the events are clear

        // here's where we could change the world state and draw.
      }
}
