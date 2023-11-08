import time
import argparse


def factorial(n):
    return 1 if n == 0 else n * factorial(n - 1)


def benchmark(n):
    start_time = time.time()
    result = factorial(n)
    end_time = time.time()
    elapsed_time = end_time - start_time
    return result, elapsed_time


if __name__ == "__main__":
    parser = argparse.ArgumentParser()
    parser.add_argument("n", help="the number to compute the factorial of", type=int)
    args = parser.parse_args()
    result, elapsed_time = benchmark(args.n)
    print("Python factorial:", result)
    print("Time taken: {} seconds".format(elapsed_time))
    with open("../Resources/Summary.md", "a") as f:
        f.write("Python factorial: {}\n".format(result))
        f.write("Python Time taken: {} seconds\n".format(elapsed_time))
