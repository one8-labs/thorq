package utils

import (
	"time"

	"github.com/didip/tollbooth/v7"
	"github.com/didip/tollbooth_gin"
	"github.com/gin-gonic/gin"
)

func RateLimiter() gin.HandlerFunc {
	// 4 requests per 10 minutes. Change this based on your requirements
	limiter := tollbooth.NewLimiter(4.0/(60.0*10), nil)

	limiter.SetBurst(4)
	limiter.SetTokenBucketExpirationTTL(10 * time.Minute)
	limiter.SetMessage("Too many requests. Please wait a bit and try again.")

	return tollbooth_gin.LimitHandler(limiter)
}
