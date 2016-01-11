



fn main() {}

struct Cell {
    status: CellStatus,
}

#[derive(Debug, Eq, PartialEq)]
enum CellStatus {
    Dead,
    Live,
}

struct CellNode {
    content: Cell,
    top: Option<Box<CellNode>>,
    left: Option<Box<CellNode>>,
    right: Option<Box<CellNode>>,
    bottom: Option<Box<CellNode>>,
    evolved: bool,
}

impl CellNode {
    fn new(status: CellStatus) -> Self {
        CellNode {
            content: Cell { status: status },
            top: None,
            left: None,
            right: None,
            bottom: None,
            evolved: false,
        }
    }

    fn evolve(mut self) -> Self {
    	
    	
    	if self.evolved {
    		return self;
    	}
    	
    	self.evolved = true;

        let mut neighbours = vec![];
        for v in self.top.iter() {
            neighbours.push(&v.content);
        }


        let next_content = self.content.evolve(&neighbours);

        CellNode {
            content: next_content,
            top: self.top.ma,
            left: self.left,
            right: self.right,
            bottom: self.bottom,
            evolved: false,
        }
    }
}

impl Cell {
    fn evolve(&self, neighbours: &Vec<&Cell>) -> Cell {

        let mut live_neighbours = 0;
        for neighbour in neighbours {
            match neighbour.status {
                CellStatus::Live => live_neighbours = live_neighbours + 1,
                CellStatus::Dead => {}
            }
        }

        if self.status == CellStatus::Live && (live_neighbours < 2 || live_neighbours > 3) {
            return Cell { status: CellStatus::Dead };
        }

        if self.status == CellStatus::Dead && live_neighbours == 3 {
            return Cell { status: CellStatus::Live };
        }

        let next_status = if self.status == CellStatus::Live {
            CellStatus::Live
        } else {
            CellStatus::Dead
        };

        return Cell { status: next_status };
    }
}


#[cfg(test)]
mod tests {

    use super::{Cell, CellStatus};

    #[test]
    fn it_should_be_dead_with_less_then_two_live_neighbours() {
        validate(CellStatus::Live, live_neighbours(1), CellStatus::Dead);
    }

    #[test]
    fn it_should_be_live_with_two_or_three_live_neighbours() {
        validate(CellStatus::Live, live_neighbours(2), CellStatus::Live);
        validate(CellStatus::Live, live_neighbours(3), CellStatus::Live);
    }

    #[test]
    fn it_should_be_dead_with_more_then_three_live_neighbours() {
        validate(CellStatus::Live, live_neighbours(4), CellStatus::Dead);
        validate(CellStatus::Live, live_neighbours(6), CellStatus::Dead);
    }

    #[test]
    fn it_should_be_live_with_three_live_neighbours() {
        validate(CellStatus::Dead, live_neighbours(2), CellStatus::Dead);
        validate(CellStatus::Dead, live_neighbours(3), CellStatus::Live);
        validate(CellStatus::Dead, live_neighbours(4), CellStatus::Dead);
    }

    fn validate(initial_cell_state: CellStatus,
                neighbours: Vec<Cell>,
                final_cell_state: CellStatus) {
        let intial_cell = Cell { status: initial_cell_state };

        let ref_neighbours = neighbours.iter().collect();

        let evolved_cell = intial_cell.evolve(&ref_neighbours);
        assert_eq!(final_cell_state, evolved_cell.status);
    }

    fn live_neighbours(amount: u32) -> Vec<Cell> {

        let mut live_neighbours = Vec::<Cell>::new();
        for _ in 0..amount {
            let live_cell = Cell { status: CellStatus::Live };
            live_neighbours.push(live_cell);
        }
        live_neighbours
    }



}
