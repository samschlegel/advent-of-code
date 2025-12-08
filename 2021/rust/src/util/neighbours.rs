use ndarray::Array2;

pub trait Array2Ext<T> {
    fn neighbors4(&self) -> ();
}

impl<T> Array2Ext<T> for Array2<T> {
    fn neighbors4(&self) -> () {
        todo!()
        // let (width, height) = self.dim();
        // self.view()
        //     .indexed_iter()
        //     .map(|((x, y), value)| {
        //         let neighbor_indices = []
        //         ((x, y), value)
        //     });
        // let neighbours = board.slice(s![x.saturating_sub(1)..=x + 1, y.saturating_sub(1)..=y + 1]);
    }
}
