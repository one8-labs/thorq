package db

import (
	"app/internal/config"
	"app/internal/models"
	"errors"
	"log"
	"time"

	"gorm.io/driver/postgres"
	"gorm.io/gorm"
	"gorm.io/gorm/logger"
)

func Connect(cfg *config.Config) (*gorm.DB, error) {
	var db *gorm.DB
	var err error

	// Retry connecting (5 attempts)
	for i := 0; i < 5; i++ {
		db, err = gorm.Open(postgres.Open(cfg.DatabaseURL), &gorm.Config{
			PrepareStmt: true,
			Logger: logger.Default.LogMode(func() logger.LogLevel {
				if cfg.Env == "prod" {
					return logger.Error
				}
				return logger.Warn
			}()),
		})

		if err == nil {
			break
		}

		log.Printf("DB connection failed (%v). Retrying in 2s...", err)
		time.Sleep(2 * time.Second)
	}

	if err != nil {
		return nil, errors.New("failed to connect to database: " + err.Error())
	}

	// Connection pooling
	sqlDB, err := db.DB()
	if err != nil {
		return nil, err
	}

	sqlDB.SetMaxIdleConns(10)
	sqlDB.SetMaxOpenConns(50)
	sqlDB.SetConnMaxLifetime(1 * time.Hour)
	sqlDB.SetConnMaxIdleTime(30 * time.Minute)

	// Auto-migrate only in dev
	if cfg.Env != "prod" {
		if err := db.AutoMigrate(&models.User{}); err != nil {
			return nil, err
		}
	}

	return db, nil
}
