module DailyByte.RemoveValue
  ( remove
  ) where

-- | This question is asked by Google. Given a linked list and a value,
-- remove all nodes containing the provided value, and return the resulting list.
--
--  >>> remove 3 [1, 2, 3]
--  [1,2]
--  >>> remove 1 [8, 1, 1, 4, 12]
--  [8,4,12]
--  >>> remove 7 [7, 12, 2, 9]
--  [12,2,9]
remove :: Eq a => a -> [a] -> [a]
remove v (x:xs)
  | x == v      = remove v xs
  | otherwise   = x : remove v xs
remove _ []     = []
