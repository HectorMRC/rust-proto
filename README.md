# rust-proto
Rust server using protocol buffers.

## Motivation
This repo have been done in order to make easer to understand and solve the error mentioned on [stackoVerflow.com][1]

## Question

I'm having some issues understanding lifetimes in Rust. It may also be the way I implement my design.

```none
error[E0597]: `request` does not live long enough
  --> src/service/session/server.rs:25:23
   |
25 |         let msg_ref = request.get_ref();
   |                       ^^^^^^^ borrowed value does not live long enough
...
32 |         let body: Box<dyn Body> = Box::new(signup);
   |                                   ---------------- cast requires that `request` is borrowed for `'static`
...
44 |     }
   |     - `request` dropped here while still borrowed
```

and here the main source:

```rust
#[tonic::async_trait]
impl Endpoint for EndpointImplementation {
    async fn send( &self, request: Request<ProtoRequest>) -> Result<Response<ProtoResponse>, Status> {
        let msg_ref = request.get_ref();
        let signup = TxRequest::new(
            &msg_ref.msg,
        );
        
        let body: Box<dyn Body> = Box::new(signup);
        let tx = Transaction::new(body);
        let mut tx_signup: Box<dyn Tx> = Box::new(tx);
        tx_signup.execute();

        let response = ProtoResponse {
            msg: "".to_string(),
        };

        Ok(Response::new(response))
    }
}
```

## Background

The idea is to have a `Transaction`, that implements `Tx { execute(&self), result(&self) ... };`. This `Transaction` has a parameter `body` of the type `Box<dyn Box>`, being the trait `Body { /*some fn*/ }`. Having this, I'm pretending to implement some kind of hierarchy.

## The code above

From the header of send funtion I'm getting some requests of the type `ProtoRequest` (from proto file). This is the implementation of proto's server, using tonic and tokio.
After this, I have also an object `TxRequest` with some parameters of the type `&str` (set at the top of send body). `TxRequest` implements the `Body` trait so I'm able to turn it into a `Tx` trait, apparently. The `Transaction` object wraps a `Body` implementation. I call the `execute()` function from the given trait `Tx`. All that explained has been done just after signup's declaration.

## The problem

If I replace the `&str` type from `TxRequest` by type `String` it works. However, if I want them to be of the type `&str` there emerge a lot of "incongruences" with lifetimes. I want it as `&str` because any of these values will never change. I think is better to keep them in the stack instead of heap memory. Am I wrong?

If I want it as `&str` I'm coerced to define `TxRequest` with `<'a>`, and here is where I get lost. I get why a lifetime is required, but not why all these problems appears. As the one in the compiler output.
As far I do understand, all elements inside the function should have the same lifetime, being killed at the end of its block (just where send's body ends). I will never send them outside.

I have tried giving to `Body` trait also an `<'a>`, and even to `Tx` trait (meaning the `Trasanction` object must have one too to match the trait).

Is there any way to make it work? Am I misunderstanding the Trait use and how they work, or this patter design will never work?

**Github**
I have also reproduced this same error in the [rust-proto][2] repository from Github.com.

Running `cargo run` should warn up about it.

**Btw:** I come from Golang development and some c++, java and python. So I have a way to code that may not be the most appropriate one using Rust. That's what I want to solve.

Thanks you a lot.


  [1]: https://stackoverflow.com/questions/64341661/lifetime-error-using-traits-and-async-function-on-protobufers
  [2]: https://github.com/HectorMRC/rust-proto.git
