CC = clang
CXX = clang++
CFLAGS = -Wall -Wextra -O2
CXXFLAGS = -Wall -Wextra -O2 -std=c++17
SRC_DIR = src
BIN_DIR = bin
INPUTS_DIR = inputs

# Find all source files
C_SRCS = $(wildcard $(SRC_DIR)/*.c)
CPP_SRCS = $(wildcard $(SRC_DIR)/*.cpp)

# Define binaries
C_BINS = $(patsubst $(SRC_DIR)/%.c, $(BIN_DIR)/%_c, $(C_SRCS))
CPP_BINS = $(patsubst $(SRC_DIR)/%.cpp, $(BIN_DIR)/%_cpp, $(CPP_SRCS))

ALL_BINS = $(C_BINS) $(CPP_BINS)

.PHONY: all clean

all: $(BIN_DIR) $(ALL_BINS)

$(BIN_DIR):
	mkdir -p $(BIN_DIR)

# Rule for C binaries
$(BIN_DIR)/%_c: $(SRC_DIR)/%.c
	$(CC) $(CFLAGS) $< -o $@

# Rule for C++ binaries
$(BIN_DIR)/%_cpp: $(SRC_DIR)/%.cpp
	$(CXX) $(CXXFLAGS) $< -o $@

clean:
	rm -rf $(BIN_DIR)

