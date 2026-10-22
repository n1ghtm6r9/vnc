use super::CountingReader;

impl<R> CountingReader<R> {
    pub(crate) fn new(inner: R) -> CountingReader<R> {
        CountingReader { inner, count: 0 }
    }
}
