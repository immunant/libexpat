#!/bin/bash

set -euo pipefail

usage() {
  cat <<'EOF'
usage: ./migrate.sh [options]

Options:
  --list-steps           Print the configured migration steps and exit
  --from-step N          Start at step N (1-based, default: 1)
  --to-step N            Stop after step N (1-based, default: last step)
  --run-dir PATH         Write artifacts to PATH instead of target/migrate/<timestamp>
  --max-attempts N       Maximum attempts per step (default: 8)
  --codex-model MODEL    Pass --model MODEL to codex exec
  --claude-model MODEL   Pass --model MODEL to claude
  -h, --help             Show this help and exit
EOF
}

log() {
  printf '[migrate] %s\n' "$*" >&2
}

die() {
  log "error: $*"
  exit 1
}

require_command() {
  local tool="$1"
  command -v "$tool" >/dev/null 2>&1 || die "'$tool' is not in PATH"
}

script_dir="$(cd "$(dirname "${BASH_SOURCE[0]}")" && pwd)"
cd "$script_dir"

declare -a step_legacy_shas=()
declare -a step_kinds=()
declare -a step_texts=()

load_steps() {
  local legacy_sha kind text

  while IFS=$'\t' read -r legacy_sha kind text; do
    [[ -n "$kind" ]] || continue
    if [[ "$legacy_sha" == "-" ]]; then
      legacy_sha=""
    fi
    case "$kind" in
      prompt|command) ;;
      *) die "unsupported step kind '$kind'" ;;
    esac
    step_legacy_shas+=("$legacy_sha")
    step_kinds+=("$kind")
    step_texts+=("$text")
  done <<'EOF'
0d7b9639	prompt	remove no_mangle attribute and pub extern "C" from all rust functions that are neither listed as XMLPARSEAPI in expat/lib/expat.h nor declared in expat/lib/internal.h
04405edd	prompt	in every rust module, import at the top every definition used 3 or more times
811d6d7a	prompt	fix all rust cargo warnings; rustfmt reflow is fine, do not use allow(unused_unsafe)
fbf8330b	prompt	replace __assert_fail checks with rust assert macro
720fa2df	prompt	eliminate Option from function pointers that are never None
66ad0e93	prompt	remove extern C from crate-local Rust functions
428c0588	prompt	change the types of m_tagStack, m_freeTagList, and tag::parent to Option<Box<tag>>
742230fc	prompt	initialize encodings and encodingsNS idiomatically and get rid of run_static_initializers in xmltok.rs
abfced74	prompt	replace the element types of encodings and encodingsNS in xmltok.rs with references instead of pointers and update all uses
c3d9aa9e	prompt	make all statics immutable if mut is not needed
a1423bcd	prompt	Run shorten_imported_paths
170eb839	prompt	import every fully qualified Rust path that is used 3 or more times
f4db8628	prompt	replace all const encoding raw pointers with Rust references
4424cd8e	prompt	replace all raw pointers to ENCODING with Rust references in xmltok.rs
1585113a	prompt	remove if statements where the condition is a provably false or true check like 1 > 1 or 0 != 0
476d3646	prompt	replace literals matching against type_0 in xmltok.rs with their corresponding BT_ consts
EOF
}

list_steps() {
  local total_steps index legacy_sha kind text

  total_steps="${#step_texts[@]}"
  for ((index = 0; index < total_steps; index += 1)); do
    legacy_sha="${step_legacy_shas[index]}"
    kind="${step_kinds[index]}"
    text="${step_texts[index]}"
    if [[ -n "$legacy_sha" ]]; then
      printf '%2d. [%s, legacy %s] %s\n' \
        "$((index + 1))" "$kind" "$legacy_sha" "$text"
    else
      printf '%2d. [%s] %s\n' \
        "$((index + 1))" "$kind" "$text"
    fi
  done
}

ensure_clean_worktree() {
  local status_output

  status_output="$(git status --porcelain --untracked-files=all)"
  [[ -z "$status_output" ]] || die "worktree must be clean before running migrate.sh"
}

sanitize_claude_project_dir() {
  printf '%s' "$script_dir" | sed 's#/#-#g'
}

write_review_schema() {
  cat >"$review_schema_file" <<'EOF'
{
  "type": "object",
  "properties": {
    "decision": {
      "type": "string",
      "enum": ["approve", "request_changes"]
    },
    "summary": {
      "type": "string"
    },
    "required_changes": {
      "type": "array",
      "items": {
        "type": "string"
      }
    }
  },
  "required": ["decision", "summary", "required_changes"],
  "additionalProperties": false
}
EOF
}

write_run_config() {
  jq -n \
    --arg started_at "$(date -u +"%Y-%m-%dT%H:%M:%SZ")" \
    --arg script_dir "$script_dir" \
    --argjson from_step "$from_step" \
    --argjson to_step "$to_step" \
    --argjson max_attempts "$max_attempts" \
    --arg codex_model "$codex_model" \
    --arg claude_model "$claude_model" \
    '{
      started_at: $started_at,
      script_dir: $script_dir,
      from_step: $from_step,
      to_step: $to_step,
      max_attempts: $max_attempts,
      codex_model: (if $codex_model == "" then null else $codex_model end),
      claude_model: (if $claude_model == "" then null else $claude_model end)
    }' >"$run_dir/config.json"
}

setup_run_dir() {
  if [[ -z "$run_dir" ]]; then
    run_dir="$script_dir/target/migrate/$(date -u +"%Y%m%dT%H%M%SZ")"
  fi

  if [[ -e "$run_dir" ]]; then
    if [[ -n "$(find "$run_dir" -mindepth 1 -maxdepth 1 -print -quit 2>/dev/null)" ]]; then
      die "run directory '$run_dir' already exists and is not empty"
    fi
  fi

  mkdir -p "$run_dir"
  review_schema_file="$run_dir/review-schema.json"
  summary_jsonl_file="$run_dir/summary.jsonl"
  final_summary_file="$run_dir/summary.json"
  write_review_schema
  : >"$summary_jsonl_file"
  write_run_config
}

assert_integer() {
  local name="$1"
  local value="$2"

  [[ "$value" =~ ^[0-9]+$ ]] || die "$name must be a non-negative integer"
}

has_repo_changes() {
  local status_output

  status_output="$(git status --porcelain --untracked-files=all)"
  [[ -n "$status_output" ]]
}

tail_file_or_empty() {
  local file="$1"
  local lines="${2:-80}"

  if [[ -f "$file" ]]; then
    tail -n "$lines" "$file"
  fi
}

aggregate_usage_files() {
  local output_file="$1"
  shift

  if [[ "$#" -eq 0 ]]; then
    jq -n '{}' >"$output_file"
    return
  fi

  jq -s '
    reduce .[] as $item (
      {};
      reduce ($item | to_entries[]) as $entry (
        .;
        if ($entry.value | type) == "number" then
          .[$entry.key] = (.[$entry.key] // 0) + $entry.value
        else
          .
        end
      )
    )' "$@" >"$output_file"
}

update_step_summary() {
  local step_dir="$1"
  local step_number="$2"
  local attempt_count="$3"
  local final_status="$4"
  local commit_sha="$5"
  local legacy_sha="$6"
  local kind="$7"
  local text="$8"
  local codex_total_file="$step_dir/codex-usage-total.json"
  local claude_total_file="$step_dir/claude-usage-total.json"
  local step_summary_file="$step_dir/step-summary.json"
  local -a codex_usage_files=()
  local -a claude_usage_files=()

  shopt -s nullglob
  codex_usage_files=("$step_dir"/attempt-*/codex-usage.json)
  claude_usage_files=("$step_dir"/attempt-*/claude-usage.json)
  shopt -u nullglob

  aggregate_usage_files "$codex_total_file" "${codex_usage_files[@]}"
  aggregate_usage_files "$claude_total_file" "${claude_usage_files[@]}"

  jq -n \
    --argjson step "$step_number" \
    --arg legacy_sha "$legacy_sha" \
    --arg kind "$kind" \
    --arg text "$text" \
    --arg status "$final_status" \
    --arg commit_sha "$commit_sha" \
    --argjson attempts "$attempt_count" \
    --slurpfile codex_usage "$codex_total_file" \
    --slurpfile claude_usage "$claude_total_file" \
    '{
      step: $step,
      legacy_sha: (if $legacy_sha == "" then null else $legacy_sha end),
      kind: $kind,
      text: $text,
      status: $status,
      attempts: $attempts,
      commit_sha: (if $commit_sha == "" then null else $commit_sha end),
      codex_usage: ($codex_usage[0] // {}),
      claude_usage: ($claude_usage[0] // {})
    }' >"$step_summary_file"

  jq -c '.' "$step_summary_file" >>"$summary_jsonl_file"
}

write_final_summary() {
  jq -s '{steps: ., step_count: length}' "$run_dir"/step-*/step-summary.json >"$final_summary_file"
}

build_codex_prompt() {
  local prompt_file="$1"
  local step_number="$2"
  local attempt_number="$3"
  local kind="$4"
  local text="$5"
  local feedback_file="$6"
  local feedback_mode="without prior feedback"

  if [[ -s "$feedback_file" ]]; then
    feedback_mode="with prior feedback"
  fi

  cat >"$prompt_file" <<EOF
You are applying one migration step in the libexpat Rust port at $script_dir.

Requirements:
- Work only on Rust-side files: src/, lib.rs, build.rs, Cargo.toml, and Rust tooling files.
- Do not touch any C sources or headers under expat/.
- Preserve FFI/ABI compatibility with the C library:
  - keep extern "C" signatures and exported symbol names stable unless this step explicitly changes crate-local Rust functions,
  - preserve repr(C) layouts and field order for C-facing types,
  - avoid unwinding across FFI boundaries.
- Keep the change self-contained to this single migration step.
- Do not commit.
- Do not run ./test.sh r or ./bench.sh r; the wrapper script will run validation after your edit.

Current step:
- Step number: $step_number
- Attempt: $attempt_number
- Kind: $kind
- Instruction: $text
- Prompt mode: $feedback_mode

Implement the step now.
EOF

  if [[ -s "$feedback_file" ]]; then
    {
      printf '\nAddress all of the following feedback while keeping the current uncommitted changes that are already correct:\n\n'
      cat "$feedback_file"
    } >>"$prompt_file"
  fi
}

extract_codex_usage() {
  local jsonl_file="$1"
  local usage_file="$2"

  jq -sR '
    split("\n")
    | map(fromjson? | select(. != null))
    | map(select(.type == "turn.completed" and (.usage? != null)))
    | last
    | .usage // {}
  ' "$jsonl_file" >"$usage_file"
}

run_codex_attempt() {
  local attempt_dir="$1"
  local step_number="$2"
  local attempt_number="$3"
  local kind="$4"
  local text="$5"
  local feedback_file="$6"
  local prompt_file="$attempt_dir/codex-prompt.txt"
  local jsonl_file="$attempt_dir/codex.jsonl"
  local stderr_file="$attempt_dir/codex.stderr"
  local last_message_file="$attempt_dir/codex-last-message.txt"
  local usage_file="$attempt_dir/codex-usage.json"
  local -a codex_args=(
    exec
    --json
    --full-auto
    --cd "$script_dir"
    -o "$last_message_file"
  )

  build_codex_prompt "$prompt_file" "$step_number" "$attempt_number" "$kind" "$text" "$feedback_file"

  if [[ -n "$codex_model" ]]; then
    codex_args+=(--model "$codex_model")
  fi

  if ! codex "${codex_args[@]}" - <"$prompt_file" >"$jsonl_file" 2>"$stderr_file"; then
    {
      printf 'Codex exited non-zero for step %d attempt %d.\n' "$step_number" "$attempt_number"
      printf '\nRecent stderr:\n'
      tail_file_or_empty "$stderr_file" 120
      printf '\nRecent stdout:\n'
      tail_file_or_empty "$jsonl_file" 120
    } >"$feedback_file"
    return 1
  fi

  extract_codex_usage "$jsonl_file" "$usage_file"
  return 0
}

run_command_attempt() {
  local attempt_dir="$1"
  local step_number="$2"
  local attempt_number="$3"
  local command_text="$4"
  local feedback_file="$5"
  local command_log="$attempt_dir/command.log"

  if ! bash -lc "$command_text" >"$command_log" 2>&1; then
    {
      printf 'Command step failed for step %d attempt %d.\n' "$step_number" "$attempt_number"
      printf '\nCommand:\n%s\n' "$command_text"
      printf '\nRecent output:\n'
      tail_file_or_empty "$command_log" 120
    } >"$feedback_file"
    return 1
  fi

  jq -n '{}' >"$attempt_dir/codex-usage.json"
  return 0
}

run_validation_command() {
  local log_file="$1"
  shift

  "$@" >"$log_file" 2>&1
}

run_validation() {
  local attempt_dir="$1"
  local feedback_file="$2"
  local status_file="$attempt_dir/validation-status.json"
  local fmt_log="$attempt_dir/validate-fmt.log"
  local test_log="$attempt_dir/validate-test.log"
  local bench_log="$attempt_dir/validate-bench.log"
  local fmt_status=0
  local test_status=0
  local bench_status=0

  if run_validation_command "$fmt_log" cargo fmt --all --check; then
    fmt_status=0
  else
    fmt_status=$?
  fi
  if run_validation_command "$test_log" ./test.sh r; then
    test_status=0
  else
    test_status=$?
  fi
  if run_validation_command "$bench_log" ./bench.sh r; then
    bench_status=0
  else
    bench_status=$?
  fi

  jq -n \
    --argjson fmt_status "$fmt_status" \
    --argjson test_status "$test_status" \
    --argjson bench_status "$bench_status" \
    '{
      cargo_fmt: $fmt_status,
      test_sh_r: $test_status,
      bench_sh_r: $bench_status
    }' >"$status_file"

  if ((fmt_status == 0 && test_status == 0 && bench_status == 0)); then
    return 0
  fi

  {
    printf 'Wrapper validation failed.\n'
    printf '\nStatuses:\n'
    jq -r '
      to_entries[]
      | "- \(.key): " + (if .value == 0 then "ok" else ("failed (exit " + (.value | tostring) + ")") end)
    ' "$status_file"

    if ((fmt_status != 0)); then
      printf '\nRecent cargo fmt output:\n'
      tail_file_or_empty "$fmt_log" 120
    fi
    if ((test_status != 0)); then
      printf '\nRecent ./test.sh r output:\n'
      tail_file_or_empty "$test_log" 120
    fi
    if ((bench_status != 0)); then
      printf '\nRecent ./bench.sh r output:\n'
      tail_file_or_empty "$bench_log" 120
    fi
  } >"$feedback_file"

  return 1
}

build_review_prompt() {
  local prompt_file="$1"
  local step_number="$2"
  local attempt_number="$3"
  local kind="$4"
  local text="$5"

  cat >"$prompt_file" <<EOF
Review the current uncommitted changes in $script_dir for a single migration step.

Rules:
- Focus on correctness, ABI/FFI compatibility, behavioral regressions, and missing must-fix changes for this step.
- Do not make or suggest optional stylistic improvements.
- Do not edit files.
- Return approve only if the diff is ready to commit as-is.
- If you request changes, list only must-fix items in required_changes.

Step metadata:
- Step number: $step_number
- Attempt: $attempt_number
- Kind: $kind
- Instruction: $text
EOF
}

extract_claude_usage() {
  local session_id="$1"
  local usage_file="$2"
  local project_dir
  local session_file

  project_dir="$(sanitize_claude_project_dir)"
  session_file="$HOME/.claude/projects/$project_dir/$session_id.jsonl"

  [[ -f "$session_file" ]] || die "Claude session log not found at '$session_file'"

  jq -s '
    map(select(.message.usage? != null))
    | last
    | .message.usage // {}
  ' "$session_file" >"$usage_file"
}

run_claude_review() {
  local attempt_dir="$1"
  local step_number="$2"
  local attempt_number="$3"
  local kind="$4"
  local text="$5"
  local feedback_file="$6"
  local prompt_file="$attempt_dir/claude-prompt.txt"
  local review_output_file="$attempt_dir/claude-review.json"
  local stderr_file="$attempt_dir/claude.stderr"
  local usage_file="$attempt_dir/claude-usage.json"
  local session_id
  local prompt_text
  local review_schema_text
  local decision
  local -a claude_args

  build_review_prompt "$prompt_file" "$step_number" "$attempt_number" "$kind" "$text"
  prompt_text="$(<"$prompt_file")"
  review_schema_text="$(<"$review_schema_file")"
  session_id="$(uuidgen)"
  claude_args=(
    -p
    --output-format json
    --json-schema "$review_schema_text"
    --session-id "$session_id"
    --permission-mode dontAsk
    --tools "Read,Bash,Glob,Grep,StructuredOutput"
    --add-dir "$script_dir"
  )

  if [[ -n "$claude_model" ]]; then
    claude_args+=(--model "$claude_model")
  fi

  if ! claude "${claude_args[@]}" -- "$prompt_text" >"$review_output_file" 2>"$stderr_file"; then
    {
      printf 'Claude review failed for step %d attempt %d.\n' "$step_number" "$attempt_number"
      printf '\nRecent stderr:\n'
      tail_file_or_empty "$stderr_file" 120
    } >"$feedback_file"
    return 1
  fi

  extract_claude_usage "$session_id" "$usage_file"

  decision="$(jq -r '
    if has("decision") then
      .decision
    elif (.result | type) == "object" and .result.decision? then
      .result.decision
    elif has("structured_output") and .structured_output.decision? then
      .structured_output.decision
    else
      empty
    end
  ' "$review_output_file")"
  [[ -n "$decision" ]] || die "Claude review output did not contain a decision"

  if [[ "$decision" == "approve" ]]; then
    : >"$feedback_file"
    return 0
  fi

  if [[ "$decision" != "request_changes" ]]; then
    die "unexpected Claude decision '$decision'"
  fi

  jq -r '
    (
      if has("decision") then
        .
      elif (.result | type) == "object" and .result.decision? then
        .result
      elif has("structured_output") and .structured_output.decision? then
        .structured_output
      else
        {}
      end
    ) as $review
    |
    [
      "Claude requested changes.",
      "",
      "Summary:",
      $review.summary,
      "",
      "Required changes:"
    ] + ($review.required_changes | map("- " + .))
    | .[]
  ' "$review_output_file" >"$feedback_file"

  return 1
}

create_attempt_metadata() {
  local attempt_dir="$1"
  local step_number="$2"
  local attempt_number="$3"
  local legacy_sha="$4"
  local kind="$5"
  local text="$6"
  local attempt_status="$7"

  jq -n \
    --argjson step "$step_number" \
    --argjson attempt "$attempt_number" \
    --arg legacy_sha "$legacy_sha" \
    --arg kind "$kind" \
    --arg text "$text" \
    --arg status "$attempt_status" \
    --slurpfile codex_usage "$attempt_dir/codex-usage.json" \
    --slurpfile claude_usage "$attempt_dir/claude-usage.json" \
    '{
      step: $step,
      attempt: $attempt,
      legacy_sha: (if $legacy_sha == "" then null else $legacy_sha end),
      kind: $kind,
      text: $text,
      status: $status,
      codex_usage: ($codex_usage[0] // {}),
      claude_usage: ($claude_usage[0] // {})
    }' >"$attempt_dir/attempt.json"
}

commit_step() {
  local kind="$1"
  local text="$2"
  local agent_name
  local subject_prefix

  if [[ "$kind" == "command" ]]; then
    subject_prefix="Command"
    agent_name="script"
  else
    subject_prefix="Prompt"
    agent_name="codex"
  fi

  git add -A
  git commit -m "$subject_prefix: $text" -m "Agent: $agent_name"$'\n'"Reviewer: claude" >/dev/null
}

run_step() {
  local step_number="$1"
  local legacy_sha="$2"
  local kind="$3"
  local text="$4"
  local step_dir
  local feedback_file
  local attempt_number=0
  local attempt_dir
  local commit_sha=""
  local attempt_status=""

  step_dir="$run_dir/step-$(printf '%02d' "$step_number")"
  feedback_file="$step_dir/pending-feedback.txt"
  mkdir -p "$step_dir"
  : >"$feedback_file"

  while ((attempt_number < max_attempts)); do
    attempt_number=$((attempt_number + 1))
    attempt_dir="$step_dir/attempt-$(printf '%02d' "$attempt_number")"
    mkdir -p "$attempt_dir"
    jq -n '{}' >"$attempt_dir/codex-usage.json"
    jq -n '{}' >"$attempt_dir/claude-usage.json"

    log "step $step_number attempt $attempt_number: $text"

    if [[ "$kind" == "prompt" ]]; then
      if ! run_codex_attempt "$attempt_dir" "$step_number" "$attempt_number" "$kind" "$text" "$feedback_file"; then
        create_attempt_metadata "$attempt_dir" "$step_number" "$attempt_number" "$legacy_sha" "$kind" "$text" "codex_failed"
        continue
      fi
    else
      if ! run_command_attempt "$attempt_dir" "$step_number" "$attempt_number" "$text" "$feedback_file"; then
        create_attempt_metadata "$attempt_dir" "$step_number" "$attempt_number" "$legacy_sha" "$kind" "$text" "command_failed"
        continue
      fi
    fi

    if ! has_repo_changes; then
      printf 'The step produced no repository changes.\n' >"$feedback_file"
      create_attempt_metadata "$attempt_dir" "$step_number" "$attempt_number" "$legacy_sha" "$kind" "$text" "no_changes"
      continue
    fi

    if ! run_validation "$attempt_dir" "$feedback_file"; then
      create_attempt_metadata "$attempt_dir" "$step_number" "$attempt_number" "$legacy_sha" "$kind" "$text" "validation_failed"
      continue
    fi

    if ! run_claude_review "$attempt_dir" "$step_number" "$attempt_number" "$kind" "$text" "$feedback_file"; then
      create_attempt_metadata "$attempt_dir" "$step_number" "$attempt_number" "$legacy_sha" "$kind" "$text" "review_requested_changes"
      continue
    fi

    commit_step "$kind" "$text"
    commit_sha="$(git rev-parse HEAD)"
    attempt_status="committed"
    create_attempt_metadata "$attempt_dir" "$step_number" "$attempt_number" "$legacy_sha" "$kind" "$text" "$attempt_status"
    update_step_summary "$step_dir" "$step_number" "$attempt_number" "committed" "$commit_sha" "$legacy_sha" "$kind" "$text"
    log "step $step_number committed as $commit_sha"
    return 0
  done

  update_step_summary "$step_dir" "$step_number" "$attempt_number" "failed" "" "$legacy_sha" "$kind" "$text"
  die "step $step_number exceeded the max attempt count ($max_attempts)"
}

list_only=0
from_step=1
to_step=0
run_dir=""
max_attempts=8
codex_model="${CODEX_MODEL:-}"
claude_model="${CLAUDE_MODEL:-}"
review_schema_file=""
summary_jsonl_file=""
final_summary_file=""

load_steps
total_steps="${#step_texts[@]}"

while (($# > 0)); do
  case "$1" in
    --list-steps)
      list_only=1
      ;;
    --from-step)
      shift
      [[ $# -gt 0 ]] || die "--from-step requires a value"
      from_step="$1"
      ;;
    --to-step)
      shift
      [[ $# -gt 0 ]] || die "--to-step requires a value"
      to_step="$1"
      ;;
    --run-dir)
      shift
      [[ $# -gt 0 ]] || die "--run-dir requires a value"
      run_dir="$1"
      ;;
    --max-attempts)
      shift
      [[ $# -gt 0 ]] || die "--max-attempts requires a value"
      max_attempts="$1"
      ;;
    --codex-model)
      shift
      [[ $# -gt 0 ]] || die "--codex-model requires a value"
      codex_model="$1"
      ;;
    --claude-model)
      shift
      [[ $# -gt 0 ]] || die "--claude-model requires a value"
      claude_model="$1"
      ;;
    -h|--help)
      usage
      exit 0
      ;;
    *)
      die "unknown argument '$1'"
      ;;
  esac
  shift
done

assert_integer "from_step" "$from_step"
assert_integer "to_step" "$to_step"
assert_integer "max_attempts" "$max_attempts"

((from_step >= 1)) || die "from_step must be at least 1"
if ((to_step == 0)); then
  to_step="$total_steps"
fi
((to_step >= from_step)) || die "to_step must be greater than or equal to from_step"
((to_step <= total_steps)) || die "to_step must be less than or equal to $total_steps"
((max_attempts >= 1)) || die "max_attempts must be at least 1"

if ((list_only == 1)); then
  list_steps
  exit 0
fi

require_command git
require_command jq
require_command codex
require_command claude
require_command cargo
require_command uuidgen

ensure_clean_worktree
setup_run_dir

log "artifacts will be written to $run_dir"

for ((step_index = from_step; step_index <= to_step; step_index += 1)); do
  run_step \
    "$step_index" \
    "${step_legacy_shas[step_index - 1]}" \
    "${step_kinds[step_index - 1]}" \
    "${step_texts[step_index - 1]}"
done

write_final_summary
log "migration run finished successfully"
log "summary: $final_summary_file"
