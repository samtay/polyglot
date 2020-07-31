module DailyByte.CompareKeystrokes
  ( Keystroke(..)
  ) where

newtype Keystroke = Keystroke String

-- | This question is asked by Amazon. Given two strings s and t, which represents
-- a sequence of keystrokes, where # denotes a backspace, return whether or not
-- the sequences produce the same result.
--
--  >>> Keystroke "ABC#" == Keystroke "CD##AB"
--  True
--  >>> Keystroke "como#pur#ter" == Keystroke "computer"
--  True
--  >>> Keystroke "cof#dim#ng" == Keystroke "code"
--  False

instance Eq Keystroke where
  s == t = parse s == parse t

parse :: Keystroke -> String
parse (Keystroke s) = reverse $ go [] s
  where
    go cs []           = cs
    go (_:cs) ('#':ks) = go cs ks
    go cs (k:ks)       = go (k:cs) ks
