build:
	go build -race -tags debug -o bin/matrixd-x86_64 cmd/main.go
	chmod 766 bin/matrixd-x86_64

# https://github.com/golang/go/wiki/GoArm
build-arm32:
	env GOOS=linux GOARCH=arm GOARM=7 go build -tags debug -o bin/matrixd-arm cmd/main.go
	chmod 766 bin/matrixd-arm

build-arm64:
	env GOOS=linux GOARCH=arm64 go build -tags debug -o bin/matrixd-arm64 cmd/main.go
	chmod 766 bin/matrixd-arm64

clean:
	rm -rf bin
