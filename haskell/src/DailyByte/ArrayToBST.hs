{-# LANGUAGE RecordWildCards #-}
module DailyByte.ArrayToBST
  ( toBST
  ) where

-- | Given an array of numbers sorted in ascending order, return a height balanced
-- binary search tree using every number from the array. Note: height balanced meaning
-- that the level of any node’s two subtrees should not differ by more than one.
--
--  >>> toBST [1,2,3]
--  Just (BSTree {left = Just (BSTree {left = Nothing, value = 1, right = Nothing}), value = 2, right = Just (BSTree {left = Nothing, value = 3, right = Nothing})})
--  >>> toBST [1,2,3,4,5,6]
--  Just (BSTree {left = Just (BSTree {left = Just (BSTree {left = Nothing, value = 1, right = Nothing}), value = 2, right = Just (BSTree {left = Nothing, value = 3, right = Nothing})}), value = 4, right = Just (BSTree {left = Just (BSTree {left = Nothing, value = 5, right = Nothing}), value = 6, right = Nothing})})
toBST :: [a] -> Maybe (BSTree a)
toBST []     = Nothing
toBST [a]    = leaf a
toBST [a, b] = Just $ BSTree (leaf a) b Nothing
toBST xs     =
  let mid = length xs `div` 2
      (l, (v:r)) = splitAt mid xs
  in Just $ BSTree
    { left  = toBST l
    , value = v
    , right = toBST r
    }

data BSTree a = BSTree
  { left :: Maybe (BSTree a)
  , value :: a
  , right :: Maybe (BSTree a)
  } deriving (Show)

leaf :: a -> Maybe (BSTree a)
leaf a = Just $ BSTree Nothing a Nothing
