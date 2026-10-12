# The operator's answers at the P4 dialog — 2026-10-10

Given in this session, in answer to the phase's one dialog of four questions. Each answer is the option chosen,
then the note typed with it, verbatim.

1. Question: "The coverage step passes an empty report, and its branch arm reads 0/0 = 100% because the pinned
   toolchain cannot write a branch count. Which way does the coverage job fall?"
   Answer: "Zero fails, branch retired (Recommended)"
   Note: "Operator (pc overseer): a threshold that has never read a number is a false statement, and a second
   unpinned toolchain is not its repair. Recorded PROVISIONAL for the founder; say in the plan what a real branch
   count would need."

2. Question: "Five nextest invocations pass a run that selects no test (exit 0 with the flag, exit 4 without).
   Which way do they fall?"
   Answer: "All five fail on empty (Recommended)"
   Note: none.

3. Question: "In the boot job, ci-gates' heartbeat and perf-budget arms cannot fail on a boot of about six
   seconds; its record-count and panic arms read 112 records and can. Which way does the step fall?"
   Answer: "Narrow the verb (Recommended)"
   Note: "The pre-push check runs this verb too: name in the plan every place that reads the two lines that leave
   (the pre-push stage, any pin, any master sentence) and what each reads after."

4. Question: "Three smaller readings remain: the a11y upload's path members, the quarantine check's pass over an
   absent input, and the pre-push stage that reads its own seed record. Which set is repaired here?"
   Answer: "Quarantine only (Recommended)"
   Note: "The two that stay each keep an owner and a stated reading; nothing is repaired for a job on its way out.
   P-128 is about what a CI job does: say in the concretization that the pre-push stage is outside it and who owns
   it."

The options as offered, with the description each carried, for the record of what was not chosen:
- Question 1: "Zero fails, branch on nightly" (the coverage job on a nightly toolchain with `--branch`; a second,
  unpinned toolchain; the branch percentage unmeasured) · "Zero fails, branch to founder" (the zero arm repaired,
  the branch arm left for the founder's ruling; P-128 then not claimable here).
- Question 2: "Four in CI, load-profiles kept" · "Flag kept, count asserted".
- Question 3: "Step leaves whole" (the job loses the only CI read of a panic record over a booted app) · "Repair
  in place" (two ticks per target and required budget arms; needs a longer smoke).
- Question 4: "Quarantine and upload" (also a plain run step asserting the three members) · "All three" (also
  remove the pre-push ci-gates stage, six stages becoming five) · "None here" (all three recorded as measured;
  the claim waits).

The recommended option of question 1 as offered: "A report tracking 0 lines or 0 functions fails the step. The
branch arm and the job-name clause leave: the 70% gate has never read a number. Cost: about 10 workflow lines, one
executed pin, six test-plan sites at wrap. No required check keys on the job name. Retiring a stated threshold
stays PROVISIONAL until the founder's word."

The recommended option of question 2 as offered: "Each --no-tests=pass becomes --no-tests=fail, spelled out because
the runner's nextest is older than the host's and its default was not read. The step name and two help strings
follow. Cost: seven literals and one pin. A filtered cargo xtask test that matches nothing then exits 4 on the dev
host too."

The recommended option of question 3 as offered: "ci-gates keeps the two arms that read the boot log and exits 2
on an absent log instead of four NEUTRAL lines; the heartbeat and perf-budget arms leave it. The job loses nothing
today, and lint-test's perf:budget stays the budget gate. The zero-span sentence is amended to its measured home.
Cost: about 80 lines in xtask with a pin per arm; obs-plan and arch sites at wrap."

The recommended option of question 4 as offered: "The quarantine check fails on a missing search dir and prints
what it scanned instead of NEUTRAL: about 10 script lines, a mirrored .ps1 not run on this host, one executed pin.
The a11y upload stays with its reading recorded and its carry moved to Window's gates retired. The pre-push stage
is recorded and carried to the harness entry."
