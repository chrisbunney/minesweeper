
pub mod minesweeper {

    use std::io;
    use std::cmp;
    use rand::Rng;

    #[derive(Debug, PartialEq)]
    enum State {
        Hidden,
        Visible(usize), // visible state will include number of surrounding mines
        Marker
    }

    struct Cell {
        state: State,
        is_mine: bool
    }
     
    pub struct Game {
        gridsize: usize,
        grid: Vec<Cell>,
        moves: usize,
        show_mines: bool,
    }

    impl Game {

        /// create a new Game object
        pub fn new(gridsize: usize, mines: usize) -> Game {
            let mut grid: Vec<Cell> = Vec::with_capacity(gridsize*gridsize);
            for _ in 0..gridsize*gridsize {
                grid.push(Cell{
                    state: State::Hidden,
                    is_mine: false
                });
            }

            let mut rng = rand::thread_rng();
            for _ in 0..mines {
                let i = rng.gen_range(0,gridsize*gridsize);
                grid[i].is_mine = true;
            };

            let g = Game {
                gridsize,
                grid,
                moves: 0,
                show_mines: false
            };

            g
        }
        
        /// count total number of mines surrounding cell
        fn count_nbrs(&self, i: usize, j: usize) -> usize {
            let mut cnt = 0;

            
            let i1: usize = (cmp::max(0, (i as i32)-1)) as usize;
            let i2: usize = cmp::min(self.gridsize-1, i+1);
            let j1: usize = (cmp::max(0, (j as i32)-1)) as usize;
            let j2: usize = cmp::min(self.gridsize-1, j+1);

            for ii in i1..=i2 {
                for jj in j1..=j2 {
                    let idx = jj * self.gridsize + ii;
                    if self.grid[idx].is_mine { cnt += 1; }
                }
            }
            cnt
        }

        /// show a printout of the grid to the terminal
        pub fn print_grid(&self) {      
            for row in 0..self.gridsize {
                let s = (0..self.gridsize)
                    .map(|i| {
                        if self.show_mines && self.grid[row*self.gridsize + i].is_mine {
                            return '*';
                        }

                        match self.grid[row*self.gridsize + i].state {
                            State::Hidden => 'X',
                            State::Visible(x) => {
                                if x > 0 {
                                    // need a better way to convert usize -> char...
                                    x.to_string().chars().nth(0).unwrap()
                                } else {
                                    ' '
                                }
                            },
                            State::Marker => '?'
                        }
                    }).collect::<String>();
                println!("{}",s);
            }
        }

        /// uncover the selected cell, and the neighbourhood.
        pub fn select_cell(&mut self, i: usize, j: usize) -> bool {
            self.moves += 1;

            let idx = j*self.gridsize + i;
            if self.grid[idx].is_mine {
                return false;
            } 
            
            self.update_cells(i, j);
            true
        }

        /// updates this cell and the surroundung neighbourhood
        fn update_cells(&mut self, i: usize, j: usize) {
            let cnt = self.count_nbrs(i, j);
            self.grid[j*self.gridsize+i].state = State::Visible(cnt);
            if cnt == 0 {
                let i1: usize = (cmp::max(0, (i as i32)-1)) as usize;
                let i2: usize = cmp::min(self.gridsize-1, i+1);
                let j1: usize = (cmp::max(0, (j as i32)-1)) as usize;
                let j2: usize = cmp::min(self.gridsize-1, j+1);
    
                for ii in i1..=i2 {
                    for jj in j1..=j2 {
                        match self.grid[jj*self.gridsize + ii].state {
                            State::Visible(_) => continue,
                            _ => self.update_cells(ii, jj)
                        }
                    }
                }
            }
        }

        /// runs that main game loop
        pub fn run(&mut self) -> bool {
            let mut input = String::new();
            loop {
                self.print_grid();

                // get user input:
                println!("Enter i,j: ");
                input.clear();
                io::stdin().read_line(&mut input).unwrap();
                let mut tmp = input.split_whitespace();
                let i = tmp.next().unwrap();
                let j = tmp.next().unwrap();

                let i = i.parse::<usize>().unwrap();
                let j = j.parse::<usize>().unwrap();

                println!("Mark (m) or reveal (r)?");
                input.clear();
                io::stdin().read_line(&mut input).unwrap();
                let mark = match input.to_lowercase().chars().nth(0) {
                    Some('m') => true,
                    Some('r') => false,
                    _ => continue
                };

                // update map:
                if mark {
                    self.grid[j*self.gridsize + i].state = State::Marker;
                } else {
                    if !self.select_cell(i, j) {
                        println!("BANG!!!");
                        return false;
                    }
                }
            }
        }
    }
} 

/*
fn run() {
    let g = minesweeper::Game::new(10);
    g.print_grid();
}
*/