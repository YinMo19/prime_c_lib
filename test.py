import sympy as sp

with open("primes.txt", "r") as f:
    primes = list(map(int, f.read().splitlines()))
    for num in primes:
        if not sp.isprime(num):
            print(num)
