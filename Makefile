CFLAGS = -Wall -Wextra -ggdb3 -std=c++11 -fvisibility=hidden -march=native -pipe -lm
main: src/main.cpp
	g++ $(CFLAGS) -o $@.o -lboost_system -pthread $^
clean:
	rm main
