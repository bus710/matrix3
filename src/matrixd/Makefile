build:
	go build -race -tags debug -o bin/matrixd-x86_64 cmd/main.go

# https://github.com/golang/go/wiki/GoArm
cross-build:
	env GOOS=linux GOARCH=arm GOARM=7 go build -tags debug -o bin/matrixd-ARM cmd/main.go
	env GOOS=linux GOARCH=arm64 go build -tags debug -o bin/matrixd-ARM64 cmd/main.go

clean:
	rm -rf bin