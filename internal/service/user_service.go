package service

import (
	"app/internal/models"
	"app/internal/repository"
	"app/internal/utils"
	"context"
	"fmt"
)

type UserService struct {
    repo   repository.UserRepository
    logger *utils.AppLogger
}

func NewUserService(repo repository.UserRepository, logger *utils.AppLogger) *UserService {
    return &UserService{
        repo:   repo,
        logger: logger,
    }
}

func (s *UserService) CreateUser(ctx context.Context, user *models.User) error {
    reqID := ctx.Value("request_id")

    s.logger.WithFields(map[string]interface{}{
        "email":      user.Email,
        "request_id": reqID,
    }).Info("Attempting to create user")

    if err := s.repo.Create(user); err != nil {
        s.logger.WithFields(map[string]interface{}{
            "email":      user.Email,
            "request_id": reqID,
            "error":      err.Error(),
        }).Error("Failed to create user")

        return fmt.Errorf("could not create user: %w", err)
    }

    s.logger.WithFields(map[string]interface{}{
        "id":         user.ID,
        "email":      user.Email,
        "request_id": reqID,
    }).Info("User created successfully")

    return nil
}

func (s *UserService) GetUserByID(ctx context.Context, id uint) (*models.User, error) {
    reqID := ctx.Value("request_id")

    s.logger.WithFields(map[string]interface{}{
        "id":         id,
        "request_id": reqID,
    }).Info("Fetching user by ID")

    user, err := s.repo.FindByID(id)
    if err != nil {
        s.logger.WithFields(map[string]interface{}{
            "id":         id,
            "request_id": reqID,
            "error":      err.Error(),
        }).Error("Failed to fetch user")

        return nil, fmt.Errorf("could not find user: %w", err)
    }

    return user, nil
}

func (s *UserService) GetUserByEmail(ctx context.Context, email string) (*models.User, error) {
    reqID := ctx.Value("request_id")

    s.logger.WithFields(map[string]interface{}{
        "email":      email,
        "request_id": reqID,
    }).Info("Fetching user by email")

    user, err := s.repo.FindByEmail(email)
    if err != nil {
        s.logger.WithFields(map[string]interface{}{
            "email":      email,
            "request_id": reqID,
            "error":      err.Error(),
        }).Error("Failed to fetch user")

        return nil, fmt.Errorf("could not find user: %w", err)
    }

    return user, nil
}
