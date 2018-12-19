CXX = g++

.PHONY: all

all: pub-sub

pub-sub: net.hpp node.cc node.hpp repl.cc safequeue.hpp
	$(CXX) -I./ -std=c++11 -Winline -pthread repl.cc node.cc -o pub-sub
