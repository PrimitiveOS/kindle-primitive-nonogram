use std::{
    fs,
    path::{Path, PathBuf},
};

use picturelogic_render::View;
use primitive_app::{font_paths, frame, write_png, App, Change, HEIGHT, WIDTH};
use primitive_ui::{Canvas, Input};

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let output = PathBuf::from(
        std::env::args_os()
            .nth(1)
            .unwrap_or_else(|| "build/previews".into()),
    );
    fs::create_dir_all(&output)?;
    let fonts =
        Path::new(env!("CARGO_MANIFEST_DIR")).join("../../../doannt-kindle-launcher/assets/fonts");
    let mut canvas = Canvas::new(WIDTH, HEIGHT, &font_paths(&fonts))?;
    let mut app = View::preview();
    let puzzle = app.book.puzzle();
    for y in 0..puzzle.size() {
        for x in 0..puzzle.size() {
            if puzzle.filled(x, y) {
                assert_eq!(
                    app.input(Input::Tap(
                        280.0 + 208.0 * x as f32,
                        469.0 + 208.0 * y as f32
                    )),
                    Change::Partial
                );
            }
        }
    }
    assert!(app.book.solved());
    write_png(
        &output.join("picturelogic-solved.png"),
        WIDTH,
        HEIGHT,
        &frame(&app, &mut canvas, true),
    )?;
    for _ in 0..6 {
        assert_eq!(app.input(Input::PageForward), Change::Full);
    }
    assert_eq!(app.book.puzzle().size(), 10);
    let puzzle = app.book.puzzle();
    for y in 0..puzzle.size() {
        for x in 0..puzzle.size() {
            if puzzle.filled(x, y) && y < 5 {
                app.input(Input::Tap(
                    228.0 + 104.0 * x as f32,
                    417.0 + 104.0 * y as f32,
                ));
            }
        }
    }
    write_png(
        &output.join("picturelogic.png"),
        WIDTH,
        HEIGHT,
        &frame(&app, &mut canvas, false),
    )?;
    assert_eq!(app.input(Input::Tap(621.0, 1625.0)), Change::Full);
    assert_eq!(app.input(Input::Tap(834.0, 854.0)), Change::Full);
    assert!(!app.book.solved());
    assert_eq!(app.input(Input::Tap(1115.0, 90.0)), Change::Exit);
    Ok(())
}
