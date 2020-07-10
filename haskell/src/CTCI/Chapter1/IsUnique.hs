module CTCI.Chapter1.IsUnique
  (
    isUnique
  , isUnique2
  ) where

import qualified Data.List as List
import qualified Data.HashMap.Strict as HM

-- | Check if String has all unique characters
--
--  This version just sorts first, then iterates through the list, hence
--  has complexity O(n log n + n) == O(n log n)
--
--  >>> isUnique "uniqe ya"
--  True
--  >>> isUnique "unique ya"
--  False
--
--  Properties:
--
--  Not sure why this complains: prop> isUnique xs ==> (nub xs == xs)
--
isUnique :: String -> Bool
isUnique =
  and
  . (\s -> zipWith (/=) (drop 1 s) s)
  . List.sort

-- | Hashmap version
--
--  However these persistent data structures don't have the performance of typical
--  mutable structures found in other languages. There are mutable versions that
--  operate in IO but this seems like overkill.
isUnique2 :: String -> Bool
isUnique2 = fst . foldr fn (True, HM.empty)
  where
    fn c (uniq, hm) =
      if uniq && not (HM.member c hm)
      then (True, HM.insert c () hm)
      else (False, HM.empty)
