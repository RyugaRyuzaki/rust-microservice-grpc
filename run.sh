#!/bin/bash

# Chạy các services trong background
cargo run -p api_gateway &
cargo run -p auth_service &
cargo run -p project_service &

# Đợi tất cả các process hoàn thành
wait