# python

To add a solution, add a `chapter_n/exercise_m.py` file with doctests, e.g.

```python
def solve(input):
    """Solve stuff

    >>> solve(goodInput)
    goodOutput
    >>> solve(badInput)
    Traceback (most recent call last):
     ...
    NameError: name 'x' is not defined
    """
    solution = []
    return solution
```

and run `python -m doctest -v chapter_n/exercise_m.py` to test it (or glob
`**/*.py` to test them all).
