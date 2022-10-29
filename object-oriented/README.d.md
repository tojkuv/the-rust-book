# 

### Implementing Object-Oriented Design Pattern

we call the take method to take the Some value out of the state field and leave a _None in its place. we call the take method to take the Some value out of the state field and leave a _None in its place

we’re going to have the Post delegate to a content method defined on its state

If we didn’_t call as_ref, we would get an error because we can’_t move state out of the borrowed &self of the function parameter.

We then call the unwrap method, which we know will never panic, because we know the methods on Post ensure that state will always contain a Some value when those methods are done. This is one of the cases we talked about in the “Cases In Which You Have More Information Than the Compiler” section of Chapter 9 when we know that a _None value is never possible, even though the compiler isn’_t able to understand that.
