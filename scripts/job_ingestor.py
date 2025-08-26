import requests
import time

def create_jobs(num_jobs: int):
    url = "http://localhost:3000/create-job"
    headers = {"Content-Type": "application/json"}

    for i in range(num_jobs):
        # unique idempotency key using timestamp + counter
        idempotency_key = f"{int(time.time() * 1000)}_{i}"

        payload = {
            "job_type": "api_call",
            "payload": {
                "url": "https://jsonplaceholder.typicode.com/todos/1",
                "method": "GET"
            },
            "queue_id": 1,
            "priority": 5,
            "max_attempts": 3,
            "timeout_sec": 60,
            "idempotency_key": idempotency_key,
            "tenant_id": "tenant_123",
            "run_at": "2025-08-26T18:39:02Z"
        }

        try:
            response = requests.post(url, headers=headers, json=payload)
            print(f"Job {i+1}/{num_jobs}: Status {response.status_code}, Response: {response.text}")
        except requests.exceptions.RequestException as e:
            print(f"Job {i+1}/{num_jobs} failed: {e}")

        time.sleep(0.01) 

if __name__ == "__main__":
    create_jobs(100)
