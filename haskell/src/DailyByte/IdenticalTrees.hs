{-# LANGUAGE RecordWildCards #-}
module DailyByte.IdenticalTrees
  ( BSTree(..)
  ) where

-- | Given two binary trees, return whether or not the two trees are identical.
-- Note: identical meaning they exhibit the same structure and the same values at each node.
--
--  >>> leaf a = Just $ BSTree Nothing a Nothing
--  >>> BSTree (leaf 1) 2 (leaf 3) == BSTree (leaf 1) 2 (leaf 3)
--  True
--  >>> BSTree Nothing 1 (Just $ BSTree Nothing 9 (leaf 18)) == BSTree (Just $ BSTree Nothing 9 (leaf 18)) 1 Nothing
--  False
data BSTree a = BSTree
  { left :: Maybe (BSTree a)
  , value :: a
  , right :: Maybe (BSTree a)
  } deriving (Show)

instance Eq a => Eq (BSTree a) where
  (BSTree l1 v1 r1) == (BSTree l2 v2 r2) =
    v1 == v2 && l1 == l2 && r1 == r2
