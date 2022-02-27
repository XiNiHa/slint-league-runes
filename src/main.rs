use std::{fs, path::Path, collections::HashMap};

use runetree::RuneTreeDef;
use slint_league_runes_ui::*;

mod runetree;

fn main() {
    let app = App::new();

    let runetree: HashMap<String, RuneTreeDef> =
        toml::from_str(&fs::read_to_string(Path::new("runetree.toml")).unwrap()).unwrap();

    app.set_main_rune_tree(runetree.get("precision").unwrap().clone().into());
    app.set_sub_rune_tree(runetree.get("domination").unwrap().clone().into());

    app.run();
}
