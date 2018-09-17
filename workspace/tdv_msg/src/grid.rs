#[derive(Serialize, Deserialize, Debug)]
pub struct GridElement {
    id: String,
    row: u32,
    row_max: u32,
    col: u32,
    col_max: u32
}

/// https://developer.mozilla.org/en-US/docs/Web/CSS/CSS_Grid_Layout
#[derive(Serialize, Deserialize, Debug)]
pub struct GridLayout {
    columns: u32,
    elements: Vec<GridElement>,
}

pub struct Row<T>(T);
pub struct Col<T>(T);

impl<T: RowOrCol> Row<T> {
    fn value(&self) -> (u32, u32) { self.0.value() }
}

impl<T: RowOrCol> Col<T> {
    fn value(&self) -> (u32, u32) { self.0.value() }
}

pub trait RowOrCol {
    fn value(&self) -> (u32, u32);
}

impl RowOrCol for u32 {
    fn value(&self) -> (u32, u32) { (self.clone(), 0) }
}

impl RowOrCol for (u32, u32) {
    fn value(&self) -> (u32, u32) { self.clone() }
}

impl GridLayout {
    pub fn new(columns: u32) -> Self {
        Self {
            columns,
            elements: Vec::new(),
        }
    }

    pub fn add<T1, T2>(mut self, id: String, row: Row<T1>, col: Col<T2>) -> Self
        where T1: RowOrCol, T2: RowOrCol {
        let (row, row_max) = row.value();
        let (col, col_max) = col.value();
        self.elements.push( GridElement { id, row, row_max, col, col_max } );
        self
    }
}
