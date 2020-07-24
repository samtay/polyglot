module DailyByte.MergeLists
  ( merge
  ) where

-- | This question is asked by Apple. Given two sorted linked lists, merge them together in
--   ascending order and return a reference to the merged list.
--
--  >>> merge [1, 2, 3] [4, 5, 6]
--  [1,2,3,4,5,6]
--  >>> merge [1, 3, 5] [2, 4, 6]
--  [1,2,3,4,5,6]
--  >>> merge [4, 4, 7] [1, 5, 6]
--  [1,4,4,5,6,7]
merge :: Ord a => [a] -> [a] -> [a]
merge [] [] = []
merge [] ys@(_:_) = ys
merge xs@(_:_) [] = xs
merge (x:xs) (y:ys) =
  if x < y
  then x : merge xs (y:ys)
  else y : merge (x:xs) ys
