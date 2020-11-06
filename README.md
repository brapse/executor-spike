Executor
========

What we want is a way to share access to single runtime through clonable
executor handles.

The interface of the executor handle should probably be:
- `spawn`
- `block_on`

// What's the difference between spawn and block_on
// Create a component
// Create a executor handle
// Run the component
// -> Running component creates a future which is executed on the runtime via the executor
// handle
