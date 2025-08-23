# Design

Problem Statement: Build a system which can call API with specified payload at a particular time.

Requirements:

- Pull events for this second (need millisecond precision)
- Execute them
- Retry if failed
- Push to DLQ (Dead Letter Queue) if exceeds max retries

