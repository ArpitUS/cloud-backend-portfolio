package jobqueue

import "go-job-queue-worker/worker"

type MemoryQueue struct {
	jobs []string
}

func NewMemoryQueue() *MemoryQueue {
	return &MemoryQueue{}
}

func (q *MemoryQueue) Add(job string) {
	q.jobs = append(q.jobs, job)
}

func (q *MemoryQueue) Process() {
	for _, job := range q.jobs {
		worker.ProcessJob(job)
	}
}
