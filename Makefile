

all:
	GOOS=darwin GOARCH=amd64 go build -o main main.go
	# GOOS=linux  GOARCH=amd64 go build -o goody.Linux main.go

clean:
	rm -f main
