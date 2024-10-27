"""
Main cli or app entry point
"""

import time
import psutil
from mylib.extract import extract
from mylib.transform import load
from mylib.query import queryCreate, queryRead, queryUpdate, queryDelete


def performance(func, *args):
    """
    Measures the performance of a function by tracking time and memory usage.
    """
    process = psutil.Process()
    start_memory = process.memory_info().rss / 1024  # Memory in KB
    start_time = time.time()

    # Execute the function
    result = func(*args)

    end_time = time.time()
    end_memory = process.memory_info().rss / 1024  # Memory in KB
    duration = end_time - start_time

    print(f"Function: {func.__name__}")
    print(f"Execution Time: {duration * 1000:.2f}ms")
    print(f"Memory Usage Before: {start_memory:.2f} KB")
    print(f"Memory Usage After: {end_memory:.2f} KB")
    print(f"Memory Consumed: {end_memory - start_memory:.2f} KB\n")

    return result, duration, start_memory, end_memory


def main_res():
    results = {}

    # Extract
    print("Performance of `extract` function:")
    results["extract_to"], _, _, _ = performance(extract)

    # Transform and Load
    print("Performance of `load` function:")
    results["transform_db"], _, _, _ = performance(load)

    # Query - Create
    print("Performance of `queryCreate` function:")
    results["create"], _, _, _ = performance(queryCreate)

    # Query - Read
    print("Performance of `queryRead` function:")
    results["read"], _, _, _ = performance(queryRead)

    # Query - Update
    print("Performance of `queryUpdate` function:")
    results["update"], _, _, _ = performance(queryUpdate)

    # Query - Delete
    print("Performance of `queryDelete` function:")
    results["delete"], _, _, _ = performance(queryDelete)

    return results


if __name__ == "__main__":
    main_res()
