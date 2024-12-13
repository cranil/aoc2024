use std::fmt::{Display, Formatter};

pub trait Grid<'a, T: 'a> {
    fn at(&self, x: usize, y: usize) -> Option<&T>;
    fn at_unchecked(&self, x: usize, y: usize) -> &T;
    fn set(&mut self, x: usize, y: usize, value: T);
    fn fill(&mut self, value: T);
    fn size(&self) -> (usize, usize);
    fn iter(&'a self) -> impl Iterator<Item = (usize, usize, &'a T)> + 'a;
}

#[derive(Debug, Clone)]
pub struct RectangularGrid<T> {
    pub data: Vec<T>,
    pub width: usize,
    pub height: usize,
}

impl Display for RectangularGrid<char> {
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

impl Display for RectangularGrid<&str> {
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

impl Display for RectangularGrid<String> {
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

impl Display for RectangularGrid<i64> {
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

impl Display for RectangularGrid<bool> {
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

impl<T: Default + Clone> RectangularGrid<T> {
    pub fn new(width: usize, height: usize) -> Self {
        let data = vec![<T as Default>::default(); width * height];
        Self {
            data,
            width,
            height,
        }
    }
}

impl<'a, T: Default + Clone + 'a> Grid<'a, T> for RectangularGrid<T> {
    fn at(&self, x: usize, y: usize) -> Option<&T> {
        if x >= self.width || y >= self.height {
            None
        } else {
            Some(&self.data[y * self.width + x])
        }
    }

    fn at_unchecked(&self, x: usize, y: usize) -> &T {
        &self.data[y * self.width + x]
    }

    fn set(&mut self, x: usize, y: usize, value: T) {
        if x >= self.width || y >= self.height {
            return;
        }
        self.data[y * self.width + x] = value;
    }

    fn fill(&mut self, value: T) {
        self.data.fill(value);
    }

    fn size(&self) -> (usize, usize) {
        (self.width, self.height)
    }

    fn iter(&'a self) -> impl Iterator<Item = (usize, usize, &'a T)> + 'a {
        (0..self.height).flat_map(move |y| {
            (0..self.width).map(move |x| {
                let index = y * self.width + x;
                (x, y, &self.data[index])
            })
        })
    }
}

pub struct LowerTriangularGrid<T> {
    pub data: Vec<T>,
    pub size: usize,
}

impl<T: Default + Clone> LowerTriangularGrid<T> {
    pub fn new(size: usize) -> Self {
        let data = vec![<T as Default>::default(); size * (size + 1) / 2];
        Self { data, size }
    }
}

impl<'a, T: Default + Clone + 'a> Grid<'a, T> for LowerTriangularGrid<T> {
    fn at(&self, x: usize, y: usize) -> Option<&T> {
        if x > y || y >= self.size {
            return None;
        }
        let index = y * (y + 1) / 2 + x;
        Some(&self.data[index])
    }

    fn at_unchecked(&self, x: usize, y: usize) -> &T {
        let index = y * (y + 1) / 2 + x;
        &self.data[index]
    }

    fn set(&mut self, x: usize, y: usize, value: T) {
        if x > y || y >= self.size {
            return;
        }
        let index = y * (y + 1) / 2 + x;
        self.data[index] = value;
    }

    fn fill(&mut self, value: T) {
        self.data.fill(value);
    }

    fn size(&self) -> (usize, usize) {
        (self.size, self.size)
    }

    fn iter(&'a self) -> impl Iterator<Item = (usize, usize, &'a T)> + 'a {
        (0..self.size).flat_map(move |y| {
            (0..y).map(move |x| {
                let index = y * (y + 1) / 2 + x;
                (x, y, &self.data[index])
            })
        })
    }
}

pub struct UpperTriangularGrid<T> {
    pub data: Vec<T>,
    pub size: usize,
}

impl<T: Default + Clone> UpperTriangularGrid<T> {
    pub fn new(size: usize) -> Self {
        let data = vec![<T as Default>::default(); size * (size + 1) / 2];
        Self { data, size }
    }
}

impl<'a, T: Default + Clone + 'a> Grid<'a, T> for UpperTriangularGrid<T> {
    fn at(&self, x: usize, y: usize) -> Option<&T> {
        if x < y || x >= self.size {
            return None;
        }
        let index = x * self.size - x * (x + 1) / 2 + y;
        Some(&self.data[index])
    }

    fn at_unchecked(&self, x: usize, y: usize) -> &T {
        let index = x * self.size - x * (x + 1) / 2 + y;
        &self.data[index]
    }

    fn set(&mut self, x: usize, y: usize, value: T) {
        if x < y || x >= self.size {
            return;
        }
        let index = x * self.size - x * (x + 1) / 2 + y;
        self.data[index] = value;
    }

    fn fill(&mut self, value: T) {
        self.data.fill(value);
    }

    fn size(&self) -> (usize, usize) {
        (self.size, self.size)
    }

    fn iter(&'a self) -> impl Iterator<Item = (usize, usize, &'a T)> + 'a {
        (0..self.size).flat_map(move |x| {
            (0..x).map(move |y| {
                let index = x * self.size - x * (x + 1) / 2 + y;
                (x, y, &self.data[index])
            })
        })
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
        Self {
            data,
            width: 0,
            height: 0,
        }
    }

    pub fn with_capacity(width: usize, height: usize) -> Self {
        let data = Vec::with_capacity(width * height);
        Self {
            data,
            width: 0,
            height: 0,
        }
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

    pub fn insert_row_with(&mut self, index: usize, value: T) {
        self.data.reserve(self.width);
        for _ in 0..self.width {
            self.data.insert(index * self.width, value.clone());
        }
        self.height += 1;
    }
}

impl<'a, T: Default + Clone + 'a> Grid<'a, T> for DynamicGrid<T> {
    fn fill(&mut self, value: T) {
        self.data.fill(value);
    }

    fn at(&self, x: usize, y: usize) -> Option<&T> {
        if x >= self.width || y >= self.height {
            return None;
        }
        Some(&self.data[y * self.width + x])
    }

    fn at_unchecked(&self, x: usize, y: usize) -> &T {
        &self.data[y * self.width + x]
    }

    fn set(&mut self, x: usize, y: usize, value: T) {
        if x >= self.width || y >= self.height {
            return;
        }
        self.data[y * self.width + x] = value;
    }

    fn size(&self) -> (usize, usize) {
        (self.width, self.height)
    }

    fn iter(&'a self) -> impl Iterator<Item = (usize, usize, &'a T)> + 'a {
        (0..self.height).flat_map(move |y| {
            (0..self.width).map(move |x| {
                let index = y * self.width + x;
                (x, y, &self.data[index])
            })
        })
    }
}

impl<T: Default + Clone> Default for DynamicGrid<T> {
    fn default() -> Self {
        Self::new()
    }
}
