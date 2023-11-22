package main

/*
#cgo LDFLAGS: -lsocket
#include <stdlib.h>

typedef struct Socket Socket;

Socket* create(char* ip);
void destroy(Socket* socket);

void send(Socket* socket, char* name);
char* listen(Socket* socket);
*/
import "C"

import (
	"unsafe"
	"fmt"
	"sync"
)

func main() {
	cip := C.CString("1.2.3.4")
	defer C.free(unsafe.Pointer(cip))

	csocket := C.create(cip);

	var wg sync.WaitGroup
	wg.Add(2)

	go func() {
		cdata := C.CString("hello from go!")
		defer C.free(unsafe.Pointer(cdata))

		C.send(csocket, cdata)

		defer wg.Done()
	}()

	go func() {
		cdata := C.listen(csocket);
		defer C.free(unsafe.Pointer(cdata))

		gdata := C.GoString(cdata)

		fmt.Println("<go> received:", gdata)

		defer wg.Done()
	}()

	wg.Wait()

	C.destroy(csocket);
}
