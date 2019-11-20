Issues of Async WebSocket implementations
=========================================

This repo features 4 echo websocket server and 3 client implementations.

This was not designed to be a benchmark, but it can serve this purpose; just
keep in mind that there was no such requirement.

Actix WebSocket implementation is based on actix-web-actors, while async-std
and tokio variants are based on mostly the same tungstenite implementation:

* [tokio-tungstenite](https://github.com/snapview/tokio-tungstenite/pull/68)
  with patches for
  [tokio master](https://github.com/frol/tokio-tungstenite/tree/tokio2-master)
  and [pinned tokio 0.2-alpha.6](https://github.com/frol/tokio-tungstenite/tree/tokio2-alpha)
* [async-tungstenite](https://github.com/sdroege/async-tungstenite), which is
  [ported](https://github.com/snapview/tokio-tungstenite/pull/68#issuecomment-552088308)
  from tokio-tungstenite

| Client \ Server | actix               | async-std | tokio 0.2-alpha6  | tokio master |
| --------------- | ------------------- | --------- | ----------------- | ------------ |
| async-std       | 25s                 | 32s       | 65s               | 45s          |
| tokio 0.2alpha6 | 42s                 | 59s       | stuck             | 44s          |
| tokio master    | 18s <sup>(*1)</sup> | stuck     | stuck             | stuck        |

(*1) - The implementation can deadlock if you increase the number of iterations
as it sends messages faster than receives.

(stuck) - client stops sending or receiving messages

Side notes:

1. actix server implementation is the fastest.
2. async-std client implementation is stable and faster than tokio given the
   async-tungstenite shares the codebase from tokio-tungstenite.
