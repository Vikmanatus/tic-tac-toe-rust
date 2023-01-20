// In here we can define all the utils required for our game
// What do we need for our game ?
mod frame;
mod render;
// We need a map
// The tic tac toe map is a square of 3x3
pub const NUM_ROWS: usize = 3;
pub const NUM_COLS: usize = 3;

// The map will need to be rendered, so we will need a render module to render things

// We will need a frame to hold the things which will be rendered, so we will need also a frame module

// The first item to implement is the frame in which we will render things