## Publish/Subscribe Monitor

### Compile and Run

```sh
$ make
$ ./pub-sub
```

### Usage

You will see a prompt after running the monitor.

Command:

```
> detail                              : get added node detail (id, connection)
> add broker|subscriber|publisher num : add nodes
> connect node_id1 node_id2           : add connection between node_id1 and node_id2
> subscribe publish_id node_id        : subscriber node_id will subscribe publish_id
> publish publish_id node_id          : publisher will send publish_id event
```

### Implementation

Each node will own one thread, and the whole network is in main thread.

Network monitor will maintain a thread safe queue, which will collect sent messages from each node thread.

Algorithm for communication protocol is based on Filtering. Also each node will maintain a log to avoid message twice retransmission.

