use std::collections::VecDeque;

const WINDOW_SIZE: usize = 3;

type OptionalIteratorItem<I: Iterator> = Option<I::Item>;
type SlidingWindowItem<'a, I: Iterator> = (
  &'a OptionalIteratorItem<I>,
  &'a OptionalIteratorItem<I>,
  &'a OptionalIteratorItem<I>,
);

type SlidingWindowBuffer<I: Iterator> = VecDeque<OptionalIteratorItem<I>>;

enum SlidingWindowState {
  BufferToBeFilled,
  BufferFilled,
  BufferEmptied,
}

struct SlidingWindow<I: Iterator> {
  iter: I,
  buffer: SlidingWindowBuffer<I>,
  state: SlidingWindowState,
}

fn buffer_to_sliding_window_item<I: Iterator>(
  buffer: SlidingWindowBuffer<I>,
) -> SlidingWindowItem<I> {
  assert_eq!(buffer.len(), WINDOW_SIZE);
  (&buffer[0], &buffer[1], &buffer[2])
}

impl<I> Iterator for SlidingWindow<I>
where
  I: Iterator,
{
  type Item = SlidingWindowItem<I>;

  fn next(&mut self) -> Option<SlidingWindowItem<I>> {
    match (self.iter.next(), self.state) {
      (None, SlidingWindowState::BufferEmptied) => None,
      (None, SlidingWindowState::BufferFilled) => {
        self.buffer.pop_front();
        self.buffer.push_back(None);
        let item: SlidingWindowItem<I> = buffer_to_sliding_window_item::<I>(self.buffer);
        Some(item)
      }
    }
  }
}

impl<I: Iterator> SlidingWindow<I> {
  fn new(iter: I) -> SlidingWindow<I> {
    let buffer = VecDeque::from([None, None, None]);
    SlidingWindow {
      iter: iter,
      buffer: buffer,
      state: SlidingWindowState::BufferToBeFilled,
    }
  }
}

#[test]
fn test_sliding_window() {
  let input = vec![1u32, 2, 3];
  let result: Vec<Vec<Option<u32>>> = SlidingWindow::new(input.iter().map(|&x| x)).collect();
  let expected = vec![
    [None, Some(1), Some(2)],
    [Some(1), Some(2), Some(3)],
    [Some(2), Some(3), None],
  ];

  assert_eq!(result, expected);
}
