use crate::model::data::TdmsPrimitive;

pub struct ChunkIterator<'a, T: TdmsPrimitive> {
    full_data: &'a [T],
    chunk_size: usize,
    current_index: usize,
}

impl<'a, T: TdmsPrimitive> ChunkIterator<'a, T> {
    pub fn new(full_data: &'a [T], chunk_size: usize) -> Self {
        Self {
            full_data,
            chunk_size: chunk_size.max(1),
            current_index: 0,
        }
    }
}

impl<'a, T: TdmsPrimitive> Iterator for ChunkIterator<'a, T> {
    type Item = &'a [T];

    fn next(&mut self) -> Option<Self::Item> {
        if self.current_index >= self.full_data.len() {
            return None;
        }

        let end = (self.current_index + self.chunk_size).min(self.full_data.len());
        let slice = &self.full_data[self.current_index..end];
        self.current_index = end;
        Some(slice)
    }
}
