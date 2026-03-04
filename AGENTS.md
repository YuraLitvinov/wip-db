Pick out one function from src/rust_bindings/ from any file within the directory that would be the simplest to implement.
Run ```bash ./sqlite3-traversal *fn-name*``` in pwd, this will emit a backbone_*fn_name*.json. NEVER ignore backbone_*fn_name*.json consisting of all helper functions, that are being called, as well as 
called recursively. You must register each function with sqlite3_symbols.txt and register the file using rust_bindings/mod.rs for the respective file, if a function has been registered - ignore it and think of it as done.
On simple functions you work in batches of 10, on complex you do not batch and emit one function at a time, running the test.sh command which you also use to log
what hardships you may have stumbled upon and why it was more complex - if complex, than other tasks. 
It's forbidden to use C fallback on simple functions.
When there are no tests recorded by test.sh, you must run if $(pwd) === $HOME/Projects/new-wip-db else cd $HOME/Projects/new-wip-db then run
```bash
    ./sqlite3-src-3510200/rustfixture sqlite-src-3510200/test/extraquick.test
```
For isolating faulty tests, you must edit sqlite3_symbols.txt and the remove #[unsafe(no_mangle)] from the function you are editing - instead of editing the entire function. Split the batch into two halves - run test.sh on BOTH half batches consecutively,  then, if the issue persists split the halves again and 
This test runs for about 5 minutes, when it's finished and if it's finished with no mistakes - you may proceed onto the next. Elsewise, call python3 tesing/test_logger.py
and define the functions you were porting, status of the execution and why, if it does, fails.
Permutations.test - is not a test. This file returns a list of tests, so, it can neglected during testing.
If the reimplementation was a success (no test fails) - commit the change