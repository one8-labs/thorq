import requests
import time
from datetime import datetime, timedelta, timezone

def get_current_time_plus_two_minutes():
    current_time = datetime.now(timezone.utc)
    new_time = current_time + timedelta(minutes=2)
    return new_time.isoformat().replace("+00:00", "Z")

url = "http://localhost:3000/create-job"
headers = {"Content-Type": "application/json"}

def create_api_call_mocks(num_jobs: int):
    for i in range(num_jobs):
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
            "run_at": get_current_time_plus_two_minutes()
        }

        try:
            response = requests.post(url, headers=headers, json=payload)
            print(f"Job {i+1}/{num_jobs}: Status {response.status_code}, Response: {response.text}")
        except requests.exceptions.RequestException as e:
            print(f"Job {i+1}/{num_jobs} failed: {e}")

        time.sleep(0.01)

def create_function_call_mocks(num_jobs: int):

    for i in range(num_jobs):
        py_fn = f"print('python function {i}')"
        node_fn = f"console.log('node function {i}')"
        
        idempotency_key = f"{int(time.time() * 1000)}_{i}"

        payload = {
            "job_type": "py_fn_call" if i % 2 == 0 else "js_fn_call",
            "payload": py_fn if i % 2 == 0 else node_fn,
            "queue_id": 1,
            "priority": 5,
            "max_attempts": 3,
            "t"
            "imeout_sec": 60,
            "idempotency_key": idempotency_key,
            "tenant_id": "tenant_123",
            "run_at": get_current_time_plus_two_minutes()
        }

        try:
            response = requests.post(url, headers=headers, json=payload)
            print(f"Job {i+1}/{num_jobs}: Status {response.status_code}, Response: {response.text}")
        except requests.exceptions.RequestException as e:
            print(f"Job {i+1}/{num_jobs} failed: {e}")



if __name__ == "__main__":
    # create_api_call_mocks(100)
    create_function_call_mocks(500)
