pub struct CircularBuffer<T> {
    data: Vec<Option<T>>,
    read_index: usize,
    write_index: usize,
}

#[derive(Debug, PartialEq)]
pub enum Error {
    EmptyBuffer,
    FullBuffer,
}

impl<T> CircularBuffer<T> {
    pub fn new(capacity: usize) -> Self {
        Self {
            data: (0..capacity).map(|_| None).collect(),
            read_index: 0,
            write_index: 0,
        }
    }

    pub fn write(&mut self, element: T) -> Result<(), Error> {
        if self.is_full() {
            return Err(Error::FullBuffer);
        }

        self.write_without_check(element);
        Ok(())
    }

    pub fn read(&mut self) -> Result<T, Error> {
        self.data[self.read_index]
            .take()
            .ok_or(Error::EmptyBuffer)
            .map(|value| {
                self.read_index = self.increase_index(self.read_index);
                value
            })
    }

    pub fn clear(&mut self) {
        self.data = (0..self.data.len()).map(|_| None).collect();
        self.read_index = 0;
        self.write_index = 0;
    }

    pub fn overwrite(&mut self, element: T) {
        let is_overwriting = self.is_full();
        self.write_without_check(element);
        
        if is_overwriting {
            self.read_index = self.increase_index(self.read_index);
        }
    }

    fn is_full(&self) -> bool {
        self.data[self.write_index].is_some()
    }

    fn increase_index(&self, index: usize) -> usize {
        (index + 1) % self.data.len()
    }

    fn write_without_check(&mut self, element: T) {
        self.data[self.write_index] = Some(element);
        self.write_index = self.increase_index(self.write_index);
    }
}
