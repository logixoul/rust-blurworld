pub struct Array2D<T> {
    pub width: i32,
    pub height: i32,
    pub data: Vec<T>,
}

impl<T> Array2D<T> where T:Default+Clone+Copy {
    pub fn new(width: i32, height: i32) -> Self {
        Self {
            width: width,
            height: height,
            data: vec![T::default(); (width * height) as usize],
        }
    }

    fn calc_index(self: &Self, x : i32, y: i32) -> i32 {
        return y * self.width + x;
    }

    pub fn set(self: &mut Self, x: i32, y: i32, value: T) {
        let idx = self.calc_index(x, y); // immutable borrow ends here
        self.data[idx as usize] = value; // mutable borrow happens after
    }

    pub fn get(self: &Self, x: i32, y: i32) -> T {
        let idx = self.calc_index(x, y);
        self.data[idx as usize]
    }
}
