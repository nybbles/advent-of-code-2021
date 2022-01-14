use std::collections::VecDeque;

const WINDOW_SIZE: usize = 3;

type OptionalIteratorItem<'a, T> = Option<&'a T>;

type SlidingWindowItem<'a, T> = (
  OptionalIteratorItem<'a, T>,
  OptionalIteratorItem<'a, T>,
  OptionalIteratorItem<'a, T>,
);

type SlidingWindowBuffer<'a, T> = VecDeque<OptionalIteratorItem<'a, T>>;

enum SlidingWindowState {
  IterationNotStarted,
  Iterating,
  IterationDone,
}

struct SlidingWindow<'a, I: Iterator> {
  iter: I,
  buffer: SlidingWindowBuffer<'a, I::Item>,
  state: SlidingWindowState,
}

fn buffer_to_sliding_window_item<'a, 'b, I: Iterator>(
  buffer: &'a SlidingWindowBuffer<'b, I::Item>,
) -> SlidingWindowItem<'b, I::Item> {
  assert_eq!(buffer.len(), WINDOW_SIZE);
  (buffer[0], buffer[1], buffer[2])
}

impl<'a, I> Iterator for SlidingWindow<'a, I>
where
  I: Iterator,
{
  type Item = SlidingWindowItem<'a, I::Item>;

  fn next(&mut self) -> Option<SlidingWindowItem<'a, I::Item>> {
    match (self.iter.next(), &self.state) {
      (None, SlidingWindowState::IterationDone | SlidingWindowState::IterationNotStarted) => None,
      (None, SlidingWindowState::Iterating) => {
        self.buffer.pop_front();
        self.buffer.push_back(None);
        self.state = SlidingWindowState::IterationDone;
        Some(buffer_to_sliding_window_item::<I>(&mut self.buffer))
      }
      // TODO: Implement
      (Some(item), SlidingWindowState::IterationDone) => None,
      // TODO: Implement
      (Some(item), SlidingWindowState::IterationNotStarted) => None,
      // TODO: Implement
      (Some(item), SlidingWindowState::Iterating) => None,
    }
  }
}

impl<'a, I: Iterator> SlidingWindow<'a, I> {
  fn new(iter: I) -> SlidingWindow<'a, I> {
    let buffer = VecDeque::from([None, None, None]);
    SlidingWindow {
      iter: iter,
      buffer: buffer,
      state: SlidingWindowState::IterationNotStarted,
    }
  }
}

#[test]
fn test_sliding_window() {
  let input = vec![1u32, 2, 3];
  let result: Vec<SlidingWindowItem<u32>> = SlidingWindow::new(input.iter().map(|&x| x)).collect();
  let expected = vec![
    (None, Some(&1), Some(&2)),
    (Some(&1), Some(&2), Some(&3)),
    (Some(&2), Some(&3), None),
  ];

  assert_eq!(result, expected);
}
