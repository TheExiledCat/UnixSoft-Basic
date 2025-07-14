# Compiler and flags
CC = gcc
CFLAGS = -Wall -Wextra -O2 -fPIC
AR = ar
ARFLAGS = rcs

# Directories
SRC_DIR = .
BUILD_DIR = build
INCLUDE_DIR = include

# Target library
LIB_NAME = libusblib.a

# Source files
SRCS = $(wildcard $(SRC_DIR)/*.c)

# Object files
OBJS = $(patsubst $(SRC_DIR)/%.c,$(BUILD_DIR)/%.o,$(SRCS))

.PHONY: all clean

all: $(BUILD_DIR)/$(LIB_NAME)

# Compile .c to .o
$(BUILD_DIR)/%.o: $(SRC_DIR)/%.c | $(BUILD_DIR)
	$(CC) $(CFLAGS) -I$(INCLUDE_DIR) -c $< -o $@

# Create static library
$(BUILD_DIR)/$(LIB_NAME): $(OBJS)
	$(AR) $(ARFLAGS) $@ $^

# Create build dir if missing
$(BUILD_DIR):
	mkdir -p $(BUILD_DIR)

clean:
	rm -rf $(BUILD_DIR)
