CXX = g++

.PHONY: all

all: pub-sub

pub-sub: net.hpp node.hpp repl.cc safequeue.hpp
	$(CXX) -I./ -std=c++11 -Winline -pthread repl.cc -o pub-sub
