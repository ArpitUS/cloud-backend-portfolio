package main

import (
	"jobqueue"
)

func main() {
	queue := jobqueue.NewMemoryQueue()

	queue.Add("Send welcome email to user123")
	queue.Add("Generate monthly report")

	queue.Process()
}
