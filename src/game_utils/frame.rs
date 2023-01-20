// To handle our frame we can have two choices
// 2D array
// Vectoooors

// As our map is quadrilateral, we will have one Vector to hold to rows and one to hold the cols

use super::{NUM_COLS, NUM_ROWS};

// The first element will be the rows, the seconds the columns
pub type Frame = Vec<Vec<&'static str>>;

// We also need to draw our frame
// We can create a new function for it
pub fn new_frame() -> Frame {
    // We need to create our Frame which is a vector of vector
    // The frame will be drawn from the top to the bottom --> so y axis
    // The first vector will be with the columns
    let mut cols = Vec::with_capacity(NUM_COLS);
    for _ in 0..NUM_COLS {
        let mut col = Vec::with_capacity(NUM_ROWS);
        for _ in 0..NUM_ROWS {
            col.push("X");
        }
        cols.push(col);
    }
    cols
}
// What do we need to draw ?

// Our frame has to be display somehow
// We can draw it, but we will have multiple items that we will need to draw, so we can create a trait
pub trait Drawable {
    fn draw(&self, frame: &Frame);
}
