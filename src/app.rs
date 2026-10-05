pub enum GameState {
    Menu,
    Playing,
}

pub enum Selected
{
    Width,
    Height,
    Profile1,
    Profile2,
}

const NEIGHBORS: [(i32, i32); 8] =
[
    (-1, -1), (-1, 0), (-1, 1),
    ( 0, -1),          ( 0, 1),
    ( 1, -1), ( 1, 0), ( 1, 1),
];

pub struct App {
    pub state: GameState,
    pub cursor_x: usize,
    pub cursor_y: usize,
    pub start: bool,
    pub timer: u32,
    pub should_quit: bool,
    pub map_width: usize,
    pub map_height: usize,
    pub bombs: usize,
    pub located: Vec<usize>, //Numbers (and bombs (9))
    pub clicked: Vec<usize>, //Cells to show
    pub defeat: bool,
    pub ncells: u16, //this variable is used to check if the defeat is actually a win
    pub selection: Selected, //this enum is used in the menu part
}

impl App {
    pub fn new() -> Self {
        let map_width = 30;
        let map_height = 16;
        Self {
            state: GameState::Menu,
            cursor_x: 0,
            cursor_y: 0,
            start: false,
            timer: 0,
            should_quit: false,
            map_width,
            map_height,
            bombs: 0,
            located: vec![0; map_height*map_width],
            clicked: vec![0; map_height*map_width],
            defeat: false,
            ncells: 0,
            selection: Selected::Profile2,
        }
    }
    //Reset after a game (reset is called in main in GameState::Play after you get defeat
    pub fn reset(&mut self) {
        self.start = false;
        self.timer = 0;
        self.located = vec![0; self.map_height*self.map_width];
        self.clicked = vec![0; self.map_height*self.map_width];
        self.defeat = false;
        self.ncells = 0;
    }

    pub fn on_tick(&mut self) {
        if let GameState::Playing = self.state{
            if self.start && !self.defeat {self.timer += 1;}
        }
    }

    pub fn move_cursor(&mut self, dx: i16, dy: i16) {
        if let GameState::Playing = self.state {
            let max_x = self.map_width.saturating_sub(2);
            let max_y = self.map_height.saturating_sub(2);

            let new_x = (self.cursor_x as i16 + dx) as usize;
            let new_y = (self.cursor_y as i16 + dy) as usize;

            self.cursor_x = new_x.clamp(1, max_x);
            self.cursor_y = new_y.clamp(1, max_y);
        }
    }
    
    pub fn generate(&mut self) {
        self.located = vec![0; self.map_height*self.map_width];
        let mut count = 0;
        while count < self.bombs {
            let x = rand::random_range(1..=self.map_width.saturating_sub(2));
            let y = rand::random_range(1..=self.map_height.saturating_sub(2));
            //The bomb is placed only if there's not a bomb in that place and it's not near the player's cursor
            if self.located[y*self.map_width+x]!=9 && !((x == self.cursor_x || x+1 == self.cursor_x || x-1 == self.cursor_x) && (y == self.cursor_y || y+1 == self.cursor_y || y-1 == self.cursor_y))
            {
                self.located[y*self.map_width+x] = 9;
                count += 1;
            }
        }
        //Calculating the numbers
        for k in 1..=self.map_height.saturating_sub(2) {
            for j in 1..=self.map_width.saturating_sub(2) {
                if self.located[j+k*self.map_width as usize]!=9{
                    let mut bomb_count = 0;
                    if self.located[(j-1)+(k-1)*self.map_width] == 9{bomb_count+=1;}
                    if self.located[j+(k-1)*self.map_width] == 9{bomb_count+=1;}
                    if self.located[(j+1)+(k-1)*self.map_width] == 9{bomb_count+=1;}
                    if self.located[(j-1)+k*self.map_width] == 9{bomb_count+=1;}
                    if self.located[(j+1)+k*self.map_width] == 9{bomb_count+=1;}
                    if self.located[(j-1)+(k+1)*self.map_width] == 9{bomb_count+=1;}
                    if self.located[j+(k+1)*self.map_width] == 9{bomb_count+=1;}
                    if self.located[(j+1)+(k+1)*self.map_width] == 9{bomb_count+=1;}
                    self.located[j+k*self.map_width]=bomb_count;
                    if bomb_count!=0{self.ncells+=1;}
                }
            }
        }
    }
    pub fn gameover(&mut self)
    {
        //Show all the bombs in the map
        for k in 1..=self.map_height.saturating_sub(2) {
            for j in 1..=self.map_width.saturating_sub(2) {
                if self.located[j+k*self.map_width]==9
                {
                    self.clicked[j+k*self.map_width]=1;
                }
            }
        }
        self.defeat=true;
    }

    pub fn click(&mut self)
    { 
        if self.located[self.cursor_x+self.cursor_y*self.map_width]==9
        {
            self.gameover();
        }
        //not clicked
        else if self.clicked[self.cursor_x+self.cursor_y*self.map_width]==0
        {
            self.clicked[self.cursor_x+self.cursor_y*self.map_width]=1;
            //expand if the cell is numberless
            if self.located[self.cursor_x+self.cursor_y*self.map_width]==0
            {
                self.expand(self.cursor_x, self.cursor_y, false);
            }
            else
            {
                self.ncells-=1;
            }
        }
        //if already clicked then expand
        else if self.clicked[self.cursor_x+self.cursor_y*self.map_width]==1
        {
            let j = self.cursor_x;
            let k = self.cursor_y;
            //only if the flags respect the number
            if (self.clicked[(j-1)+(k-1)*self.map_width]==2) as u16 + (self.clicked[(j-1)+k*self.map_width]==2) as u16 + (self.clicked[(j-1)+(k+1)*self.map_width]==2) as u16 + (self.clicked[j+(k-1)*self.map_width]==2) as u16 + (self.clicked[j+(k+1)*self.map_width]==2) as u16 + (self.clicked[(j+1)+(k-1)*self.map_width]==2) as u16 + (self.clicked[(j+1)+k*self.map_width]==2) as u16 + (self.clicked[(j+1)+(k+1)*self.map_width]==2) as u16 == self.located[self.cursor_x+self.cursor_y*self.map_width] as u16
            {
                self.expand(self.cursor_x, self.cursor_y, true);
            }
        }
        if self.ncells==0
        {
            self.gameover();
        }
    }
    //flag placer
    pub fn click2(&mut self)
    {
        if self.start && self.bombs > 0 && self.clicked[self.cursor_x+self.cursor_y*self.map_width]==0
        {
            self.clicked[self.cursor_x+self.cursor_y*self.map_width]=2;
            self.bombs-=1;
        }
        else if self.start && self.clicked[self.cursor_x+self.cursor_y*self.map_width]==2
        {
            self.clicked[self.cursor_x+self.cursor_y*self.map_width]=0;
            self.bombs+=1;
        }
    }
    //it just works
    //the function check if a cell next to the called one is already clicked.
    //If not then it click it and if it's an empty cell then it calls another expand,
    //if there's a bomb is gameover
    //if there's a number it reduce the ncell variable(numbered cells) by 1 
    pub fn expand(&mut self, j: usize, k: usize, click: bool)
    {
        for (dx, dy) in NEIGHBORS
        {
            let nj = j as i32 + dx;
            let nk = k as i32 + dy;
            if nj < 1 || nj > self.map_width as i32 - 2 {continue;}
            if nk < 1 || nk > self.map_height as i32 - 2 {continue;}
            let address = nj as usize + nk as usize * self.map_width;
            let hidden = self.clicked[address] == 0;
            let flagged = self.clicked[address] == 2;

            if hidden || (!click && flagged)
            {
                self.clicked[address]=1;
                if self.located[address]==0 {self.expand(nj as usize, nk as usize, false);}
                else if self.located[address]==9 {self.gameover();}
                else {self.ncells = self.ncells.saturating_sub(1);}
            }
        }
    }
}
