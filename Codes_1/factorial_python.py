import timeit


def factorial(n):
    return 1 if n == 0 else n * factorial(n - 1)


def benchmark():
    return timeit.timeit(lambda: factorial(10000000), number=1)


if __name__ == "__main__":
    print("Python factorial:", benchmark())
