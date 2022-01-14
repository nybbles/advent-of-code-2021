use std::collections::VecDeque;
use std::rc::Rc;

const WINDOW_SIZE: usize = 3;

type OptionalIteratorItem<T> = Option<T>;

type SlidingWindowItem<T> = (
  OptionalIteratorItem<T>,
  OptionalIteratorItem<T>,
  OptionalIteratorItem<T>,
);

type SlidingWindowBuffer<T> = VecDeque<OptionalIteratorItem<T>>;

enum SlidingWindowState {
  IterationNotStarted,
  Iterating,
  IterationDone,
}

struct SlidingWindow<I>
where
  I: Iterator,
  I::Item: Clone,
{
  iter: I,
  buffer: SlidingWindowBuffer<I::Item>,
  state: SlidingWindowState,
}

fn buffer_to_sliding_window_item<I>(
  buffer: &SlidingWindowBuffer<I::Item>,
) -> SlidingWindowItem<I::Item>
where
  I: Iterator,
  I::Item: Clone,
{
  assert_eq!(buffer.len(), WINDOW_SIZE);
  (buffer[0].clone(), buffer[1].clone(), buffer[2].clone())
}

impl<I> Iterator for SlidingWindow<I>
where
  I: Iterator,
  I::Item: Clone,
{
  type Item = SlidingWindowItem<I::Item>;

  fn next(&mut self) -> Option<SlidingWindowItem<I::Item>> {
    match (self.iter.next(), &self.state) {
      (None, SlidingWindowState::IterationDone | SlidingWindowState::IterationNotStarted) => None,
      (None, SlidingWindowState::Iterating) => {
        self.buffer.pop_front();
        self.buffer.push_back(None);
        self.state = SlidingWindowState::IterationDone;
        Some(buffer_to_sliding_window_item::<I>(&self.buffer))
      }
      // TODO: Implement
      (Some(item), SlidingWindowState::IterationDone) => None,
      // TODO: Implement
      (Some(item), SlidingWindowState::IterationNotStarted) => None,
      // TODO: Implement
      (Some(item), SlidingWindowState::Iterating) => {
        self.buffer.pop_front();
        self.buffer.push_back(Some(item));
        Some(buffer_to_sliding_window_item::<I>(&self.buffer))
      }
    }
  }
}

impl<I> SlidingWindow<I>
where
  I: Iterator,
  I::Item: Clone,
{
  fn new(iter: I) -> SlidingWindow<I> {
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
    (None, Some(1), Some(2)),
    (Some(1), Some(2), Some(3)),
    (Some(2), Some(3), None),
  ];

  assert_eq!(result, expected);
}
