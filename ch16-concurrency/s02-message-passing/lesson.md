## 16.2 Transfer Data Between Threads with Message Passing

Channels let threads communicate by sending values to each other, rather than sharing memory. This sub-section sends one value, then several, then values from more than one
thread.

* [Sending and receiving one value](#goto/01-sending-one-value): `mpsc::channel`, `send`, and `recv`.
* [Receiving several values](#goto/02-sending-several-values): the receiver as an iterator, and the end of the channel.
* [Multiple producers](#goto/03-multiple-producers): cloning the transmitter, and sorting messages for fixed output.
* [Summary](#goto/04-summary): the main ideas, and the book's quizzes.

---

*This overview is adapted from the Brown University edition of The Rust Programming Language.*
