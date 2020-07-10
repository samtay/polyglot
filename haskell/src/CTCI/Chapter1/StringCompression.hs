module CTCI.Chapter1.StringCompression
  (
    compress
  ) where

import           Control.Monad (guard)
import           Data.DList (DList (..))
import qualified Data.DList as DL

-- | Take a string and, if it reduces length, return a
--   run-length encoding of it, otherwise return the original string
--
--  O(n) Iterate over list of length n:
--    O(1) Accumulate counts of grouped characters via pattern matching
--    O(log c) Concat difference lists of character & counts [*]
--    O(1) If at any point we exceed `n`, return the original string [**]
--  O(n) convert dlist back into string
--
--  O(n log c) total
--
-- [*] Technically speaking, because we do a (DL.fromList . show) on the integer
-- counts, the inner operation is O(log c) where c is the maximum length of
-- a character streak (the string representation of integer c is log_{10}(c)
-- characters long, and it takes O(log_{10} c) to convert this representation
-- into a DList). Unless there are character streaks of length 10^N for large N,
-- we can probably consider this O(1) :)
--
-- [**] What's really nice about this is that due to laziness, we don't even
-- concat anything if the encoding turns out to be greater. Of course the benefit
-- is a little lost given that DLists have O(1) appends, but still pretty awesome.
--
-- >>> compress("aabcccccaaa")
-- "a2b1c5a3"
-- >>> compress("uniquegroups")
-- "uniquegroups"
-- >>> compress("aalmostuuniqueggroupss")
-- "aalmostuuniqueggroupss"
-- >>> compress("closssssse calllllllll")
-- "c1l1o1s6e1 1c1a1l9"
-- >>> compress("a")
-- "a"
--
-- prop> length (compress str) <= length str
--
compress :: String -> String
compress str =
  maybe str DL.toList $ go 0 0 str
  where
    n = length str

    go :: Int -> Int -> String -> Maybe (DL.DList Char)
    go _ _ []     = pure DL.empty
    go len ct [c] = snd <$> mkSubStr len ct c
    go len ct (c1 : c2 : cs)
      | c1 == c2  = go len (ct + 1) (c2 : cs)
      | otherwise = do
        (len', substr) <- mkSubStr len ct c1
        (DL.append $ substr) <$> go len' 0 (c2 : cs)

    mkSubStr :: Int -> Int -> Char -> Maybe (Int, DList Char)
    mkSubStr len ct c = do
      let substr = c : show (ct + 1)
          newLen = len + length substr
      guard $ newLen < n
      pure $ (newLen, DL.fromList substr)
