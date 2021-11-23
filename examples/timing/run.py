import timing_example
from timeit import default_timer as timer


def multiply(a: int, b: int) -> int:
    """
    This function is the same as timing_example.multiply, which is written as follows:

    fn multiply(a: isize, b: isize) -> PyResult<isize> {
        Ok(a * b)
    }

    """
    return a * b


function_calls = 10000000
start = timer()
for _ in range(function_calls):
    x = timing_example.multiply(2, 4)
calling_rust_function = round(timer() - start, 5)

start = timer()
for _ in range(function_calls):
    x = multiply(2, 4)
calling_python_function = round(timer() - start, 5)


print(
    f"""
{function_calls} to the Rust 'timing_example.multiply' function:\t{calling_rust_function} 
{function_calls} to the Python 'multiply' function:\t\t\t{calling_python_function}
"""
)


big_list = [x for x in range(100000000)]

start = timer()
timing_example.list_sum(big_list)
calling_rust_list_sum = round(timer() - start, 5)

print(
    f"""
{function_calls} to the Rust 'timing_example.list_sum' function:\t{calling_rust_list_sum} 

"""
)