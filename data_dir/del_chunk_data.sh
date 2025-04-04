#!/bin/bash

find "chunk_data" -type f -delete

echo "All files in 'chunk-data' have been deleted."

find "recombine" -type f -delete

echo "Recombine directory cleared successfully!"