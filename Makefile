CC = gcc
CFLAGS = `pkg-config --cflags gtk+-3.0` -Wall -g
LDFLAGS = `pkg-config --libs gtk+-3.0`

TARGET = wacom_tablet_app

all: $(TARGET)

$(TARGET): wacom_tablet_app.c
	$(CC) $(CFLAGS) -o $(TARGET) wacom_tablet_app.c $(LDFLAGS)

clean:
	rm -f $(TARGET)

.PHONY: all clean
