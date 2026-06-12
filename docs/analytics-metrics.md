# GitRadar Analytics Specification

Version: 2.0

---

# Dashboard Metrics

## Total Repositories

Number of tracked repositories.

---

## Active Repositories

Repositories with activity in last 30 days.

---

## Total Commits

Indexed commits across all repositories.

---

## Recent Activity

Repositories active within:

* 24 hours
* 7 days
* 30 days

---

# Repository Activity Metrics

## Commit Frequency

Commits:

* Daily
* Weekly
* Monthly

---

## Commit Velocity

Formula:

Commits / Active Days

---

## Activity Score

Formula:

```text
Recent Commits
+
Working Tree Activity
+
Branch Activity
```

Range:

0–100

---

# Branch Analytics

## Total Branches

---

## Active Branches

Branches updated in last 30 days.

---

## Stale Branches

Default threshold:

90 days

---

## Branch Divergence

Measure distance from default branch.

---

# Commit Analytics

## Total Commits

---

## Average Commit Size

---

## Large Commit Count

Threshold:

500 lines

---

## Merge Commit Count

---

## Commit Timeline

Visualization source.

---

# File Analytics

## Touch Count

---

## Churn Score

---

## Hotspot Score

Formula:

log(1 + touch_count) × churn

---

## Most Modified Files

---

## Recently Modified Files

7 days

30 days

90 days

---

# Contributor Analytics

## Commit Count

---

## Active Days

---

## Churn Contribution

---

## Bus Factor Estimate

Knowledge concentration indicator.

---

# Working Tree Metrics

## Modified Files

## Staged Files

## Deleted Files

## Untracked Files

---

## Dirty Repository Score

Formula:

Modified
+
Staged
+
Untracked

---

# Repository Health

Inputs:

* Stale branches
* Large commits
* Hotspots
* Dirty working tree

Output:

0–100

---

# Repository Ranking

Used on dashboard.

Formula:

Activity Score
+
Health Score
+
Recent Commit Weight

---

# Search Analytics

Track:

* Most viewed repositories
* Most viewed files
* Most viewed commits

Local only.

---

# Future WakaTime Analytics

## Coding Time Today

## Coding Time This Week

## Coding Time This Month

## Language Usage

## Editor Usage

## Productivity Trend

---

# Future Git Metrics

## Pull Frequency

## Push Frequency

## Merge Frequency

## Branch Lifetime

## Release Frequency

---

# MVP Metrics

Implement first:

✓ Total repositories

✓ Total commits

✓ Commit frequency

✓ Hotspot score

✓ Churn score

✓ Contributors

✓ Health score

✓ Working tree metrics

✓ Commit graph metrics

✓ Activity ranking

Post-MVP:

✓ WakaTime analytics

✓ Git operation analytics

✓ Repository growth metrics
