package worker

import (
	"bytes"
	"encoding/json"
	"fmt"
	"net/http"
	"os"
	"os/exec"
	"time"

	"app/internal/models"

	"gorm.io/gorm"
	"gorm.io/gorm/clause"
)

func FetchReadyJobs(db *gorm.DB, now time.Time) ([]models.Job, error) {
	tx := db.Begin()
	if tx.Error != nil {
		return nil, tx.Error
	}

	var jobs []models.Job

	err := tx.
		Where("status = ? AND run_at <= ?", "pending", now).
		Order("priority DESC, run_at ASC").
		Clauses(clause.Locking{Strength: "UPDATE", Options: "SKIP LOCKED"}).
		Find(&jobs).Error

	if err != nil {
		tx.Rollback()
		return nil, err
	}

	ids := make([]int64, 0)
	for _, j := range jobs {
		ids = append(ids, j.ID)
	}

	if len(ids) > 0 {
		err = tx.Model(&models.Job{}).
			Where("id IN ?", ids).
			Updates(map[string]interface{}{
				"status":     "executing",
				"updated_at": time.Now().UTC(),
			}).Error

		if err != nil {
			tx.Rollback()
			return nil, err
		}
	}

	return jobs, tx.Commit().Error
}

func UpdateJobStatus(db *gorm.DB, jobID int64, status string) error {
	return db.Model(&models.Job{}).
		Where("id = ?", jobID).
		Updates(map[string]interface{}{
			"status":     status,
			"updated_at": time.Now().UTC(),
		}).Error
}

type ApiCallPayload struct {
	URL    string          `json:"url"`
	Method string          `json:"method"`
	Body   json.RawMessage `json:"body"`
}

func PerformApiCall(payload ApiCallPayload) error {
	fmt.Println("🌐 API call:", payload.URL)

	client := &http.Client{}

	var req *http.Request
	var err error

	if payload.Method == "POST" {
		req, err = http.NewRequest("POST", payload.URL, bytes.NewBuffer(payload.Body))
		req.Header.Set("Content-Type", "application/json")
	} else {
		req, err = http.NewRequest("GET", payload.URL, nil)
	}

	if err != nil {
		return err
	}

	resp, err := client.Do(req)
	if err != nil {
		return err
	}
	defer resp.Body.Close()

	fmt.Println("API STATUS:", resp.StatusCode)
	return nil
}

func uniqueFilename(ext string) string {
	return fmt.Sprintf("/tmp/job_%d.%s", time.Now().UnixNano(), ext)
}

func PerformFunctionExecution(interpreter, code string) (string, error) {
	var tmpFile string
	if interpreter == "python3" {
		tmpFile = uniqueFilename("py")
	} else if interpreter == "node" {
		tmpFile = uniqueFilename("js")
	} else {
		tmpFile = uniqueFilename("tmp")
	}

	if err := os.WriteFile(tmpFile, []byte(code), 0644); err != nil {
		return "", fmt.Errorf("failed to write code to file: %w", err)
	}
	defer func() {
		if err := os.Remove(tmpFile); err != nil {
			fmt.Printf("Warning: failed to remove temp file %s: %v\n", tmpFile, err)
		}
	}()

	cmd := exec.Command(interpreter, tmpFile)
	out, err := cmd.CombinedOutput()

	if err != nil {
		return "", fmt.Errorf("execution failed: %w, output: %s", err, string(out))
	}

	return string(out), nil
}

func ProcessJob(db *gorm.DB, job models.Job) {
	switch job.JobType {

	case "api_call":
		var payload ApiCallPayload
		if err := json.Unmarshal(job.Payload, &payload); err != nil {
			fmt.Printf("Failed to deserialize API call payload for job %d: %v\n", job.ID, err)
			UpdateJobStatus(db, job.ID, "failed")
			return
		}

		if err := PerformApiCall(payload); err != nil {
			fmt.Printf("API call failed for job %d: %v\n", job.ID, err)
			UpdateJobStatus(db, job.ID, "failed")
			return
		}
		UpdateJobStatus(db, job.ID, "executed")

	case "py_fn_call":
		// The payload is stored as a JSON string, so we need to unmarshal it
		var code string
		if err := json.Unmarshal(job.Payload, &code); err != nil {
			fmt.Printf("Failed to deserialize Python code for job %d: %v\n", job.ID, err)
			UpdateJobStatus(db, job.ID, "failed")
			return
		}

		result, err := PerformFunctionExecution("python3", code)
		if err != nil {
			fmt.Printf("Python execution failed for job %d: %v\n", job.ID, err)
			UpdateJobStatus(db, job.ID, "failed")
			return
		}
		fmt.Printf("Python execution result for job %d: %s\n", job.ID, result)
		UpdateJobStatus(db, job.ID, "executed")

	case "js_fn_call":
		// The payload is stored as a JSON string, so we need to unmarshal it
		var code string
		if err := json.Unmarshal(job.Payload, &code); err != nil {
			fmt.Printf("Failed to deserialize Node code for job %d: %v\n", job.ID, err)
			UpdateJobStatus(db, job.ID, "failed")
			return
		}

		result, err := PerformFunctionExecution("node", code)
		if err != nil {
			fmt.Printf("Node execution failed for job %d: %v\n", job.ID, err)
			UpdateJobStatus(db, job.ID, "failed")
			return
		}
		fmt.Printf("Node execution result for job %d: %s\n", job.ID, result)
		UpdateJobStatus(db, job.ID, "executed")

	default:
		fmt.Printf("Unknown job type: %s for job %d\n", job.JobType, job.ID)
	}
}

func StartWorker(db *gorm.DB) {
	fmt.Println("⚙️ Worker started...")

	const LIMIT = 100
	semaphore := make(chan struct{}, LIMIT)

	ticker := time.NewTicker(1 * time.Second)

	for range ticker.C {
		now := time.Now().UTC()

		jobs, err := FetchReadyJobs(db, now)
		if err != nil {
			fmt.Println("Error fetching jobs:", err)
			continue
		}

		for _, job := range jobs {
			semaphore <- struct{}{}

			go func(j models.Job) {
				defer func() { <-semaphore }()
				ProcessJob(db, j)
			}(job)
		}
	}
}
