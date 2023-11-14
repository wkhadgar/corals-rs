use crate::cells::Cell;

struct KDtree {
    axis: usize,
    cell: Cell,
    left: Option<Box<KDtree>>,
    right: Option<Box<KDtree>>,
}

impl KDtree {
    pub fn new(cell: Cell, axis: usize) -> Box<Self> {
        Box::from(Self {
            axis,
            cell,
            left: None,
            right: None,
        })
    }
    pub fn add(&mut self, cell: Cell) {
        let new_axis = (self.axis + 1) % 2;
        if cell.position.get_components()[self.axis]
            < self.cell.position.get_components()[self.axis]
        {
            match &self.left {
                None => {
                    self.left = Some(Self::new(cell, new_axis));
                }
                Some(_) => self.left.as_mut().unwrap().add(cell),
            }
        } else {
            match &self.right {
                None => {
                    self.right = Some(Self::new(cell, new_axis));
                }
                Some(_) => self.right.as_mut().unwrap().add(cell),
            }
        }
    }
}
