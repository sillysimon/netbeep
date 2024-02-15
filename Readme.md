# Netbeep
It's `beep` over the Network

**WARNING** This repo is currently under construction
## Purpose
This programm listens on a TCP-Sockets and beeps your PC-Speaker accordingly.

## ToDo
I am currently stuck on the TCP Protocol. To work with TCP Packets, I need to do the following:

1. Understand the exact layout of the TCP header: from here: [https://www.ietf.org/rfc/rfc9293.html#name-header-format](https://www.ietf.org/rfc/rfc9293.html#name-header-format)
2. somehow extract the header size
3. read the payload

here are some links in no particular order:
https://doc.rust-lang.org/std/primitive.array.html
https://doc.rust-lang.org/std/primitive.usize.html
https://doc.rust-lang.org/std/vec/struct.Vec.html
https://users.rust-lang.org/t/reading-length-payload-from-a-tcpstream/51211
https://doc.rust-lang.org/std/net/struct.TcpStream.html#impl-Read-for-%26TcpStream
https://en.wikipedia.org/wiki/Transmission_Control_Protocol
https://datatracker.ietf.org/doc/html/draft-mcquistin-augmented-ascii-diagrams-10
