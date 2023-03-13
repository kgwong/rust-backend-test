use rand::seq::SliceRandom;

use super::drawing::Drawing;



/***
 * Chooses n strokes from drawing, and creates a new drawing
 */
pub fn random(drawing: &Drawing, n: usize) -> Drawing {
    let mut rng = rand::thread_rng();

    drawing.choose_multiple(&mut rng, n).cloned().collect()
}