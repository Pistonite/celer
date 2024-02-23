# pure/result

TypeScript based result return type, inspired by Rust, but not completely the same.

This project is used internally in my own projects. If you want to depend on it,
simply copy the files over to your project.

## Function that can fail
Instead of having functions throw, make it return instead.
```typescript
// Instead of 
function doSomethingCanFail() {
    if (Math.random() < 0.5) {
        return;
    }
    throw "oops";
}
// Do this (what ResultHandle and Result are will be explained below)
import type { ResultHandle, Result } from "pure/result";

function doSomethingCanFail(r: ResultHandle): Result<void, string> {
    if (Math.random() < 0.5) {
        return r.voidOk();
    }
    return r.putErr("oops");
}
```
This is similar to Rust:
```rust
fn do_something_can_fail() -> Result<(), String> {
    if ... {
        return Ok(());
    }

    Err("oops".to_string())
}
```

## Calling function that can fail
A function that returns `Result` should take in `ResultHandle` as one of the parameters,
and use it to interact with the result system.

The `ResultHandle` is actually the same object as the `Result`. The functions you call
are only for TypeScript magic. You can think of `ResultHandle` as uninitialized `Result`.

This example below shows this interaction:
```typescript
function getParam(r: ResultHandle, name: string): Result<number, Error> {
    if (name === "a") {
        // `putOk` mutates r to contain an Ok value
        return r.putOk(13); // the return expression has type Result<number, Error>
    }
    if (name === "b") {
        return r.putOk(42);
    }
    // `putErr` mutates r to contain an Err value
    return r.putErr(new Error("bad name"));
}

function multiplyFormat(
    r: ResultHandle, 
    name1: string, 
    name2: string, 
    prefix: string
): Result<string, Error> {
    // breaking this down to individual steps so I can explain the TypeScript magic 
    r.put(
        // when calling this, r has type ResultHandle
        getParam(r, name1) // The return type is Result<number, Error>
    ); // calling `put` will do nothing, but use TypeScript magic to make the type of r Result<number,Error> now
    // (if you call `put` with a parameter that is not `this`, it will give a warning and try to copy the result over)

    // now that r is `Result<number, Error>`, you can use isOk and isErr to check the result
    if (r.isErr()) {
        // here the type of r is Err<T>
        console.error(r.error); // `error` property is only accessible after the check

        // ret() gives r back, but casted to the right type (can be casted to any Ok type)
        // without ret(), r is Result<number, Error>, which is not assignable to Result<string, Error>
        return r.ret(); 
    }

    // here, r is not Err<T>
    // so r is ResultHandle & UncheckedResult<T, E> & Ok<T>
    // which means we can get the value
    const v1 = r.value;

    // now we want to reuse r to handle the next call
    // by calling r.erase(), it returns r back with ResultHandle type
    // since that's is compatible with the function parameter,
    // we can assign it and erase the previous handled information TypeScript knows
    r.put(getParam(r = r.erase(), name2));
    if (r.isErr()) {
        return r.ret();
    }
    const v2 = r.value;

    const formatted = `${prefix}${v1 * v2}`;
    return r.putOk(formatted);
}
```

You might be thinking, why all the TypeScript magic?? Why not just do this:
```typescript
type Result<T, E> = { ok: true, value: T } | { ok: false, error: E };
```
I have 2 reasons:
1. Unlike Rust, you cannot redeclare a variable to shadow the previous declaration. With a naive implementation, you end up with:
    ```typescript
    function foo() {
        const result1 = doSomething();
        if (!result1.ok) {
            return result1;
        }
        const v1 = result1.value;
        const result2 = doSomethingElse();
        if (!result2.ok) {
            return result2;
        }
        const v2 = result2.value;
    }
    ```
    Note the temporary `result1` and `result2`, which doesn't look pretty.

2. You need to constantly create and destructure objects. This could be a performance issue, but I never
   benchmarked anything, so it could just be my imagination. (In fact, my approach could perform worse)

## Holding on to result
One issue left is that since we are using the same `r` handle, we could run into concurrency issues.
Say the example above becomes async:
```typescript
async function getParam(r: ResultHandle, name: string): Promise<Result<number, Error>> {
    ...
}

async function multiplyFormat(
    r: ResultHandle,
    name1: string,
    name2: string,
    prefix: string
): Promise<Result<string, Error>> {
    r.put(await getParam(r, name1));
    if (r.isErr()) {
        return r.ret();
    }
    const v1 = r.value;

    r.put(await getParam(r = r.erase(), name2));
    if (r.isErr()) {
        return r.ret();
    }
    const v2 = r.value;

    const formatted = `${prefix}${v1 * v2}`;
    return r.putOk(formatted);
}
```
The problem comes if we want to call both `getParam` first, then await together:
```typescript
async function multiplyFormatAsync(
    r: ResultHandle,
    name1: string,
    name2: string,
    prefix: string
): Promise<Result<string, Error>> {
    await Promise.all([getParam(r, name1), getParam(r = r.erase(), name2)]);
    // since getParam will store the result in r directly, we lost one value
}
```
To overcome this, `fork()` is provided. It creates an empty ResultHandle.
Despite the name, it will not contain any value from the original handle.
```typescript
async function multiplyFormatAsync(
    r1: ResultHandle,
    name1: string,
    name2: string,
    prefix: string
): Promise<Result<string, Error>> {
    const r2 = r1.fork();
    await Promise.all([
        r1.put(getParam(r1, name1)),
        r2.put(getParam(r2, name2))
    ]);
    if (r1.isErr()) {
        return r1.ret();
    }
    if (r2.isErr()) {
        return r2.ret();
    }
    const formatted = `${prefix}${r1.value * v2.value}`;
    // you must use r1, not r2 here, since that's the parameter passed in
    return r1.putOk(formatted);
}
```

## Outermost call site
The last question remains: how to get `ResultHandle` in the first place to pass
to a function? This library provides 4 utility functions to initiate the call.
```typescript
import { tryCatch, tryCatchAsync, tryInvoke, tryInvokeAsync } from "pure/result";

// 1. Use tryInvoke to get a handle for invoking functions that return result
const result = tryInvoke(r => multiplyFormat(r, "a", "b", "answer: "));
// the type of result is StableResult<T, E>
// you can call isOk and isErr on it, 
// but cannot call putOk or putErr like you would with a ResultHandle
if (result.isOk()) {
    console.log(result.value) // 42 * 13 = 546
}

// 2. Use tryInvoke to do the same, but async
// tryInvokeAsync takes in a (r: ResultHandle) => Promise<Result<T, E>>
const result = await tryInvokeAsync((r) => multiplyFormatAsync(r, "a", "b", "answer: "));

// 3. Use tryCatch to wrap a function that throws with try-catch
const result = tryCatch(() => JSON.parse<MyData>(...));
// result has type StableResult<MyData, unknown>

// 4. Use tryCatchAsync to wrap an async function that can throw
async function doStuff() {
    throw "oops";
}
const result = await tryCatchAsync(doStuff);
```

## Innermost call site
What if you need to call a throwing function inside a result-handling function?
Use `r.tryCatch` and `r.tryCatchAsync`
```typescript
import { ResultHandle, Result } from "pure/result";

function doSomethingThatCouldThrow(): FooType {
    ...
}

function foo(r: ResultHandle): Result<void, Error> {
    // r.erase() is only needed if there are previous usage of r
    // in this function
    r.put(r.tryCatch(r = r.erase(), doSomethingThatCouldThrow));
    // type of r is Result<FooType, unknown>
}

async function doSomethingThatCouldThrowAsync(): Promise<FooType> {
    ...
}

async function foo(r: ResultHandle): Promise<Result<void, Error>> {
    // r.erase() is only needed if there are previous usage of r
    // in this function
    r.put(await r.tryCatchAsync(r = r.erase(), doSomethingThatCouldThrowAsync));
    // type of r is Result<FooType, unknown>
}
```

## Why is there no `match`/`map`/`mapErr`, etc?

If you are thinking this is a great idea:
```typescript
const result = tryInvoke(foo);
result.match(
    (okValue) => {
        // handle ok case
    },
    (errValue) => {
        // handle err case
    },
);
```
The vanilla `if` doesn't allocate the closures, and has less code, and you can
control the flow properly inside the blocks with `return`/`break`/`continue`
```typescript
const result = tryInvoke(foo);
if (result.isOk()) {
    // handle ok case
} else {
    // handle err case
}
```

As for the other utility functions from Rust's Result type, they really only benefit
because you can early return with `?` AND those abstractions are zero-cost in Rust.
Neither is true in JavaScript. Please just handle it in the most straightforward way.
