use cursive::{
    theme::{BaseColor, Color, ColorStyle},
    vec::Vec2,
    Printer,
};

#[derive(Clone, Copy)]
pub struct Options {
    pub size: Vec2,
    pub acknowledge_cells: usize,
}

#[derive(Clone, Copy)]
pub enum Cell {
    Acknowledged,
    Pending,
    Empty,
}

pub struct AcknowledgementBoard {
    pub size: Vec2,
    pub cells: Vec<Cell>,
}

impl AcknowledgementBoard {
    pub fn new(options: Options) -> Self {
        let n_cells = options.size.x * options.size.y;

        let mut board = AcknowledgementBoard {
            size: options.size,
            cells: vec![Cell::Pending; n_cells],
        };

        let empty_cells = n_cells - options.acknowledge_cells;

        for (i, ref mut cell) in board.cells.iter_mut().enumerate() {
            if i > n_cells - empty_cells {
                **cell = Cell::Empty;
            }
        }

        board
    }
}

pub struct AcknowledgementBoardView {
    // Actual board, unknown to the player.
    board: AcknowledgementBoard,

    // Visible board
    overlay: Vec<Cell>,
}

impl AcknowledgementBoardView {
    pub fn new(options: Options) -> Self {
        let overlay = vec![Cell::Pending; options.size.x * options.size.y];
        let board = AcknowledgementBoard::new(options);
        AcknowledgementBoardView { board, overlay }
    }

    pub fn acknowledge(&mut self, acknowledge_id: usize) {
        self.overlay[acknowledge_id] = Cell::Acknowledged;
    }
}

impl cursive::view::View for AcknowledgementBoardView {
    fn draw(&self, printer: &Printer) {
        for (i, cell) in self.overlay.iter().enumerate() {
            let x = i % self.board.size.x;
            let y = i / self.board.size.x;

            let text = match *cell {
                Cell::Acknowledged => "■",
                Cell::Pending => "■",
                Cell::Empty => "■",
            };

            let color = match *cell {
                Cell::Acknowledged => Color::Rgb(0, 255, 0),
                Cell::Pending => Color::Rgb(255, 0, 0),
                Cell::Empty => Color::Rgb(0, 0, 255),
            };

            printer.with_color(
                ColorStyle::new(color, Color::Dark(BaseColor::Black)),
                |printer| printer.print((x, y), text),
            );
        }
    }

    fn required_size(&mut self, _: Vec2) -> Vec2 {
        self.board.size.map_x(|x| 2 * x)
    }
}
