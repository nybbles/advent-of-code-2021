/*
use std::collections::VecDeque;
use std::rc::Rc;
use trees::{tr, Node, Tree};

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
    println!("{}", self.buffer.len());
    match (self.iter.next(), &self.state) {
      (None, SlidingWindowState::IterationDone | SlidingWindowState::IterationNotStarted) => None,
      (None, SlidingWindowState::Iterating) => {
        self.buffer.pop_front();
        self.buffer.push_back(None);
        self.state = SlidingWindowState::IterationDone;
        Some(buffer_to_sliding_window_item::<I>(&self.buffer))
      }
      (Some(_), SlidingWindowState::IterationDone) => {
        // There should be no way to get another item from iter while in the
        // iteration done state.
        panic!("Logic error");
      }
      (Some(item), SlidingWindowState::IterationNotStarted) => {
        self.buffer.push_back(None);
        self.buffer.push_back(Some(item));
        match self.iter.next() {
          // Only one item in iter, so return single window ((), item, ())
          None => {
            self.buffer.push_back(None);
            self.state = SlidingWindowState::IterationDone;
            Some(buffer_to_sliding_window_item::<I>(&self.buffer))
          }
          // More than two items, so return first window ((), item0, item1)
          Some(next_item) => {
            self.buffer.push_back(Some(next_item));
            self.state = SlidingWindowState::Iterating;
            Some(buffer_to_sliding_window_item::<I>(&self.buffer))
          }
        }
      }
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
    let buffer = VecDeque::from([]);
    SlidingWindow {
      iter: iter,
      buffer: buffer,
      state: SlidingWindowState::IterationNotStarted,
    }
  }
}
*/

/*
#[test]
fn test_sliding_window_u32() {
  let input = vec![1u32, 2, 3];
  let result: Vec<SlidingWindowItem<u32>> = SlidingWindow::new(input.iter().map(|&x| x)).collect();
  let expected = vec![
    (None, Some(1), Some(2)),
    (Some(1), Some(2), Some(3)),
    (Some(2), Some(3), None),
  ];

  assert_eq!(result, expected);
}

#[test]
fn test_sliding_window_tree() {
  type Foo = u32;
  let input: Tree<u32> = tr(0) / tr(1) / tr(2);
  let result: Vec<Rc<&Node<u32>>> = SlidingWindow::new(input.iter().map(|x| Rc::new(x))).collect();
}
*/
