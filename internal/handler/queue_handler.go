package handler

import (
	"time"

	"app/internal/models"

	"gorm.io/gorm"
)

func CreateQueue(db *gorm.DB, name string, concurrency *int32, ratePerSec *float64) (int64, error) {
	con := int32(10)
	rate := float64(100)

	if concurrency != nil {
		con = *concurrency
	}

	if ratePerSec != nil {
		rate = *ratePerSec
	}

	queue := models.Queue{
		Name:        name,
		Concurrency: con,
		RatePerSec:  rate,
		CreatedAt:   time.Now().UTC(),
	}

	if err := db.Create(&queue).Error; err != nil {
		return 0, err
	}

	return queue.ID, nil
}
