{-# LANGUAGE RecordWildCards #-}
module DailyByte.LowestCommonAncestor
  ( ancestor
  ) where

-- | Given a binary search tree that contains unique values and two nodes within
-- the tree, a, and b, return their lowest common ancestor. Note: the lowest common
-- ancestor of two nodes is the deepest node within the tree such that both nodes
-- are descendants of it.
--
-- I'm going to do this by value instead of by reference. Also, based on wording of
-- the question, I'm going to assume the two values do exist within the tree.
--
--  >>> leaf a = Just $ BSTree Nothing a Nothing
--  >>> ancestor 1 9 $ BSTree (Just $ BSTree (leaf 1) 2 (leaf 5)) 7 (leaf 9)
--  Just 7
--  >>> ancestor 2 6 $ BSTree (Just $ BSTree (leaf 2) 3 (leaf 6)) 8 (leaf 9)
--  Just 3
--  >>> ancestor 6 8 $ BSTree (leaf 6) 8 (leaf 9)
--  Just 8
ancestor :: Ord a => a -> a -> BSTree a -> Maybe a
ancestor a b BSTree{..}
  | a < value && b < value = ancestor a b =<< left
  | a > value && b > value = ancestor a b =<< right
  | a <= value && value <= b = Just value
  | b <= value && value <= a = Just value
  | otherwise = Nothing

data BSTree a = BSTree
  { left :: Maybe (BSTree a)
  , value :: a
  , right :: Maybe (BSTree a)
  }

