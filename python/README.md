# python

For now the tooling is quite bare in this directory. To add a solution, simply
add a `chapter_n/exercise_m.py` file that will run doctests when executed, e.g.

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

if __name__ == "__main__":
    import doctest
    doctest.testmod(verbose=True)
```

and run `./test` to check it.
