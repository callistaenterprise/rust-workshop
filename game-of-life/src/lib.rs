use rand::{thread_rng, Rng};

pub struct World {
    /// Cell at X,Y can be found at index `x+y*dim`
    cells: Vec<Cell>,
    dim: usize,
}

#[derive(Clone, Copy, PartialEq, Eq)]
enum Cell {
    Dead,
    Alive,
}

impl World {
    pub fn new(dim: usize) -> Self {
        let mut rnd = thread_rng();

        let cells = (0..dim * dim)
            .into_iter()
            .map(|_| rnd.gen::<bool>())
            .map(|b| if b { Cell::Alive } else { Cell::Dead })
            .take((dim * dim) as usize)
            .collect::<Vec<Cell>>();

        Self { cells, dim }
    }

    // Draw cells on screen
    pub fn draw(&self, screen: &mut [u8]) {
        for (cell, pix) in self.cells.iter().zip(screen.chunks_exact_mut(4)) {
            let color = if *cell == Cell::Alive {
                [0x0, 0x0, 0xff, 0xff]
            } else {
                [0, 0, 0x0, 0x0]
            };
            pix.copy_from_slice(&color);
        }
    }

    // Update the world
    //
    // Game of Life rules:
    // Any live cell with fewer than two live neighbours dies, as if by underpopulation.
    // Any live cell with two or three live neighbours lives on to the next generation.
    // Any live cell with more than three live neighbours dies, as if by overpopulation.
    // Any dead cell with exactly three live neighbours becomes a live cell, as if by reproduction.
    pub fn update(&mut self) {
        todo!("update the cells according to Game of Life rules")
    }
}
