package models

import (
	"time"

	"gorm.io/datatypes"
)

type Job struct {
	ID             int64          `gorm:"primaryKey;autoIncrement" json:"id"`
	QueueID        int64          `json:"queue_id"`
	JobType        string         `json:"job_type"`
	Payload        datatypes.JSON `json:"payload"` // JSONB
	Priority       int32          `json:"priority"`
	Status         string         `json:"status"`
	RunAt          time.Time      `json:"run_at"`
	InsertedAt     time.Time      `json:"inserted_at"`
	UpdatedAt      time.Time      `json:"updated_at"`
	Attempts       int32          `json:"attempts"`
	MaxAttempts    int32          `json:"max_attempts"`
	LastError      *string        `json:"last_error"` // nullable
	TimeoutSec     int32          `json:"timeout_sec"`
	IdempotencyKey *string        `json:"idempotency_key"`
	TenantID       *string        `json:"tenant_id"`
}

func NewJob(queueID int64, jobType string, payload datatypes.JSON, runAt time.Time) *Job {
	now := time.Now().UTC()

	return &Job{
		ID:             0,
		QueueID:        queueID,
		JobType:        jobType,
		Payload:        payload,
		Priority:       0,
		Status:         "pending",
		RunAt:          runAt,
		InsertedAt:     now,
		UpdatedAt:      now,
		Attempts:       0,
		MaxAttempts:    25,
		LastError:      nil,
		TimeoutSec:     300,
		IdempotencyKey: nil,
		TenantID:       nil,
	}
}
