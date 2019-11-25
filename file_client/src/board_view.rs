use cursive::direction::Direction;
use cursive::event::{Event, EventResult, MouseButton, MouseEvent};
use cursive::theme::{BaseColor, Color, ColorStyle};
use cursive::vec::Vec2;
use cursive::views::{Button, Dialog, LinearLayout, Panel, SelectView};
use cursive::Cursive;
use cursive::Printer;

#[derive(Clone, Copy)]
pub struct Options {
    pub size: Vec2,
    pub acknowledge_cells: usize
}

#[derive(Clone, Copy)]
pub enum Cell {
    Acknowledged,
    Pending,
    Empty
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

        board
    }

    fn get_mut(&mut self, pos: Vec2) -> Option<&mut Cell> {
        self.cell_id(pos).map(move |i| &mut self.cells[i])
    }

    pub fn cell_id(&self, pos: Vec2) -> Option<usize> {
        if pos < self.size {
            Some(pos.x + pos.y * self.size.x)
        } else {
            None
        }
    }
}

pub struct AcknowledgementBoardView {
    // Actual board, unknown to the player.
    board: AcknowledgementBoard,

    // Visible board
    overlay: Vec<Cell>,

    focused: Option<Vec2>,
}

impl AcknowledgementBoardView {
    pub fn new(options: Options) -> Self {
        let overlay = vec![Cell::Pending; options.size.x * options.size.y];
        let board = AcknowledgementBoard::new(options);
        AcknowledgementBoardView {
            board,
            overlay,
            focused: None,
        }
    }

//    fn get_cell(&self, pos: Vec2) -> Option<Vec2> {
//        pos
//            .checked_sub(0)
//            .map(|pos| pos.map_x(|x| x / 2))
//            .and_then(|pos| {
//                if pos.fits_in(self.board.size) {
//                    Some(pos)
//                } else {
//                    None
//                }
//            })
//    }

    pub fn acknowledge(&mut self, acknowledge_id: usize) {
        self.overlay[acknowledge_id] = Cell::Acknowledged;
    }
}

impl cursive::view::View for AcknowledgementBoardView {
    fn draw(&self, printer: &Printer) {
        for (i, cell) in self.overlay.iter().enumerate() {
            let x = (i % self.board.size.x);
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