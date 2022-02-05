Individual test case defines:
- starting scenario
- input streams for both players

Inputstream: player input is given as a frame perfect input stream consisting of holds and inputs. Implicitly hold forever as last instruction.
- idle(seconds) - do nothing for a while
- input(inputstr) - one by one inputs the given requirements and waits a tick in between each one. (Same parsing as motion_input)
- `wait<T>(predicate: impl Fn(Query<T>)`, timeout: seconds), timeout: seconds) - wait for the predicate to return true or ubtil timeout.
- `assertion<T>(impl Fn(Query<T>))` - run an assertion function on the world

Scenario: a situation that is used as a basis for other tests. Created before running test cases with input streams.

- [x] Separate test reader for parser tests (one that takes inputs one change at a time)
- [ ] Separate test reader for integration tests (one that is preloaded)