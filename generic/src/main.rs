use buffer::Buffer;

mod buffer;
fn main() {
    let buffer = Buffer::new(vec![1, 2, 3, 4, 5]);
    if let Some(sum) = buffer.sum() {
        assert_eq!(sum, 15);
    }
    let buffer = Buffer::new(vec![0.33, 0.66, 0.11, 0.22]);
    if let Some(sum) = buffer.sum() {
        assert_eq!(sum, 1.32);
    }
    let buffer = Buffer::new(vec![1, -2, -3, -4, 5]);
    if let Some(sum) = buffer.sum() {
        assert_eq!(sum, -3);
    }
}
