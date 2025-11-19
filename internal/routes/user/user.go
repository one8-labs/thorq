package user

import (
	"app/internal/handler"

	"app/internal/repository"
	"app/internal/service"
	"app/internal/utils"

	"github.com/gin-gonic/gin"
	"gorm.io/gorm"
)

func Register(r *gin.Engine, db *gorm.DB, logger *utils.AppLogger) {
    // DI for repository → service → handler
    userRepo := repository.NewUserRepository(db, logger)
    userService := service.NewUserService(userRepo, logger)
    userHandler := handler.NewUserHandler(userService, logger)

    user := r.Group("/api/users")
    {
        user.POST("/", userHandler.CreateUser)
        user.GET("/:id", userHandler.GetUserByID)
        user.GET("/email/:email", userHandler.GetUserByEmail)
    }
}
