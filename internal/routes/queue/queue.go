package queue

import (
	"net/http"

	"app/internal/handler"

	"github.com/gin-gonic/gin"
	"gorm.io/gorm"
)

type CreateQueueRequest struct {
	Name        string   `json:"name" binding:"required"`
	Concurrency *int32   `json:"concurrency"`
	RatePerSec  *float64 `json:"rate_per_sec"`
}

func Register(r *gin.Engine, db *gorm.DB) {
	r.POST("/queues", func(c *gin.Context) {
		var req CreateQueueRequest

		if err := c.ShouldBindJSON(&req); err != nil {
			c.JSON(http.StatusBadRequest, gin.H{"error": err.Error()})
			return
		}

		queueID, err := handler.CreateQueue(db, req.Name, req.Concurrency, req.RatePerSec)
		if err != nil {
			c.JSON(http.StatusInternalServerError, gin.H{"error": err.Error()})
			return
		}

		c.JSON(http.StatusOK, gin.H{
			"queue_id": queueID,
		})
	})
}
