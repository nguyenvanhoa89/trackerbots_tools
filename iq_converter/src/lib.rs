use std::iter::{Iterator, IntoIterator};

fn gen_lookup_table() -> [f32; 256] {
    let mut data = [0.0; 256];
    for i in 0..0x100 {
        data[i] = (i as i8) as f32 / 128.0;
    }
    data
}

pub struct IqConverter<Iter> {
    input: Iter,
    lookup_table: [f32; 256],
}

impl<Iter: Iterator<Item=u8>> IqConverter<Iter> {
    pub fn new<I: IntoIterator<IntoIter=Iter, Item=u8>>(input: I) -> IqConverter<Iter> {
        IqConverter {
            input: input.into_iter(),
            lookup_table: gen_lookup_table(),
        }
    }
}

impl<Iter: Iterator<Item=u8>> Iterator for IqConverter<Iter> {
    type Item = (f32, f32);

    fn next(&mut self) -> Option<Self::Item> {
        match (self.input.next(), self.input.next()) {
            (Some(i), Some(q)) => {
                Some((self.lookup_table[i as usize], self.lookup_table[q as usize]))
            }
            _ => None
        }
    }
}
