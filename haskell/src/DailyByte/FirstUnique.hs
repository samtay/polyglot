module DailyByte.FirstUnique
  ( firstUnique
  ) where

import Data.Maybe (fromMaybe, listToMaybe)
import qualified Data.Map as Map

-- | This question is asked by Microsoft. Given a string, return the index of its first unique
--   character. If a unique character does not exist, return -1.
--
--  >>> firstUnique "abcabd"
--  2
--  >>> firstUnique "thedailybyte"
--  1
--  >>> firstUnique "developer"
--  0
--  >>> firstUnique "no non unique iqe"
--  -1
firstUnique :: String -> Int
firstUnique str = fromMaybe (-1) $ do
  let chars = foldr (Map.alter $ Just . maybe (1::Int) (+1)) Map.empty str
  fmap fst
    . listToMaybe
    . filter ( (== Just 1) . (`Map.lookup` chars) . snd )
    $ (zip [0..] str)
