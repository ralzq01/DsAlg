#ifndef _NODE_H_
#define _NODE_H_

#include <iostream>
#include <unordered_map>
#include <unordered_set>
#include <safequeue.hpp>
#include <thread>
#include <mutex>
#include <condition_variable>
#include <chrono>
#include <algorithm>

using std::vector;
template <typename T>
using Set = std::unordered_set<T>;
using NodeId = int;
using SubId = int;
using PubId = int;

enum MsgType {Send = 0, Notify, Publish, Subscribe, Stop, Unknown};
typedef struct Msg{
  MsgType type;
  NodeId source;
  NodeId from_node;
  NodeId to_node;
  PubId pub_id;
  int id;
  Msg(MsgType type_, NodeId source_, NodeId from_node_)
      : type(type_)
      , source(source_)
      , from_node(from_node_)
      , to_node(-1)
      , pub_id(-1) {}
  Msg()
      : type(Unknown)
      , source(-1)
      , from_node(-1)
      , to_node(-1)
      , pub_id(-1) {}
}Msg;

enum NodeType {kBroker = 0, kPublisher, kSubscriber};

class Node{

 public:

  Node(NodeId id, NodeType type, SafeQueue<Msg> *disnet)
      : id_(id)
      , type_(type)
      , net_msg_(disnet){
    msg_id_ = 0;
    msg_handler_ = std::thread(&Node::recv, this);
  }

  virtual inline NodeId getId() const{
    return id_;
  }

  virtual inline void addNeighbor(NodeId id, NodeType type){
    neighbors_[id] = type;
  }

  virtual const std::unordered_map<NodeId, NodeType>& getNeighbors(){
    return neighbors_;
  }

  virtual inline NodeType getType() const{
    return type_;
  }

  virtual void recv(){
    Msg msg;
    while(true){
      net_msg_->front(msg);
      if(msg.type == Stop){
        break;
      }
      if(msg.to_node == this->id_){
        net_msg_->pop();
        if(!handled_log_.count(msg.source)){
          handled_log_[msg.source] = vector<int>();
        }
        const auto& handled = handled_log_[msg.source];
        auto res = std::find(handled.begin(), handled.end(), msg.id);
        if(res == handled.end()){
          handled_log_[msg.source].push_back(msg.id);
          printf("Node #%d recv msg: type: %d, source_id: %d, "\
                 "from_id: %d, to_id: %d, pub_id: %d, msg_id: %d\n",
                 id_, static_cast<int>(msg.type), msg.source,
                 msg.from_node, msg.to_node, msg.pub_id, msg.id);
          this->msgHandler(msg);
        }
      }
      else{
        std::this_thread::sleep_for(std::chrono::milliseconds(10));
      }
    }
  }

  ~Node(){
    msg_handler_.join();
  }

 protected:

  // send msg to to_id
  virtual inline void send(PubId publish_id, NodeId to_id, MsgType type, bool new_msg = true){
    Msg msg(type, this->id_, this->id_);
    msg.pub_id = publish_id;
    msg.to_node = to_id;
    msg.id = new_msg ? ++msg_id_ : msg_id_;
    net_msg_->push(msg);
  }

  virtual inline void send(Msg& msg, NodeId to_id, MsgType type){
    msg.type = type;
    msg.to_node = to_id;
    msg.from_node = this->id_;
    net_msg_->push(msg);
  }

  virtual void msgHandler(Msg& msg) = 0;
   
  NodeId id_;

  NodeType type_;

  std::unordered_map<NodeId, NodeType> neighbors_;

  std::unordered_map<NodeId, vector<int> > handled_log_;

  SafeQueue<Msg> *net_msg_;

  std::thread msg_handler_;

  int msg_id_;

};

class Broker: public Node{

 public:

  Broker(NodeId id, NodeType type, SafeQueue<Msg> *disnet) : Node(id, type, disnet) {}

  // recv publish_id from node_id
  void recvPublish(Msg& msg);

  // recv subscribe_id from node_id
  void recvSubscribe(Msg& msg);

 private:

  vector<NodeId> matchSub(PubId publish_id);

  vector<NodeId> matchRoute(PubId publish_id);

  virtual void msgHandler(Msg& msg);

  // routing table: subscription_id : [node_id list]
  std::unordered_map<PubId, vector<NodeId> > routing_;

  // subscription table: for broker connected directly with subscriber
  std::unordered_map<SubId, vector<NodeId> > subscriptions_;
};

class Subscriber: public Node{

 public:

  Subscriber(NodeId id, NodeType type, SafeQueue<Msg> *disnet) : Node(id, type, disnet) {}

  void sendRequest(int subscription_id);

 private:
  
  virtual void msgHandler(Msg& msg);

  std::unordered_set<PubId> subscriptions_;
  
};

class Publisher: public Node{

 public:

  Publisher(NodeId id, NodeType type, SafeQueue<Msg> *disnet) : Node(id, type, disnet) {}

  void sendNews(int publish_id);

 private:
  // publisher will do nothing for recved msg
  virtual void msgHandler(Msg& msg) {}
};

#endif