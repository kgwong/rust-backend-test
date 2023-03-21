use std::rc::Rc;

use rand::seq::SliceRandom;

use super::drawing::{Drawing, Stroke};



/***
 * Chooses n strokes from drawing + imprint, and returns it as a Drawing
 */
pub fn random(
    drawing: Option<Rc<Drawing>>,
    imprint: Option<Rc<Drawing>>,
    n: usize) -> Option<Rc<Drawing>> {
    let mut rng = rand::thread_rng();

    if drawing.is_some() && imprint.is_some() {
        let combined: &[Stroke] = &[&drawing.unwrap()[..], &imprint.unwrap()[..]].concat();
        Some(Rc::new(combined.choose_multiple(&mut rng, n).cloned().collect()))
    } else if drawing.is_some() {
        Some(Rc::new(drawing.unwrap().choose_multiple(&mut rng, n).cloned().collect()))
    } else if imprint.is_some() {
        Some(Rc::new(imprint.unwrap().choose_multiple(&mut rng, n).cloned().collect()))
    } else {
        None
    }
}