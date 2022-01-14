const WINDOW_SIZE: usize = 3;

struct SlidingWindow<I: Iterator> {
  iter: I,
  buffer: [Option<I::Item>; WINDOW_SIZE],
}

impl<I> Iterator for SlidingWindow<I>
where
  I: Iterator,
{
  type Item = [Option<I::Item>; WINDOW_SIZE];

  fn next(&mut self) -> Option<[Option<I::Item>; WINDOW_SIZE]> {
    None
  }
}

impl<I: Iterator> SlidingWindow<I> {
  fn new(iter: I) -> SlidingWindow<I> {
    let buffer: [Option<I::Item>; WINDOW_SIZE] = Default::default();
    SlidingWindow {
      iter: iter,
      buffer: buffer,
    }
  }
}

#[test]
fn test_sliding_window() {
  let input = vec![1u32, 2, 3];
  let result: Vec<[Option<u32>; WINDOW_SIZE]> =
    SlidingWindow::new(input.iter().map(|&x| x)).collect();
  let expected = vec![
    [None, Some(1), Some(2)],
    [Some(1), Some(2), Some(3)],
    [Some(2), Some(3), None],
  ];

  assert_eq!(result, expected);
}
