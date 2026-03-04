Pick out one function from src/rust_bindings/ from any file within the directory that would be the simplest to implement.
Check sqlite3_symbols.txt whether this function is already implemented - if it, then, proceed with any other function.
Run ```bash ./sqlite3-traversal *fn-name*``` in pwd, this will emit a backbone_*fn_name*.json. NEVER ignore backbone_*fn_name*.json consisting of all helper functions, that are being called, as well as 
called recursively. You must register the file using rust_bindings/mod.rs for the respective file, if a function has been registered - ignore it and think of it as done.
On simple functions you work in batches of 10, on complex you do not batch and emit one function at a time, running the test.sh command which you also use to log
what hardships you may have stumbled upon and why it was more complex - if complex, than other tasks. 
It's forbidden to use C fallback, wrapping on simple functions. WRAPPING - IS NOT IMPLEMENTING. 
When there are no tests recorded by test.sh, you must run if $(pwd) === $HOME/Projects/new-wip-db else cd $HOME/Projects/new-wip-db then run
```bash
    ./sqlite3-src-3510200/rustfixture sqlite-src-3510200/test/extraquick.test
```
COMPLEXITIY METRIC: when sqlite3-traversal output is over 200 lines
For isolating faulty tests, you must edit sqlite3_symbols.txt and the remove #[unsafe(no_mangle)] from the function you are editing - instead of editing the entire function. Split the batch into two halves - run test.sh on BOTH half batches consecutively,  then, if the issue persists split the halves again and 
This test runs for about 7 minutes, it is only resorted for complex functions. Simple functions may omit it, if there is at least one relevant test being fould. When the test is finished and if it's finished with no mistakes - you may proceed onto the next. Elsewise, call python3 tesing/test_logger.py
and define the functions you were porting, status of the execution and why, if it does, fails.
Permutations.test - is not a test. This file returns a list of tests, so, it can neglected during testing.
If the reimplementation was a success (no test fails) - append the new functions to sqlite3_symbols.txt. Then, commit the change.