use cursive::{
    theme::{BaseColor, Color, ColorStyle},
    vec::Vec2,
    Printer,
};

#[derive(Clone, Copy)]
pub struct Options {
    pub size: Vec2,
    pub acknowledged_fields: usize,
}

#[derive(Clone, Copy)]
pub enum Cell {
    Acknowledged,
    Pending,
    NotSent,
    Empty,
}

#[derive(Clone)]
pub struct AcknowledgementBoard {
    pub size: Vec2,
    pub cells: Vec<Cell>,
}

impl AcknowledgementBoard {
    pub fn new(options: Options) -> Self {
        let n_cells = options.size.x * options.size.y;

        let mut board = AcknowledgementBoard {
            size: options.size,
            cells: vec![Cell::NotSent; n_cells],
        };

        let empty_cells = n_cells - options.acknowledged_fields;

        for (i, ref mut cell) in board.cells.iter_mut().enumerate() {
            if i >= n_cells - empty_cells {
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
        let board = AcknowledgementBoard::new(options);
        let overlay = board.cells.clone();
        AcknowledgementBoardView { board, overlay }
    }

    pub fn mark_sent(&mut self, packet_id: usize) {
        self.overlay[packet_id] = Cell::Pending;
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

            let text = "■";

            let color = match *cell {
                Cell::Acknowledged => Color::Rgb(0, 255, 0),
                Cell::NotSent => Color::Rgb(255, 0, 0),
                Cell::Pending => Color::Rgb(255, 255, 0),
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
