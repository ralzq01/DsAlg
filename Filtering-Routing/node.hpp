#ifndef _NODE_H_
#define _NODE_H_

#include <iostream>
#include <unordered_map>
#include <unordered_set>
#include <safequeue.hpp>

using std::vector;
using NodeId = int;
using SubId = int;
using PubId = int;

enum MsgType {Send = 0, Notify, Publish, Subscribe};
typedef struct Msg{
  MsgType type;
  NodeId from_node;
  NodeId to_node;
  int id;
  Msg(MsgType type_, NodeId from_node_, NodeId to_node_, int id_)
      : type(type_)
      , from_node(from_node_)
      , to_node(to_node_)
      , id(id_) {}
}Msg;


class Node{

 public:

  Node(NodeId id, SafeQueue<Msg> *disnet)
      : id_(id)
      , net_msg_(disnet)
    {}

  virtual inline NodeId getId(){
    return id_;
  }

  virtual inline void addNeighbor(NodeId id){
    neighbors_.insert(id);
  }

  virtual const std::unordered_set<NodeId>& getNeighbors(){
    return neighbors_;
  }

 protected:
   
  NodeId id_;

  std::unordered_set<NodeId> neighbors_;

  SafeQueue<Msg> *net_msg_;

};

class Broker: public Node{

 public:

  Broker(NodeId id, SafeQueue<Msg> *disnet) : Node(id, disnet) {}

  void recvPublish(PubId publish_id, NodeId node_id) {};

  void recvSubscribe(SubId subscription_id, NodeId node_id) {};

 private:

  void sendPublish(PubId publish_id, NodeId node_id) {};
  
  void sendSubscribe(SubId subscription_id, NodeId node_id) {};

  vector<NodeId> matchSub(PubId publish_id) {};

  vector<NodeId> matchRoute(PubId publish_id) {};

  void notify(NodeId node_id) {};

  // routing table: subscription_id : [node_id list]
  std::unordered_map<PubId, vector<NodeId> > routing_;

  // subscription table: for broker connected directly with subscriber
  std::unordered_set<NodeId> subscriptions_;
};

class Subscriber: public Node{

 public:

  Subscriber(NodeId id, SafeQueue<Msg> *disnet) : Node(id, disnet) {}

  void sendRequest(int subscription_id) {};
  
};

class Publisher: public Node{

 public:

  Publisher(NodeId id, SafeQueue<Msg> *disnet) : Node(id, disnet) {}

  void sendNews(int publish_id) {};
};

#endif