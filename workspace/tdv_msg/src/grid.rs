#[derive(Serialize, Deserialize, Debug)]
pub struct GridElement {
    pub id: String,
    pub row: u32,
    pub row_max: u32,
    pub col: u32,
    pub col_max: u32
}

/// https://developer.mozilla.org/en-US/docs/Web/CSS/CSS_Grid_Layout
#[derive(Serialize, Deserialize, Debug)]
pub struct GridLayout {
    pub n_columns: u32,
    pub elements: Vec<GridElement>,
}

pub struct Row<T>(pub T);
pub struct Col<T>(pub T);

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
    pub fn new(n_columns: u32) -> Self {
        Self {
            n_columns,
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
