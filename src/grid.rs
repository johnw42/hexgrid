pub struct HexGrid<T> {
    data: Vec<T>,
    width: usize,
    height: usize,
}

impl<T> HexGrid<T> {
    pub fn new(width: usize, height: usize) -> Self
    where
        T: Default,
    {
        let mut data = Vec::with_capacity(width * height);
        data.resize_with(width * height, T::default);
        HexGrid {
            data,
            width,
            height,
        }
    }

    pub fn get(&self, x: usize, y: usize) -> Option<&T> {
        todo!()
    }

    pub fn get_mut(&mut self, x: usize, y: usize) -> Option<&mut T> {
        todo!()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn it_works() {}
}
