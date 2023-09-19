use algos::data_structures::array::*;
#[test]
fn test_ring_buffer() {
    let mut rb = RingBuffer::new(100);
    for i in 0..10 {
        for j in 0..57 {
            rb.push(57 * i + j).unwrap();
        }
        dbg!(&rb);
        for j in 0..57 {
            assert_eq!(rb.pop(), Some(57 * i + j));
        }
    }
    dbg!(&rb);
}
