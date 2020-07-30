module DailyByte.ReverseList
  ( reverse
  ) where

import Prelude hiding (reverse)

-- | This question is asked by Facebook. Given a linked list,
-- containing unique values, reverse it, and return the result.
--
--  >>> reverse [1, 2, 3]
--  [3,2,1]
--  >>> reverse [7,15,9,2]
--  [2,9,15,7]
--  >>> reverse [1]
--  [1]
--  >>> reverse []
--  []
reverse :: [a] -> [a]
reverse = go []
  where
    go ys [] = ys
    go ys (x:xs) = go (x:ys) xs
