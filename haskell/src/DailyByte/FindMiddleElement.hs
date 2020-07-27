module DailyByte.FindMiddleElement
  ( middle
  ) where

-- | This question is asked by Amazon. Given a non-empty linked list,
-- return the middle node of the list. If the linked list contains an
-- even number of elements, return the node closer to the end.
--
-- Pretending length and !! don't exist
--
--  >>> middle [1, 2, 3]
--  Just 2
--  >>> middle [1, 2, 3, 4]
--  Just 3
--  >>> middle [1]
--  Just 1
--  >>> middle []
--  Nothing
middle :: [a] -> Maybe a
middle xs = move (len `div` 2) xs
  where
    len           = foldr (\_ i -> i + 1) (0 :: Int) xs
    move _ []     = Nothing
    move 0 (x:_)  = Just x
    move i (_:ys) = move (i-1) ys
