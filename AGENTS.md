Pick out one function from src/rust_bindings/ from any file within the directory that would be the simplest to implement.
Run ```bash ./sqlite3-traversal *fn-name*``` in pwd, this will emit a json, consisting of all helper functions, that are being called, as well as 
called recursively. You must register each function with sqlite3_symbols.txt, if a function has been registered - try to find it with 
 On simple functions you work in batches of 10, On complex you do not batch and emit one function at a time, running the test suite. 
