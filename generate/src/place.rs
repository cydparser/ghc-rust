use crate::symbols;
use crate::symbols::{Place, Places};

impl Place {
    pub fn to_str(self) -> &'static str {
        match self {
            Place::Compiler => "compiler",
            Place::Docs => "docs",
            Place::Driver => "driver",
            Place::Libraries => "libraries",
            Place::Testsuite => "testsuite",
            Place::Utils => "utils",
        }
    }
}

impl Places {
    pub fn is_empty(self) -> bool {
        self.0 == 0
    }
}

pub struct PlacesIter {
    places: u32,
    index: usize,
}

impl Iterator for PlacesIter {
    type Item = Place;

    fn next(&mut self) -> Option<Self::Item> {
        match symbols::PLACE_VARIANTS.get(self.index) {
            Some(place) => {
                self.index += 1;

                if (*place as u32) & self.places != 0 {
                    Some(*place)
                } else {
                    None
                }
            }
            None => None,
        }
    }
}

impl IntoIterator for Places {
    type Item = Place;

    type IntoIter = PlacesIter;

    fn into_iter(self) -> Self::IntoIter {
        PlacesIter {
            places: self.0,
            index: 0,
        }
    }
}
