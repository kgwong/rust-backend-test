use std::{collections::HashMap, rc::Rc};

use rand::seq::SliceRandom;
use uuid::Uuid;

use super::drawing::Drawing;


/***
 * Assigns an imprint to a random player (including themselves)
 */
pub fn random(imprint_map: &HashMap<Uuid, Option<Rc<Drawing>>>)
    -> HashMap<Uuid, Option<Rc<Drawing>>> {

    let mut imprints: Vec<&Option<Rc<Drawing>>> = imprint_map.values().collect();
    imprints.shuffle(&mut rand::thread_rng());

    imprint_map.keys()
        .map(|k| k.clone())
        .zip(imprints.into_iter().cloned())
        .collect()
}