package models

import "time"

type Queue struct {
	ID          int64     `gorm:"primaryKey;autoIncrement" json:"id"`
	Name        string    `json:"name"`
	Concurrency int32     `json:"concurrency"`
	RatePerSec  float64   `json:"rate_per_sec"`
	CreatedAt   time.Time `json:"created_at"`
}
