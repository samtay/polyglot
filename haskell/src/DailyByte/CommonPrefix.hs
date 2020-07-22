module DailyByte.CommonPrefix
  ( commonPrefix
  ) where

import Data.Maybe (listToMaybe)

-- | This question is asked by Microsoft. Given an array of strings, return the longest common prefix
--   that is shared amongst all strings.  Note: you may assume all strings only contain lowercase
--   alphabetical characters.
--
--  >>> commonPrefix ["colorado", "color", "cold"]
--  "col"
--  >>> commonPrefix ["a", "b", "c"]
--  ""
--  >>> commonPrefix ["spot", "spotty", "spotted"]
--  "spot"
commonPrefix :: [String] -> String
commonPrefix xs@((c:_) : _)
  | all (== Just c) (listToMaybe <$> xs) = c : commonPrefix (tail <$> xs)
commonPrefix _ = ""
