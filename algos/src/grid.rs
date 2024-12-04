use std::fmt::{Display, Formatter};

#[derive(Debug, Clone)]
pub struct Grid<T> {
    pub data: Vec<T>,
    pub width: usize,
    pub height: usize,
}

impl Display for Grid<char> {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        let mut s = String::new();
        for y in 0..self.height {
            for x in 0..self.width {
                s.push(*self.at(x, y).unwrap());
            }
            s.push('\n');
        }
        write!(f, "{}", s)
    }
}

impl Display for Grid<&str> {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        let mut s = String::new();
        for y in 0..self.height {
            for x in 0..self.width {
                s.push_str(self.at(x, y).unwrap());
            }
            s.push('\n');
        }
        write!(f, "{}", s)
    }
}

impl Display for Grid<String> {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        let mut s = String::new();
        for y in 0..self.height {
            for x in 0..self.width {
                s.push_str(self.at(x, y).unwrap().as_str());
            }
            s.push('\n');
        }
        write!(f, "{}", s)
    }
}

impl Display for Grid<i64> {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        let mut s = String::new();
        for y in 0..self.height {
            for x in 0..self.width {
                s.push_str(&format!("{:^5}", *self.at(x, y).unwrap()));
            }
            s.push('\n');
        }
        write!(f, "{}", s)
    }
}

impl Display for Grid<bool> {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        let mut s = String::new();
        for y in 0..self.height {
            for x in 0..self.width {
                s.push_str(if *self.at(x, y).unwrap() { "#" } else { "." });
            }
            s.push('\n');
        }
        write!(f, "{}", s)
    }
}

#[allow(dead_code)]
impl<T: Default + Clone> Grid<T> {
    pub fn new(width: usize, height: usize) -> Self {
        let data = vec![<T as Default>::default(); width * height];
        return Self {
            data,
            width,
            height,
        };
    }

    pub fn at(&self, x: usize, y: usize) -> Option<&T> {
        if x >= self.width || y >= self.height {
            return None;
        }
        return Some(&self.data[y * self.width + x]);
    }

    pub fn set(&mut self, x: usize, y: usize, value: T) {
        if x >= self.width || y >= self.height {
            return;
        }
        self.data[y * self.width + x] = value;
    }

    pub fn fill(&mut self, value: T) {
        self.data.fill(value);
    }
}

pub struct LowerTriangularGrid<T> {
    pub data: Vec<T>,
    pub size: usize,
}

impl<T: Default + Clone> LowerTriangularGrid<T> {
    pub fn new(size: usize) -> Self {
        let data = vec![<T as Default>::default(); size * (size + 1) / 2];
        return Self { data, size };
    }

    pub fn at(&self, x: usize, y: usize) -> Option<&T> {
        if x > y || y >= self.size {
            return None;
        }
        let index = y * (y + 1) / 2 + x;
        return Some(&self.data[index]);
    }

    pub fn set(&mut self, x: usize, y: usize, value: T) {
        if x > y || y >= self.size {
            return;
        }
        let index = y * (y + 1) / 2 + x;
        self.data[index] = value;
    }
}

pub struct UpperTriangularGrid<T> {
    pub data: Vec<T>,
    pub size: usize,
}

impl<T: Default + Clone> UpperTriangularGrid<T> {
    pub fn new(size: usize) -> Self {
        let data = vec![<T as Default>::default(); size * (size + 1) / 2];
        return Self { data, size };
    }

    pub fn at(&self, x: usize, y: usize) -> Option<&T> {
        if x < y || x >= self.size {
            return None;
        }
        let index = x * self.size - x * (x + 1) / 2 + y;
        return Some(&self.data[index]);
    }

    pub fn set(&mut self, x: usize, y: usize, value: T) {
        if x < y || x >= self.size {
            return;
        }
        let index = x * self.size - x * (x + 1) / 2 + y;
        self.data[index] = value;
    }

    pub fn fill(&mut self, value: T) {
        self.data.fill(value);
    }
}

#[derive(Debug, Clone)]
pub struct DynamicGrid<T> {
    pub data: Vec<T>,
    pub width: usize,
    pub height: usize,
}

impl<T: Default + Clone> DynamicGrid<T> {
    pub fn new() -> Self {
        let data = Vec::new();
        return Self {
            data,
            width: 0,
            height: 0,
        };
    }

    pub fn with_capacity(width: usize, height: usize) -> Self {
        let data = Vec::with_capacity(width * height);
        return Self {
            data,
            width: 0,
            height: 0,
        };
    }

    pub fn fill(&mut self, value: T) {
        self.data.fill(value);
    }

    pub fn at(&self, x: usize, y: usize) -> Option<&T> {
        if x >= self.width || y >= self.height {
            return None;
        }
        return Some(&self.data[y * self.width + x]);
    }

    pub fn set(&mut self, x: usize, y: usize, value: T) {
        if x >= self.width || y >= self.height {
            return;
        }
        self.data[y * self.width + x] = value;
    }

    pub fn resize(&mut self, width: usize, height: usize) {
        let mut new_data = vec![<T as Default>::default(); width * height];
        for y in 0..std::cmp::min(self.height, height) {
            for x in 0..std::cmp::min(self.width, width) {
                new_data[y * width + x] = self.data[y * self.width + x].clone();
            }
        }
        self.data = new_data;
        self.width = width;
        self.height = height;
    }

    pub fn insert_row(&mut self, index: usize) {
        self.data.reserve(self.width);
        for _ in 0..self.width {
            self.data
                .insert(index * self.width, <T as Default>::default());
        }
        self.height += 1;
    }

    pub fn insert_column(&mut self, index: usize) {
        self.data.reserve(self.height);
        for y in 0..self.height {
            self.data.insert(
                index + y * (self.width + 1),
                <T as Default>::default(),
            );
        }
        self.width += 1;
    }

    pub fn insert_column_with(&mut self, index: usize, value: T) {
        self.data.reserve(self.height);
        for y in 0..self.height {
            self.data
                .insert(index + y * (self.width + 1), value.clone());
        }
        self.width += 1;
    }
}

impl<T: Default + Clone> Default for DynamicGrid<T> {
    fn default() -> Self {
        return Self::new();
    }
}
