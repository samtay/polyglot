# @example
#   is_unique("uniqe ya") #=> true
#   is_unique("unique ya") #=> false
def is_unique(str)
  seen = Hash.new
  str.each_char { |c|
    if seen.member?(c)
      return false
    else
      seen[c] = true
    end
  }
  return true
end
