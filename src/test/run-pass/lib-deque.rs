// -*- rust -*-

use std;
import std.deque;

fn test_simple() {
  let deque.t[int] d = deque.create[int]();
  check (d.size() == 0u);
  d.add_front(17);
  d.add_front(42);
  d.add_back(137);
  check (d.size() == 3u);
  d.add_back(137);
  check (d.size() == 4u);

  log d.peek_front();
  check (d.peek_front() == 42);

  log d.peek_back();
  check (d.peek_back() == 137);

  let int i = d.pop_front();
  log i;
  check (i == 42);

  i = d.pop_back();
  log i;
  check (i == 137);

  i = d.pop_back();
  log i;
  check (i == 137);

  i = d.pop_back();
  log i;
  check (i == 17);

  /* FIXME (issue #138):  Test d.get() once it no longer causes
   * segfault. */
}

fn test_boxes(@int a, @int b, @int c, @int d) {
  let deque.t[@int] deq = deque.create[@int]();
  check (deq.size() == 0u);
  deq.add_front(a);
  deq.add_front(b);
  deq.add_back(c);
  check (deq.size() == 3u);
  deq.add_back(d);
  check (deq.size() == 4u);

  check (deq.peek_front() == b);
  check (deq.peek_back() == d);

  check (deq.pop_front() == b);
  check (deq.pop_back() == d);
  check (deq.pop_back() == c);
  check (deq.pop_back() == a);

  /* FIXME (issue #138):  Test d.get() once it no longer causes
   * segfault. */
}

type eqfn[T] = fn(&T a, &T b) -> bool;

fn test_parameterized[T](eqfn[T] e, &T a, &T b, &T c, &T d) {
  let deque.t[T] deq = deque.create[T]();
  check (deq.size() == 0u);
  deq.add_front(a);
  deq.add_front(b);
  deq.add_back(c);
  check (deq.size() == 3u);
  deq.add_back(d);
  check (deq.size() == 4u);

  check (e(deq.peek_front(), b));
  check (e(deq.peek_back(), d));

  check (e(deq.pop_front(), b));
  check (e(deq.pop_back(), d));
  check (e(deq.pop_back(), c));
  check (e(deq.pop_back(), a));

  /* FIXME (issue #138):  Test d.get() once it no longer causes
   * segfault. */
}

type taggy = tag(one(int), two(int, int), three(int, int, int));

type taggypar[T] = tag(onepar(int),
                       twopar(int, int),
                       threepar(int, int, int));

type reccy = rec(int x, int y, taggy t);

fn main() {
  fn inteq(&int a, &int b) -> bool {
    ret a == b;
  }

  fn intboxeq(&@int a, &@int b) -> bool {
    ret a == b;
  }

  fn taggyeq(&taggy a, &taggy b) -> bool {
    alt (a) {
      case (one(a1)) {
        alt (b) {
          case (one(b1)) { ret a1 == b1; }
          case (_) { ret false; }
        }
      }
      case (two(a1, a2)) {
        alt (b) {
          case (two(b1, b2)) { ret (a1 == b1 && a2 == b2); }
          case (_) { ret false; }
        }
      }
      case (three(a1, a2, a3)) {
        alt (b) {
          case (three(b1, b2, b3)) { ret (a1 == b1 && a2 == b2 && a3 == b3); }
          case (_) { ret false; }
        }
      }
    }
  }

  fn taggypareq[T](&taggypar[T] a, &taggypar[T] b) -> bool {
    alt (a) {
      case (onepar[T](a1)) {
        alt (b) {
          case (onepar[T](b1)) { ret a1 == b1; }
          case (_) { ret false; }
        }
      }
      case (twopar[T](a1, a2)) {
        alt (b) {
          case (twopar[T](b1, b2)) { ret (a1 == b1 && a2 == b2); }
          case (_) { ret false; }
        }
      }
      case (threepar[T](a1, a2, a3)) {
        alt (b) {
          case (threepar[T](b1, b2, b3)) {
            ret (a1 == b1 && a2 == b2 && a3 == b3);
          }
          case (_) { ret false; }
        }
      }
    }
  }

  fn reccyeq(&reccy a, &reccy b) -> bool {
    ret (a.x == b.x && a.y == b.y && taggyeq(a.t, b.t));
  }

  log "test simple";
  test_simple();

  /*
   * FIXME: Causes "Invalid read of size 4" under valgrind.

  log "test boxes";
  test_boxes(@5, @72, @64, @175);

   */

  log "test parameterized: int";
  let eqfn[int] eq1 = bind inteq(_, _);
  test_parameterized[int](eq1, 5, 72, 64, 175);

  /*
   * FIXME: Appears to segfault after an upcall_grow_task

  log "test parameterized: @int";
  let eqfn[@int] eq2 = bind intboxeq(_, _);
  test_parameterized[@int](eq2, @5, @72, @64, @175);

   */

  log "test parameterized: taggy";
  let eqfn[taggy] eq3 = bind taggyeq(_, _);
  test_parameterized[taggy](eq3,
                            one(1), two(1, 2), three(1, 2, 3), two(17, 42));

  /*
   * FIXME: Segfault.

  log "test parameterized: taggypar[int]";
  let eqfn[taggypar[int]] eq4 = bind taggypareq[int](_, _);
  test_parameterized[taggypar[int]](eq4,
                                    onepar[int](1),
                                    twopar[int](1, 2),
                                    threepar[int](1, 2, 3),
                                    twopar[int](17, 42));

   */

  /*
   * FIXME: Segfault.

  log "test parameterized: reccy";
  let reccy reccy1 = rec(x=1, y=2, t=one(1));
  let reccy reccy2 = rec(x=345, y=2, t=two(1, 2));
  let reccy reccy3 = rec(x=1, y=777, t=three(1, 2, 3));
  let reccy reccy4 = rec(x=19, y=252, t=two(17, 42));
  let eqfn[reccy] eq5 = bind reccyeq(_, _);
  test_parameterized[reccy](eq5,
                            reccy1, reccy2, reccy3, reccy4);

   */

  log "done";
}