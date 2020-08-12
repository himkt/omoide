PLATFORM = $(shell uname| tr '[:upper:]' '[:lower:]')


all: omoide

omoide: main.go
	GOOS=$(PLATFORM) GOARCH=amd64 go build -o omoide main.go

clean:
	rm -f omoide
