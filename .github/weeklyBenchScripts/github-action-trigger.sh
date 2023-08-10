#!/bin/bash

echo "Triggering behchmark trigger"
sshpass -p $BENCH_RESULTS_PASS ssh -t -t -o StrictHostKeyChecking=no ubuntu@43.130.90.57 "bash -s" -- "$GITHUB_RUN_ID" << EOF
$(<bench-results-trigger.sh)
RESULT=$?
echo "exiting github-acton-trigger with RESULT $RESULT" >&2
exit $RESULT
pkill ssh
EOF

echo "Exiting github-action-trigger"
exit 0
