# Week 08 Mini Project: Comparing Performance of Rust vs Python [![Python CI](https://github.com/nogibjj/DukeIDS706_ds655_Week08/actions/workflows/main.yml/badge.svg)](https://github.com/nogibjj/DukeIDS706_ds655_Week08/actions/workflows/main.yml)

**Requirement** - Rewrite a Python script for data processing in Rust, highlighting the improvements in speed and resource usage.

This repository contains both `Python` and `Rust` Codes for the recursive [Factorial](https://www.mathsisfun.com/numbers/factorial.html) function. The time comparisons for both are logged in the `Summary.md` file. 


To run the Python code, the enter the following code in the terminal - 
`python main.py x` where x is the number to calculate the factorial of.

Python took 0.000012874603271484375 seconds to calculate the factorial of 30


To run the Rust code, the enter the following code in the terminal - 
`cargo run x` where x is the number to calculate the factorial of.

Rust took 0.000000431 seconds to calculate the factorial of 30.


Comparison of times to calculate factorial of 30 using Rust and Python:

![Comp](https://github.com/nogibjj/DukeIDS706_ds655_Week08/blob/64dbbcd002d704bc18dc3033e545746b572c0f43/Resources/Python%20Rust%20Comparison.png)


These timings can be found in the `Resources/Summary.md` file as well. 

In general, Rust is significantly faster than Python. Rust is a compiled language with a focus on zero-cost abstractions, efficient C bindings and fearless concurrency. Its performance is comparable to that of C and C++.

Python, on the other hand, is an interpreted language. While it has many features that make it easy to write clean, readable code, it is generally not as fast as compiled languages like Rust.

For computationally intensive tasks, Rust could be many times faster than Python. However, for IO-bound tasks or tasks involving a lot of text processing, the difference might be less noticeable.

Comparing the time taken using a recursive function to calculate the factorial of 30, we observe that Rust took `0.43` micro seconds, Python took `12.87` micro seconds. This suggests that Rust is `30x` faster when compared to Python.



Files in this repository include:


## 1. Readme
  The `README.md` file is a markdown file that contains basic information about the repository, what files it contains, and how to consume them


## 2. Requirements
  The `requirements.txt` file has a list of packages to be installed for any required project. Currently, my requirements file contains some basic python packages.


## 3. Codes
  This folder contains all the code files used in this repository - the files named "Test_" will be used for testing and the remaining will define certain functions
  * `main.py` contains the recursive factorial code in Python
  * `src/main.rs` contains the recursive factorial code in Rust


## 4. Resources
  -  This folder contains any other files relevant to this project. Currently, I have added -


## 5. CI/CD Automation Files


  ### 5(a). Makefile
  The `Makefile` contains instructions for installing packages (specified in `requirements.txt`), formatting the code (using black formatting), testing the code (running all the sample python code files starting with the term *'Check...'* ), and linting the code using pylint


  ### 5(b). Github Actions
  Github Actions uses the `main.yml` file to call the functions defined in the Makefile based on triggers such as push or pull. Currently, every time a change is pushed onto the repository, it runs the install packages, formatting the code, linting the code, and then testing the code functions


  ### 5(c). Devcontainer
  
  The `.devcontainer` folder mainly contains two files - 
  * `Dockerfile` defines the environment variables - essentially it ensures that all collaborators using the repository are working on the same environment to avoid conflicts and version mismatch issues
  * `devcontainer.json` is a json file that specifies the environment variables including the installed extensions in the virtual environment
