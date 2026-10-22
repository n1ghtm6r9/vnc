use crate::{PlaneBuffer, PlanePool};

#[test]
fn test_the_pool_keeps_at_most_eight_spare_buffers() {
    let pool = PlanePool::default();
    let marked: Vec<PlaneBuffer> = (1..=10)
        .map(|marker| {
            let mut buffer = pool.take(1);
            buffer[0] = marker;
            buffer
        })
        .collect();
    drop(marked);

    let again: Vec<PlaneBuffer> = (0..10).map(|_| pool.take(1)).collect();
    let mut markers: Vec<u8> = again.iter().map(|buffer| buffer[0]).collect();
    markers.sort_unstable();

    assert_eq!(
        markers,
        [0, 0, 1, 2, 3, 4, 5, 6, 7, 8],
        "the pool keeps the first eight returned buffers and frees the rest"
    );
}
