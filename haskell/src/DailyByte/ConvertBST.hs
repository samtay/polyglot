{-# LANGUAGE RecordWildCards #-}
module DailyByte.ConvertBST
  ( convert
  ) where

import Data.Maybe (fromMaybe)

-- | Given a binary search tree, rearrange the tree such that it forms a linked list where all its
-- values are in ascending order.
--
-- Note: all values in the binary search tree will be unique.
--
--  >>> leaf a = Just $ BSTree Nothing a Nothing
--  >>> convert $ BSTree (leaf 1) 5 (leaf 6)
--  [1,5,6]
--  >>> convert $ BSTree (Just $ BSTree (leaf 1) 2 (leaf 3)) 5 (leaf 9)
--  [1,2,3,5,9]
--  >>> convert $ BSTree Nothing 5 (leaf 6)
--  [5,6]

data BSTree a = BSTree
  { left :: Maybe (BSTree a)
  , value :: a
  , right :: Maybe (BSTree a)
  }

-- Note: for simplicity just using the default cons list. Since this requires a lot of appending,
-- we could improve this drastically by using a DList.
convert :: BSTree a -> [a]
convert BSTree{..} =
     fromMaybe [] (convert <$> left)
  ++ [value]
  ++ fromMaybe [] (convert <$> right)
