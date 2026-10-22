use crate::PlanePool;

#[test]
fn test_a_returned_plane_buffer_serves_the_next_frame() {
    let pool = PlanePool::default();
    let mut first = pool.take(8);
    first[0] = 42;
    drop(first);

    let next = pool.take(4);

    assert_eq!(next.len(), 4, "the next frame gets the size it asks for");
    assert_eq!(
        next[0], 42,
        "the next frame reuses the memory of the returned one instead of a fresh allocation"
    );
}
