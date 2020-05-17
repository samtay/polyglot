def is_unique(s):
    """Return whether or not string contains all unique characters.

    >>> is_unique("uniqe ya")
    True
    >>> is_unique("unique ya")
    False
    """
    seen = {}
    for c in s:
        if c in seen: return False
        seen[c] = True
    return True


if __name__ == "__main__":
    import doctest
    doctest.testmod(verbose=True)
