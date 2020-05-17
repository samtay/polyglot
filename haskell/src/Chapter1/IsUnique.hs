-- | Example of a library file. It is also used for testing the test suites.
module Chapter1.IsUnique
  (
    -- * Exported functions
    isUnique
  ) where

import qualified Data.HashMap.Strict as HM

-- | Check if String has all unique characters
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
isUnique = fst . foldr fn (True, HM.empty)
  where
    fn c (uniq, hm) =
      if uniq && not (HM.member c hm)
      then (True, HM.insert c () hm)
      else (False, HM.empty)
