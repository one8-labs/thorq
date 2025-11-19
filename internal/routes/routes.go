package routes

import (
	"app/internal/routes/health"
	"app/internal/routes/job"
	"app/internal/routes/queue"
	"app/internal/routes/sample"
	"app/internal/routes/user"
	"app/internal/utils"

	"github.com/gin-gonic/gin"
	"gorm.io/gorm"
)

func Register(r *gin.Engine, db *gorm.DB) {
	health.Register(r)
	sample.Register(r)
	user.Register(r, db, utils.Logger)
	job.Register(r, db)
	queue.Register(r, db)
}
