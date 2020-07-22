{-# LANGUAGE ViewPatterns #-}
module DailyByte.AddBinary
  ( add
  ) where

-- | This question is asked by Apple. Given two binary strings (strings containing only 1s and 0s)
--   return their sum (also as a binary string).  Note: neither binary string will contain leading 0s
--   unless the string itself is 0
--
--  >>> add "100" "1"
--  "101"
--  >>> add "11" "1"
--  "100"
--  >>> add "1" "0"
--  "1"
--  >>> add "111" "1111"
--  "10110"
add :: String -> String -> String
add (reverse -> a) (reverse -> b) = reverse $ go a b 0
  where
    go :: String -> String -> Int -> String
    -- At the end; don't put a leading zero
    go "" "" 0 = []
    -- At the end; handle the last carry
    go "" "" 1 = "1"
    -- We've finished one side
    go "" (x:xs) c = case sum [read [x], c] of
      0 -> '0' : go "" xs 0
      1 -> '1' : go "" xs 0
      2 -> '0' : go "" xs 1
      _ -> bomb
    -- Convenience; only handle the RHS
    go xs "" c = go "" xs c
    -- We're still iterating both sides
    go (x:xs) (y:ys) c = case sum [read [x], read[y], c] of
      0 -> '0' : go xs ys 0
      1 -> '1' : go xs ys 0
      2 -> '0' : go xs ys 1
      3 -> '1' : go xs ys 1
      _ -> bomb
    bomb = error "Invalid input"

