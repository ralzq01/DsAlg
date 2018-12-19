
#include <net.hpp>
#include <string>
#include <sstream>

using std::string;

void split(const std::string& str, std::vector<std::string>& v) {
  v.clear();
  std::stringstream ss(str);
  ss >> std::noskipws;
  std::string field;
  char ws_delim;
  while(1) {
    if(ss >> field)
      v.push_back(field);
    else if (ss.eof())
      break;
    else
      v.push_back(std::string());
    ss.clear();
    ss >> ws_delim;
  }
}

int main(){

  Net net;

  string command;
  vector<string> commands;

  while(true){

    while(!net.noMsg()){}

    std::cout << "> " << std::flush;
    // get new commands
    command.clear();
    std::getline(std::cin, command);
    split(command, commands);
    if(commands.size() == 0){
      continue;
    }

    if(commands[0] == "quit"){
      net.quit();
      return 0;
    }
    else if(commands[0] == "detail"){
      net.getDetail();
    }
    else if(commands[0] == "add"){
      if(commands.size() != 3){
        std::cerr << "add Usage: add [broker|publisher|subscriber] num" << std::endl;
        continue;
      }
      int num = std::stoi(commands[2]);
      if(commands[1] == "broker"){
        net.addBroker(num);
      }
      else if(commands[1] == "publisher"){
        net.addPublisher(num);
      }
      else if(commands[1] == "subscriber"){
        net.addSubscriber(num);
      }
      else{
        std::cerr << "add Usage: add [broker|publisher|subscriber] [num]" << std::endl;
      }
    }
    else if(commands[0] == "connect"){
      if(commands.size() != 3){
        std::cerr << "connect Usage: connect node_id_1 node_id_2" << std::endl;
        continue;
      }
      NodeId id1, id2;
      try{
        id1 = std::stoi(commands[1]);
        id2 = std::stoi(commands[2]);
      }catch(std::exception e){
        std::cerr << "last two parameters should be numbers" << std::endl;
        continue;
      }
      net.addConnection(id1, id2);
    }
    else if(commands[0] == "publish"){
      if(commands.size() != 3){
        std::cerr << "publish Usage: publish publish_id node_id" << std::endl;
        continue;
      }
      PubId pub_id;
      NodeId node_id;
      try{
        pub_id = std::stoi(commands[1]);
        node_id = std::stoi(commands[2]);
      }catch(std::exception e){
        std::cerr << "publish Usage: publish publish_id node_id" << std::endl;
        continue;
      }
      net.sendPublish(pub_id, node_id);
    }
    else if(commands[0] == "subscribe"){
      if(commands.size() != 3){
        std::cerr << "subscribe Usage: subscribe publish_id node_id" << std::endl;
        continue;
      }
      PubId pub_id;
      NodeId node_id;
      try{
        pub_id = std::stoi(commands[1]);
        node_id = std::stoi(commands[2]);
      }catch(std::exception e){
        std::cerr << "subscribe Usage: subscribe publish_id node_id" << std::endl;
        continue;
      }
      net.sendSubscribe(pub_id, node_id);
    }
    else{
      std::cerr << "Unknown Commands" << std::endl;
    }
  }

}