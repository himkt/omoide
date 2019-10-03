

all: omoide

omoide: main.go
	GOOS=darwin GOARCH=amd64 go build -o omoide main.go
	# GOOS=linux  GOARCH=amd64 go build -o omoide main.go

clean:
	rm -f omoide
