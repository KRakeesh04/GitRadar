# GitRadar Analytics Metrics

## Overview

GitRadar provides local analytics for Git repositories by combining commit history, file-level change summaries, and working tree state.

This document defines the core metrics used by the application.

---

## Metric Categories

1. Repository activity
2. Commit behavior
3. File churn
4. Hotspots
5. Contributor activity
6. Repository health

---

## Repository Activity Metrics

### Total Commits
The number of indexed commits for a repository.

Formula:
- count of rows in `commits` for a given repository

### Commits Over Time
The number of commits grouped by day, week, or month.

Used for:
- trend charts
- active/inactive period analysis

### Last Activity Time
Timestamp of the most recent commit or working tree snapshot event.

Used for:
- repo freshness indicators
- sorting repositories by activity

---

## Commit Behavior Metrics

### Average Commit Size
Average number of changed lines per commit.

Formula:
- sum of `total_changes` across all commit file stats / number of commits

### Large Commit Count
Number of commits above a configurable threshold.

Example threshold:
- commits where total changed lines > 500

Used for:
- health scoring
- identifying risky changes

### Commit Frequency
How often commits occur during a selected period.

Representations:
- commits per day
- commits per week
- moving average

---

## File Churn Metrics

### File Touch Count
How many commits touched a file.

Formula:
- count of distinct commits for a given file

### File Churn Score
A measure of how unstable a file is.

Suggested formula:
- churn score = sum(additions + deletions) across all commits affecting the file

This helps identify files that are edited frequently and heavily.

### Recent File Activity
Files changed most often in the recent time window.

Suggested default windows:
- 7 days
- 30 days
- 90 days

---

## Hotspot Metrics

### Hotspot Score
A weighted metric for risky files that are both frequently touched and heavily changed.

Suggested formula:
- hotspot score = log(1 + touch_count) * churn_score

Alternative formula:
- hotspot score = (touch_count * 0.4) + (churn_score * 0.6)

Used for:
- top hotspot lists
- repository risk indicators

### Hotspot File Count
The number of files whose hotspot score exceeds a configured threshold.

Used for:
- repo health overview
- risk summaries

---

## Contributor Metrics

### Commit Count by Author
Number of commits grouped by author.

Used for:
- contributor distribution
- top contributor analytics

### Churn by Author
Total changed lines grouped by author.

Used for:
- workload estimation
- identifying major contributors

### Active Days by Author
Number of days on which an author committed.

Used for:
- consistency/activity indicators

### Bus Factor Estimate
A rough indicator of how concentrated knowledge is.

Simple MVP approximation:
- percentage of total commits made by top contributor

Interpretation example:
- if one contributor made 80% of commits, bus factor risk is high

---

## Working Tree Metrics

### Modified File Count
Number of currently modified files.

### Staged File Count
Number of staged files.

### Untracked File Count
Number of untracked files.

### Deleted File Count
Number of deleted files in working tree state.

Used for:
- current repo state panel
- "dirty repo" indication

---

## Health Metrics

### Health Score
A derived score between 0 and 100 that summarizes repository maintainability signals.

Suggested inputs:
- large commit frequency
- stale branch count
- hotspot file count
- average commit size
- working tree cleanliness

Example rough formula:
- start from 100
- subtract penalty for very large commits
- subtract penalty for many stale branches
- subtract penalty for many hotspot files
- subtract penalty for permanently dirty working tree

This score is heuristic and should not be presented as absolute truth.

### Stale Branch Count
Branches not updated within a configurable period.

Suggested default:
- 90 days

Used for:
- cleanup recommendations
- health scoring

---

## MVP Metrics

The first version of GitRadar should implement these first:

- total commits
- commits over time
- last activity time
- modified/staged/untracked/deleted counts
- file touch count
- churn score
- hotspot score
- commit count by author
- health score

---

## Future Metrics

Possible future additions:
- commit message quality indicators
- refactor-heavy file detection
- binary file growth
- merge frequency
- branch lifetime
- file ownership confidence
- bug-fix commit correlation