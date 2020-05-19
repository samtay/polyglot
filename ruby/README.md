# ruby

This directory contains a bare ruby project, whose only dependencies are used
for executing doctests. To solve more exercises, just add a
`chapter_n/exercise_m.rb` file with a solution function and associated
doctest, e.g.
```ruby
# @example
#   solve_the_problem(testInput) #=> correctOutput
def solve_the_problem(input)
  return solution
end
```
Then add the exercise module to the `doctest_helper.rb`:
```
require 'chapter_n/exercise_m'
```
Then run `bundle exec rake yard:doctest` to validate.
