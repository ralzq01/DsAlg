
#ifndef _NET_H_
#define _NET_H_

#include <vector>
#include <node.hpp>
#include <queue>

class Net{

 public:
   
  Net() : unique_id_(0) {}

  void addBroker(int num){
    for(int i = 0; i < num; ++i){
      Broker* broker = new Broker(unique_id_++, kBroker, &net_msg_);
      brokers_.push_back(broker);
      node_list_.push_back(static_cast<Node*>(broker));
      std::cout << "broker added. #" << unique_id_ - 1 << std::endl;
    }
  }

  inline std::size_t getBrokerNum() const{
    return brokers_.size();
  }

  void addSubscriber(int num){
    for(int i = 0; i < num; ++i){
      Subscriber* subscriber = new Subscriber(unique_id_++, kSubscriber, &net_msg_);
      subscribers_.push_back(subscriber);
      node_list_.push_back(static_cast<Node*>(subscriber));
      std::cout << "subscriber added. #" << unique_id_ - 1 << std::endl;
    }
  }

  inline std::size_t getSubscriberNum() const{
    return subscribers_.size();
  }

  void addPublisher(int num){
    for(int i = 0; i < num; ++i){
      Publisher* publisher = new Publisher(unique_id_++, kPublisher, &net_msg_);
      publishers_.push_back(publisher);
      node_list_.push_back(static_cast<Node*>(publisher));
      std::cout << "publisher added. #" << unique_id_ - 1 << std::endl;
    }
  }

  inline std::size_t getPublisherNum(){
    return publishers_.size();
  }

  inline Node* getNode(NodeId id) const{
    if(id >= unique_id_){
      std::cerr << "Error: Node doesn't exist" << std::endl;
      return nullptr;
    }
    return node_list_[id];
  }

  bool addConnection(NodeId id1, NodeId id2){
    if(id1 > unique_id_ || id2 > unique_id_){
      std::cerr << "Error: Node Id doesn't exist." << std::endl;
      return false;
    }
    NodeType type_1 = node_list_[id1]->getType();
    NodeType type_2 = node_list_[id2]->getType();
    node_list_[id1]->addNeighbor(id2, type_2);
    node_list_[id2]->addNeighbor(id1, type_1);
    return true;
  }

  bool sendPublish(PubId pub_id, NodeId node_id){
    Node* node = node_list_[node_id];
    if(node->getType() != kPublisher){
      std::cerr << "Error: Can't send a publish event on non-publisher"
                << std::endl;
      return false;
    }
    dynamic_cast<Publisher*>(node)->sendNews(pub_id);
    return true;
  }

  bool sendSubscribe(PubId pub_id, NodeId node_id){
    Node* node = node_list_[node_id];
    if(node->getType() != kSubscriber){
      std::cerr << "Error: Can't send a subscribe event on non-subscriber"
                << std::endl;
      return false;
    }
    dynamic_cast<Subscriber*>(node)->sendRequest(pub_id);
    return true;
  }

  void getDetail(){
    std::cout << std::endl;
    for(NodeId id = 0; id < brokers_.size(); ++id){
      std::cout << "Broker #" << brokers_[id]->getId() << " ";
      auto neighbors = brokers_[id]->getNeighbors();
      std::cout << "connected with [";
      for(const auto& it : neighbors){
        std::cout << it.first << " ";
      }
      std::cout << "]" << std::endl;
    }
    for(NodeId id = 0; id < subscribers_.size(); ++id){
      std::cout << "Subscriber #" << subscribers_[id]->getId() << " ";
      auto neighbors = subscribers_[id]->getNeighbors();
      std::cout << "connected with [";
      for(const auto& it : neighbors){
        std::cout << it.first << " ";
      }
      std::cout << "]" << std::endl;
    }
    for(NodeId id = 0; id < publishers_.size(); ++id){
      std::cout << "Publisher #" << publishers_[id]->getId() << " ";
      auto neighbors = publishers_[id]->getNeighbors();
      std::cout << "connected with [";
      for(const auto& it : neighbors){
        std::cout << it.first << " ";
      }
      std::cout << "]" << std::endl;
    }
    std::cout << std::endl;
  }

  void quit(){
    Msg msg(Stop, -1, -1);
    net_msg_.push(msg);
    for(NodeId id = 0; id < unique_id_; ++id){
      delete node_list_[id];
    }
  }

 private:
  
  vector<Broker*> brokers_;

  vector<Subscriber*> subscribers_;

  vector<Publisher*> publishers_;

  vector<Node*> node_list_;

  SafeQueue<Msg> net_msg_;
  //std::queue<Msg> net_msg_;

  NodeId unique_id_;
};



#endif