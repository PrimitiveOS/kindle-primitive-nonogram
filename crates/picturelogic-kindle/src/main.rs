use std::path::PathBuf;

use picturelogic_render::View;

fn main() -> Result<(), String> {
    let directory = std::env::var_os("PRIMITIVE_APP_DIR")
        .map_or_else(|| PathBuf::from("/mnt/us/picturelogic"), PathBuf::from);
    primitive_app::run(View::open(directory.join("picturelogic.json")), &directory)
}
