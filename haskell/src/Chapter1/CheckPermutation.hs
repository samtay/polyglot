module Chapter1.CheckPermutation
  (
    checkPermutation
  ) where

import qualified Data.Char as Char
import qualified Data.List as List

-- | Check if one string is a permutation of another
--
--  Sort then check equality:
--    O(max(n log n, m log m))
--
--  >>> checkPermutation "Tom Marvolo Riddle" "Immortal Love Rodd"
--  True
--  >>> checkPermutation "Tom Marvolo Riddle" "Marmot Dildo Lover"
--  True
--  >>> checkPermutation "Tom Marvolo Riddle" "Mild Doormat Lover"
--  True
--  >>> checkPermutation "Sam Chong Tay" "Mach Go Nasty"
--  True
--  >>> checkPermutation "Tommy" "Tammy"
--  False
--
checkPermutation :: String -> String -> Bool
checkPermutation s1 s2 =
  let f = List.sort . fmap Char.toLower . filter (not . Char.isSpace)
  in f s1 == f s2
