CC = gcc
CFLAGS = -std=c11 -Wall -Wextra -g
SRC_DIR = src
BUILD_DIR = build

# List all .c files in the src directory
SRCS = $(wildcard $(SRC_DIR)/*.c)

# Generate corresponding .o file names
OBJS = $(SRCS:$(SRC_DIR)/%.c=$(BUILD_DIR)/%.o)

# Include directory
INC = -I$(SRC_DIR)

# The final executable
TARGET = db

all: $(BUILD_DIR) $(TARGET)

$(BUILD_DIR):
	mkdir -p $(BUILD_DIR)

$(TARGET): $(OBJS)
	$(CC) $(CFLAGS) $(OBJS) -o $@

$(BUILD_DIR)/%.o: $(SRC_DIR)/%.c
	$(CC) $(CFLAGS) $(INC) -c $< -o $@

clean:
	rm -rf $(BUILD_DIR) $(TARGET)

