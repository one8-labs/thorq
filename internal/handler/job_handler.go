package handler

import (
	"net/http"
	"time"

	"app/internal/models"

	"github.com/gin-gonic/gin"
	"gorm.io/datatypes"
	"gorm.io/gorm"
)

type CreateJobRequest struct {
	QueueID int64          `json:"queue_id"`
	JobType string         `json:"job_type"`
	Payload datatypes.JSON `json:"payload"`
	RunAt   time.Time      `json:"run_at"`
}

func CreateJob(db *gorm.DB) gin.HandlerFunc {
	return func(c *gin.Context) {
		var req CreateJobRequest

		if err := c.ShouldBindJSON(&req); err != nil {
			c.JSON(http.StatusBadRequest, gin.H{"error": err.Error()})
			return
		}

		job := models.NewJob(req.QueueID, req.JobType, req.Payload, req.RunAt)

		if err := db.Create(job).Error; err != nil {
			c.JSON(http.StatusInternalServerError, gin.H{"error": err.Error()})
			return
		}

		c.JSON(http.StatusOK, job)
	}
}
