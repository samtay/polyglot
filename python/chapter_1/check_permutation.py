def check_permutation(s1, s2):
    """Return whether or not s1 is a permutation of s2.

    Ignore spaces for fun testing.

    Uses hash table of character counts:
        - O(max(n,m)) amortized
        - O(nm) worst case

    >>> check_permutation("Tom Marvolo Riddle", "Immortal Love Rodd")
    True
    >>> check_permutation("Tom Marvolo Riddle", "Marmot Dildo Lover")
    True
    >>> check_permutation("Tom Marvolo Riddle", "Mild Doormat Lover")
    True
    >>> check_permutation("Sam Chong Tay", "Mach Go Nasty")
    True
    >>> check_permutation("Tommy", "Tammy")
    False
    """
    # not required by problem, so don't count in complexity analysis
    s1 = ''.join(s1.lower().split())
    s2 = ''.join(s2.lower().split())

    # O(m)
    # create a hash table with counts
    chars = {}
    for c in s2: chars[c] = 1 + chars.get(c, 0)

    # O(n)  amortized
    # O(nm) worst
    # subtract 1 for each c in s1
    for c in s1:
        ct = chars.get(c, 0)
        if ct == 0: return False
        chars[c] = ct - 1

    # O(m)
    return all(ct == 0 for ct in chars.values())

def check_permutation2(s1, s2):
    """More time, less space

    Sorts arrays and then compares them:
        - O(max(n log n, m log m))

    >>> check_permutation("Sam Chong Tay", "Mach Go Nasty")
    True
    >>> check_permutation("Tommy", "Tammy")
    False
    """
    # not required by problem, so don't count in complexity analysis
    s1 = ''.join(s1.lower().split())
    s2 = ''.join(s2.lower().split())

    # check lengths equal first
    if len(s1) != len(s2): return False

    # O (n log n)
    s1 = sorted(s1)
    # O (m log m)
    s2 = sorted(s2)

    # O(max(n, m))
    return s1 == s2
