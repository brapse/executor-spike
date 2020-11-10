Executor
========

As the world embraces async/await a increasing number of libraries
expose API that are inherently async. This comes with two principal
problems.

1. Async functions imply a runtime but in our software the runtime should be
   created explicitly and threaded as a dependency through components.
   Components should probably be tested with a different single threaded
   runtime than running in production with a multi threaded runtime. 

2. APIs which have async dependencies inherit the color of
   these functions. But the fact that a function does IO or has an
   otherwise "effect" should not bleed into the API of it's users. Our
   APIs should be strictly effect independent.

As an experiment, it would be interesting to see if we can construct a
single runtime with a clonable handle. The clonable handle could be
passed to other components and bbe a gateway for executing
Futures. This gateway would expose a synchronous interface which could
be completely mocked out during testing in order to isolate core logic
from effects.

## Comparables

[Libra PeerHandle](https://github.com/libra/libra/blob/master/network/src/peer/mod.rs)
Libra uses an old version of tokio (0.2.x vs latest 0.3.x) which provides since deprecated clonable Runtime::Handle.

[OpenEthereum Runtime](https://docs.rs/crate/parity-runtime/0.1.2/source/src/lib.rs)
Uses an even older version of tokio (v0.1.x vs latest 0.3.x) which uses
the depricated `TaskExecutor` which in a thin wrapper around a thread
[local global DefaultExecutor
struct](https://docs.rs/tokio-executor/0.1.8/src/tokio_executor/global.rs.html#67).

## Considerations
* I didn't want to use any 3rd party libraries here. I tried to make do
with the defaults.
* Obviously this is a trivial construction where the return type is fixed
at `u32`. It might be a bit more challenging to return different
structures.
* Even though internal, the use of `Pin<Box<dyn Future<Output=u32> +
'static + Send>` makes me personally nervous.

## References

* [Algebraic Effects for Functional Programming](https://www.microsoft.com/en-us/research/wp-content/uploads/2016/08/algeff-tr-2016-v2.pdf)
