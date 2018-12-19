
#include <node.hpp>

void Broker::recvPublish(Msg& msg){
  vector<NodeId> match_list = matchSub(msg.pub_id);
  for(NodeId id : match_list){
    this->send(msg, id, Notify);
  }
  vector<NodeId> fwd_list = matchRoute(msg.pub_id);
  for(NodeId id : fwd_list){
    if(id != msg.from_node){
      this->send(msg, id, Publish);
    }
  }
}

void Broker::recvSubscribe(Msg& msg){
  if(!this->neighbors_.count(msg.from_node)){
    std::cerr << "Error: Broker recv subscription from unconnected node: " 
              << msg.from_node << std::endl;
    return;
  }
  if(this->neighbors_[msg.from_node] == kSubscriber){
    if(!subscriptions_.count(msg.pub_id)){
      subscriptions_[msg.pub_id] = vector<NodeId>();
    }
    subscriptions_[msg.pub_id].push_back(msg.from_node);
  }
  else{
    if(!routing_.count(msg.pub_id)){
      routing_[msg.pub_id] = vector<NodeId>();
    }
    routing_[msg.pub_id].push_back(msg.from_node);
  }
  for(const auto& it : this->neighbors_){
    if(it.first != msg.from_node){
      this->send(msg, it.first, Subscribe);
    }
  }
}

vector<NodeId> Broker::matchSub(PubId publish_id){
  if(subscriptions_.count(publish_id)){
    return subscriptions_[publish_id];
  }
  return vector<NodeId>();
}

vector<NodeId> Broker::matchRoute(PubId publish_id){
  if(routing_.count(publish_id)){
    return routing_[publish_id];
  }
  return vector<NodeId>();
}

void Broker::msgHandler(Msg& msg){
  PubId pub_id = msg.id;
  NodeId from_node = msg.from_node;
  switch(msg.type){
    case Publish: {
      recvPublish(msg);
      break;
    }
    case Subscribe: {
      recvSubscribe(msg);
      break;
    }
    default: {
      std::cerr << "Broker recv error msg" << std::endl;
    }
  }
}

void Subscriber::sendRequest(int subscription_id){
  for(const auto& it : this->neighbors_){
    if(it.second == kBroker){
      this->send(subscription_id, it.first, Subscribe);
      printf("send to #%d\n", it.first);
    }
    subscriptions_.insert(subscription_id);
  }
}

void Subscriber::msgHandler(Msg& msg){
  PubId pub_id = msg.pub_id;
  NodeId from_node = msg.from_node;
  switch(msg.type){
    case Notify: {
      if(subscriptions_.find(pub_id) != subscriptions_.end()){
        std::cout << "Subscriber #" << this->id_ 
                  << ": recv notify of Publish id:"
                  << pub_id << std::endl;
      }
      break;
    }
  }
}

void Publisher::sendNews(int publish_id){
  for(const auto& it : this->neighbors_){
    if(it.second == kSubscriber){
      this->send(publish_id, it.first, Notify);
    }
    else if(it.second == kBroker){
      this->send(publish_id, it.first, Publish);
    }
  }
}
