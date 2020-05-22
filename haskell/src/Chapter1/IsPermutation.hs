module Chapter1.IsPermutation
  (
    isPermutation
  ) where

import qualified Data.Char as Char
import qualified Data.List as List

-- | Check if one string is a permutation of another
--
--  Sort then check equality:
--    O(max(n log n, m log m))
--
--  >>> isPermutation "Tom Marvolo Riddle" "Immortal Love Rodd"
--  True
--  >>> isPermutation "Tom Marvolo Riddle" "Marmot Dildo Lover"
--  True
--  >>> isPermutation "Tom Marvolo Riddle" "Mild Doormat Lover"
--  True
--  >>> isPermutation "Sam Chong Tay" "Mach Go Nasty"
--  True
--  >>> isPermutation "Tommy" "Tammy"
--  False
--
isPermutation :: String -> String -> Bool
isPermutation s1 s2 =
  let f = List.sort . fmap Char.toLower . filter (not . Char.isSpace)
  in f s1 == f s2
