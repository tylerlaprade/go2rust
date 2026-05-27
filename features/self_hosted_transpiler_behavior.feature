# Self-hosted transpiler behavior 001
Feature: Self-hosted transpiler behavior

Background:
  Given the go2rust repository is checked out
  And the self-hosting fixture suite is available

# Self-hosted transpiler behavior 001
Scenario: Self-hosted transpiler behavior 001
  When the self-transpile cargo check gate runs
  Then the generated Rust transpiler workspace builds successfully

# Self-hosted transpiler behavior 002
Scenario Outline: Self-hosted transpiler behavior 002
  When the self-transpile behavior suite runs
  Then every non-XFAIL fixture produces the same output when run through the generated Rust transpiler as when run from the original Go source
  And the behavior run reports <regular_failures> non-XFAIL fixture failures
  And the behavior run reports at least <minimum_results> fixture result

Examples:
  | regular_failures | minimum_results |
  | 0                | 1               |
