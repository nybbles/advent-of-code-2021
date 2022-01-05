from utils import threadf


def test_threadf():
  inc = lambda x: x + 1
  mul = lambda x: x * 2
  assert (threadf(10, [inc, mul]) == (10 + 1) * 2)
  assert (threadf(10, [mul, inc]) == (10 * 2) + 1)
