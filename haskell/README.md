# haskell

To add more solutions, just add a `src/ChapterN/ExcerciseM.hs` module with
doctests:
```haskell
module ChapterN.ExerciseM
  ( solution
  ) where

-- | Check problem solved
--
--  >>> solution goodInput
--  True
--  >>> solution badInput
--  False
--
--  Properties: solution is idempotent
--
--  prop> solution xs == solution (solution xs)
--
solution :: t a -> String
solution = const $ fix error
```
Run `stack test` to test all doctests, or `ghcid --test=':!stack test'` to watch them.
