package handler

import (
	"app/internal/models"
	"app/internal/service"
	"app/internal/utils"
	"context"
	"net/http"
	"strconv"

	"github.com/gin-gonic/gin"
)

type UserHandler struct {
	service *service.UserService
	logger  *utils.AppLogger
}

func NewUserHandler(service *service.UserService, logger *utils.AppLogger) *UserHandler {
	return &UserHandler{service, logger}
}

func (h *UserHandler) CreateUser(c *gin.Context) {
	var req models.User

	if err := c.BindJSON(&req); err != nil {
		c.JSON(http.StatusBadRequest, gin.H{"error": "invalid request"})
		return
	}

	ctx := context.WithValue(context.Background(), "request_id", c.GetString("request_id"))

	if err := h.service.CreateUser(ctx, &req); err != nil {
		c.JSON(http.StatusInternalServerError, gin.H{"error": err.Error()})
		return
	}

	c.JSON(http.StatusCreated, req)
}

func (h *UserHandler) GetUserByID(c *gin.Context) {
	idParam := c.Param("id")

	id, err := strconv.Atoi(idParam)
	if err != nil {
		c.JSON(http.StatusBadRequest, gin.H{"error": "invalid id"})
		return
	}

	ctx := context.WithValue(context.Background(), "request_id", c.GetString("request_id"))

	user, err := h.service.GetUserByID(ctx, uint(id))
	if err != nil {
		c.JSON(http.StatusNotFound, gin.H{"error": err.Error()})
		return
	}

	c.JSON(http.StatusOK, user)
}

func (h *UserHandler) GetUserByEmail(c *gin.Context) {
	email := c.Param("email")

	ctx := context.WithValue(context.Background(), "request_id", c.GetString("request_id"))

	user, err := h.service.GetUserByEmail(ctx, email)
	if err != nil {
		c.JSON(http.StatusNotFound, gin.H{"error": err.Error()})
		return
	}

	c.JSON(http.StatusOK, user)
}
