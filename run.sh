#!/bin/bash

diesel migration run --database-url=$DATABASE_URL
./target/release/shorty
