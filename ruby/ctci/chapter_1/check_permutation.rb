# @example
#   check_permutation("Tom Marvolo Riddle", "Immortal Love Rodd") #=> true
#   check_permutation("Sam Chong Tay", "Mach Go Nasty") #=> true
#   check_permutation("Tommy", "Tammy") #=> false
def check_permutation(s1, s2)
  # Only keep the relevant chars
  s1 = s1.downcase.delete(' ')
  s2 = s2.downcase.delete(' ')

  # Build hashmap of s2 char counts
  char_count = Hash.new
  char_count.default = 0
  s2.each_char { |c| char_count[c] += 1 }

  # Decrement the hashmap with s1 char counts
  s1.each_char { |c| char_count[c] -= 1 }

  # Return whether or not hashmap is empty
  return char_count.all? { |_, ct| ct == 0 }
end
