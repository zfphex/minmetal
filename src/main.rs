use miniwin::*;

fn main() {
    let mut window = create_window("Demo", None, 800, 600, WindowStyle::Standard);

    let scale = window.scale_factor();
    let physical_width = (800.0 * scale) as usize;
    let physical_height = (600.0 * scale) as usize;

    let mut pixels = vec![0u32; physical_width * physical_height];
    let mut frame_count = 0;
    let mut running = true;

    // Platform-agnostic drawing logic
    let draw = |pixels: &mut Vec<u32>, w: usize, h: usize, frame: &mut usize| {
        if pixels.len() != w * h {
            pixels.resize(w * h, 0);
        }
        for y in 0..h {
            for x in 0..w {
                let r = ((x + *frame) & 0xFF) as u32;
                let g = ((y + *frame) & 0xFF) as u32;
                let b = (*frame & 0xFF) as u32;
                pixels[y * w + x] = 0xFF000000 | (r << 16) | (g << 8) | b;
            }
        }
        *frame += 2;
    };

    while running {
        // Unified event polling and resize-rendering hook
        window.draw(|win| {
            let scale = win.scale_factor();
            let (w, h) = win.content_size();
            let pw = (w as f64 * scale) as usize;
            let ph = (h as f64 * scale) as usize;
            draw(&mut pixels, pw, ph, &mut frame_count);
            win.update_buffer(&pixels, pw, ph);
        });

        // Unified event retrieval
        let mut events = Vec::new();
        while let Some(evt) = window.event() {
            events.push(evt);
        }

        for event in events {
            match event {
                Event::Quit
                | Event::CloseRequested
                | Event::KeyDown {
                    key: Key::Escape, ..
                } => {
                    running = false;
                }
                _ => {}
            }
        }

        window.wait_for_vsync();
    }
}
