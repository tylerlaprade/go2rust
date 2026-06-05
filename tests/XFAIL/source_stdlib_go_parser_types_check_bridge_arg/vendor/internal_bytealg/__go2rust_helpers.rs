
pub trait GoByteSequence: Clone {
    fn go_len(&self) -> usize;
    fn go_byte(&self, index: usize) -> u8;
    fn go_slice_to_string(&self, start: usize, end: Option<usize>) -> String;

    fn go_to_string(&self) -> String {
        self.go_slice_to_string(0, None)
    }
}

impl GoByteSequence for String {
    fn go_len(&self) -> usize {
        self.len()
    }

    fn go_byte(&self, index: usize) -> u8 {
        self.as_bytes()[index]
    }

    fn go_slice_to_string(&self, start: usize, end: Option<usize>) -> String {
        let end = end.unwrap_or_else(|| self.len());
        self[start..end].to_string()
    }

    fn go_to_string(&self) -> String {
        self.clone()
    }
}

impl GoByteSequence for Vec<u8> {
    fn go_len(&self) -> usize {
        self.len()
    }

    fn go_byte(&self, index: usize) -> u8 {
        self[index]
    }

    fn go_slice_to_string(&self, start: usize, end: Option<usize>) -> String {
        let end = end.unwrap_or_else(|| self.len());
        String::from_utf8(self[start..end].to_vec()).unwrap()
    }

    fn go_to_string(&self) -> String {
        String::from_utf8(self.clone()).unwrap()
    }
}
