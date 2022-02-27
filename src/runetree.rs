use std::path::Path;

use serde_derive::Deserialize;
use slint::{Image, ModelRc, VecModel};
use slint_league_runes_ui::*;

#[derive(Deserialize, Clone)]
pub struct RuneTreeDef {
    name: String,
    icon: String,
    keystones: Vec<RuneDef>,
    slots: Vec<Vec<RuneDef>>,
}

#[derive(Deserialize, Clone)]
pub struct RuneDef {
    name: String,
    icon: String,
}

impl Into<RuneTree> for RuneTreeDef {
    fn into(self) -> RuneTree {
        RuneTree {
            name: self.name.into(),
            icon: Image::load_from_path(Path::new(&self.icon)).unwrap(),
            keystones: ModelRc::new(VecModel::from(
                self.keystones
                    .into_iter()
                    .map(|r| r.into())
                    .collect::<Vec<_>>(),
            )),
            slots: ModelRc::new(VecModel::from(
                self.slots
                    .into_iter()
                    .map(|r| {
                        ModelRc::new(VecModel::from(
                            r.into_iter().map(|r| r.into()).collect::<Vec<_>>(),
                        ))
                    })
                    .collect::<Vec<_>>(),
            )),
        }
    }
}

impl Into<Rune> for RuneDef {
    fn into(self) -> Rune {
        Rune {
            name: self.name.into(),
            icon: Image::load_from_path(Path::new(&self.icon)).unwrap(),
        }
    }
}
