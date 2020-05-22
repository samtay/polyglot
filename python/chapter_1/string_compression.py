from itertools import groupby, chain

def compress(s):
    """Compress string with character counts, only if it results in a smaller
    string. Because there are convenient itertools for such compression (which
    claim memory efficiency), this version favors cleaner code rather than
    a custom algorithm: just do groupby compression, then compare string
    lengths at the end.

    >>> compress('aabcccccaaa')
    'a2b1c5a3'
    >>> compress('uniquegroups')
    'uniquegroups'
    >>> compress('almostuniquegroupss')
    'almostuniquegroupss'
    """
    compressed = compress_crude(s)
    return compressed if len(compressed) < len(s) else s

def compress_crude(s):
    """Compress string with character counts

    >>> compress_crude('aabcccccaaa')
    'a2b1c5a3'
    >>> compress_crude('uniquegroups')
    'u1n1i1q1u1e1g1r1o1u1p1s1'
    >>> compress_crude('almostuniquegroupss')
    'a1l1m1o1s1t1u1n1i1q1u1e1g1r1o1u1p1s2'
    """
    return ''.join(chain.from_iterable([
        [c, str(len(list(c_group)))] for (c, c_group) in groupby(s)
    ]))
