package job

import (
	"app/internal/handler"

	"github.com/gin-gonic/gin"
	"gorm.io/gorm"
)

func Register(r *gin.Engine, db *gorm.DB) {
	r.POST("/create-job", handler.CreateJob(db))
}
